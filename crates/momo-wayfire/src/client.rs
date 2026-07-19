use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use serde_json::Value;
use std::{collections::VecDeque, io, path::Path, time::Duration};
use thiserror::Error;
use tokio::{net::UnixStream, time::Instant};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

const HEADER_LENGTH: usize = 4;
const ONE_MEGABYTE: usize = 1024 * 1024;
const MAXIMUM_MESSAGE_LENGTH: usize = ONE_MEGABYTE;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(3);

pub(super) struct WayfireClient {
    framed_stream: Framed<UnixStream, LengthDelimitedCodec>,
    queued_events: VecDeque<Value>,
}

impl WayfireClient {
    pub(super) async fn connect(socket_path: &Path) -> Result<Self, WayfireClientError> {
        let stream = UnixStream::connect(socket_path)
            .await
            .map_err(WayfireClientError::Connect)?;
        let framed_stream = LengthDelimitedCodec::builder()
            .little_endian()
            .length_field_length(HEADER_LENGTH)
            .max_frame_length(MAXIMUM_MESSAGE_LENGTH)
            .new_framed(stream);
        Ok(Self {
            framed_stream,
            queued_events: VecDeque::new(),
        })
    }

    pub(super) async fn request(
        &mut self,
        request: &impl Serialize,
    ) -> Result<Value, WayfireClientError> {
        let payload = serde_json::to_vec(request).map_err(WayfireClientError::Encode)?;
        if payload.len() > MAXIMUM_MESSAGE_LENGTH {
            return Err(WayfireClientError::MessageTooLarge(payload.len()));
        }

        let deadline = Instant::now() + REQUEST_TIMEOUT;
        tokio::time::timeout_at(deadline, self.framed_stream.send(Bytes::from(payload)))
            .await
            .map_err(|_| WayfireClientError::WriteTimeout)?
            .map_err(WayfireClientError::Write)?;

        loop {
            let message = tokio::time::timeout_at(deadline, self.receive_message())
                .await
                .map_err(|_| WayfireClientError::ResponseTimeout)??;
            if message.get("event").is_some() {
                self.queued_events.push_back(message);
            } else if let Some(error) = message.get("error").and_then(Value::as_str) {
                return Err(WayfireClientError::Method(error.to_string()));
            } else {
                return Ok(message);
            }
        }
    }

    pub(super) async fn receive_message(&mut self) -> Result<Value, WayfireClientError> {
        match self.framed_stream.next().await {
            Some(Ok(frame)) => serde_json::from_slice(&frame).map_err(WayfireClientError::Decode),
            Some(Err(error)) => Err(WayfireClientError::Read(error)),
            None => Err(WayfireClientError::Disconnected),
        }
    }

    pub(super) fn take_queued_event(&mut self) -> Option<Value> {
        self.queued_events.pop_front()
    }
}

#[derive(Debug, Error)]
pub(super) enum WayfireClientError {
    #[error("failed to connect to the Wayfire IPC socket: {0}")]
    Connect(#[source] io::Error),
    #[error("failed to encode a Wayfire IPC request: {0}")]
    Encode(#[source] serde_json::Error),
    #[error("failed to decode a Wayfire IPC message: {0}")]
    Decode(#[source] serde_json::Error),
    #[error("failed to write to the Wayfire IPC socket: {0}")]
    Write(#[source] io::Error),
    #[error("failed to read from the Wayfire IPC socket: {0}")]
    Read(#[source] io::Error),
    #[error("Wayfire rejected the IPC request: {0}")]
    Method(String),
    #[error("Wayfire IPC message is too large: {0} bytes")]
    MessageTooLarge(usize),
    #[error("timed out writing a Wayfire IPC request")]
    WriteTimeout,
    #[error("timed out waiting for a Wayfire IPC response")]
    ResponseTimeout,
    #[error("the Wayfire IPC socket disconnected")]
    Disconnected,
}
