use std::io::Cursor;
use super::{
    AUTH_FAILED_USER_MESSAGE, AuthMessageType, ErrorType, GreetdError, GreetdRequest,
    GreetdResponse, read_message, write_message,
};

#[test]
fn writes_length_prefixed_request_payload() {
    let cmd = vec!["wayfire".to_string()];
    let env = Vec::new();
    let mut buffer = Vec::new();
    write_message(
        &mut buffer,
        &GreetdRequest::StartSession {
            cmd: cmd.as_slice(),
            env: env.as_slice(),
        },
    )
    .expect("request should encode");

    let mut length_bytes = [0_u8; 4];
    length_bytes.copy_from_slice(&buffer[..4]);
    let payload_length = u32::from_ne_bytes(length_bytes) as usize;
    assert_eq!(payload_length, buffer.len() - 4);

    let payload: serde_json::Value =
        serde_json::from_slice(&buffer[4..]).expect("payload should be JSON");
    assert_eq!(payload["type"], "start_session");
    assert_eq!(payload["cmd"][0], "wayfire");
}

#[test]
fn reads_auth_message_protocol_aliases() {
    let payload = br#"{"type":"auth_message","message_type":"secret","message":"Password:"}"#;
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&(payload.len() as u32).to_ne_bytes());
    buffer.extend_from_slice(payload);

    let response: GreetdResponse =
        read_message(&mut Cursor::new(buffer)).expect("response should decode");
    assert_eq!(
        response,
        GreetdResponse::AuthMessage {
            auth_message_type: AuthMessageType::Secret,
            auth_message: "Password:".to_string(),
        }
    );
}

#[test]
fn auth_errors_use_actionable_user_message() {
    let error = GreetdError::Response {
        error_type: ErrorType::AuthError,
        description: "pam authentication failed".to_string(),
    };

    assert_eq!(error.user_message(), AUTH_FAILED_USER_MESSAGE);
}
