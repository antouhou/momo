use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

const GREETD_SOCK_ENV: &str = "GREETD_SOCK";
const DEFAULT_GREETD_SOCKET: &str = "/run/greetd.sock";
const AUTH_FAILED_USER_MESSAGE: &str = "Incorrect password. Try again.";

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GreetdAuthRequest {
    pub username: String,
    pub secret: String,
    pub session_command: Vec<String>,
    pub env: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct GreetdAuthenticator {
    socket_path: Arc<PathBuf>,
}

impl GreetdAuthenticator {
    pub fn new(socket_path: impl Into<PathBuf>) -> Self {
        Self {
            socket_path: Arc::new(socket_path.into()),
        }
    }

    pub fn from_environment() -> Self {
        let socket_path =
            std::env::var(GREETD_SOCK_ENV).unwrap_or_else(|_| DEFAULT_GREETD_SOCKET.to_string());
        Self::new(socket_path)
    }

    pub fn authenticate(&self, request: GreetdAuthRequest) -> Result<(), GreetdError> {
        let mut stream = UnixStream::connect(self.socket_path.as_path())?;
        let mut conversation = GreetdConversation::new(&mut stream);
        conversation.authenticate(&request)
    }
}

struct GreetdConversation<'a> {
    stream: &'a mut UnixStream,
}

impl<'a> GreetdConversation<'a> {
    fn new(stream: &'a mut UnixStream) -> Self {
        Self { stream }
    }

    fn authenticate(&mut self, request: &GreetdAuthRequest) -> Result<(), GreetdError> {
        self.send(&GreetdRequest::CreateSession {
            username: request.username.as_str(),
        })?;

        let mut secret = Some(request.secret.as_str());
        loop {
            match self.receive()? {
                GreetdResponse::Success => break,
                GreetdResponse::Error {
                    error_type,
                    description,
                } => {
                    return Err(GreetdError::Response {
                        error_type,
                        description,
                    });
                }
                GreetdResponse::AuthMessage {
                    auth_message_type,
                    auth_message,
                } => {
                    let response = match auth_message_type {
                        AuthMessageType::Secret | AuthMessageType::Visible => match secret.take() {
                            Some(secret) => Some(secret),
                            None => {
                                self.cancel();
                                return Err(GreetdError::UnsupportedPrompt {
                                    message: auth_message,
                                });
                            }
                        },
                        AuthMessageType::Info | AuthMessageType::Error => {
                            tracing::info!(
                                message = %auth_message,
                                message_type = ?auth_message_type,
                                "greetd auth message"
                            );
                            None
                        }
                    };

                    self.send(&GreetdRequest::PostAuthMessageResponse { response })?;
                }
            }
        }

        self.send(&GreetdRequest::StartSession {
            cmd: request.session_command.as_slice(),
            env: request.env.as_slice(),
        })?;

        match self.receive()? {
            GreetdResponse::Success => Ok(()),
            GreetdResponse::Error {
                error_type,
                description,
            } => Err(GreetdError::Response {
                error_type,
                description,
            }),
            GreetdResponse::AuthMessage { auth_message, .. } => {
                Err(GreetdError::UnexpectedAuthMessage {
                    message: auth_message,
                })
            }
        }
    }

    fn send(&mut self, request: &GreetdRequest<'_>) -> Result<(), GreetdError> {
        write_message(self.stream, request)
    }

    fn receive(&mut self) -> Result<GreetdResponse, GreetdError> {
        read_message(self.stream)
    }

    fn cancel(&mut self) {
        if let Err(error) = self.send(&GreetdRequest::CancelSession) {
            tracing::warn!(%error, "failed to cancel greetd session");
        }
    }
}

#[derive(Debug, Error)]
pub enum GreetdError {
    #[error("failed to connect to greetd: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to encode greetd message: {0}")]
    Encode(#[source] serde_json::Error),
    #[error("failed to decode greetd message: {0}")]
    Decode(#[source] serde_json::Error),
    #[error("greetd payload is too large")]
    PayloadTooLarge,
    #[error("greetd returned {error_type:?}: {description}")]
    Response {
        error_type: ErrorType,
        description: String,
    },
    #[error("greetd requested an additional prompt: {message}")]
    UnsupportedPrompt { message: String },
    #[error("greetd requested authentication after session start: {message}")]
    UnexpectedAuthMessage { message: String },
}

impl GreetdError {
    pub fn user_message(&self) -> String {
        match self {
            Self::Response {
                error_type: ErrorType::AuthError,
                ..
            } => AUTH_FAILED_USER_MESSAGE.to_string(),
            Self::UnsupportedPrompt { .. } => {
                "Additional authentication prompts are not supported yet".to_string()
            }
            Self::Io(_) => "Unable to contact greetd".to_string(),
            _ => self.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum GreetdRequest<'a> {
    CreateSession {
        username: &'a str,
    },
    PostAuthMessageResponse {
        response: Option<&'a str>,
    },
    StartSession {
        cmd: &'a [String],
        env: &'a [String],
    },
    CancelSession,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
enum GreetdResponse {
    Success,
    Error {
        error_type: ErrorType,
        description: String,
    },
    AuthMessage {
        #[serde(alias = "message_type")]
        auth_message_type: AuthMessageType,
        #[serde(alias = "message")]
        auth_message: String,
    },
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    Error,
    AuthError,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum AuthMessageType {
    Visible,
    Secret,
    Info,
    Error,
}

fn write_message<T: Serialize>(writer: &mut impl Write, message: &T) -> Result<(), GreetdError> {
    let payload = serde_json::to_vec(message).map_err(GreetdError::Encode)?;
    let length = u32::try_from(payload.len()).map_err(|_| GreetdError::PayloadTooLarge)?;
    writer.write_all(&length.to_ne_bytes())?;
    writer.write_all(&payload)?;
    writer.flush()?;
    Ok(())
}

fn read_message<T: for<'de> Deserialize<'de>>(reader: &mut impl Read) -> Result<T, GreetdError> {
    let mut length_bytes = [0_u8; std::mem::size_of::<u32>()];
    reader.read_exact(&mut length_bytes)?;
    let length = u32::from_ne_bytes(length_bytes) as usize;

    let mut payload = vec![0_u8; length];
    reader.read_exact(&mut payload)?;
    serde_json::from_slice(&payload).map_err(GreetdError::Decode)
}
