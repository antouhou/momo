use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    thread,
};

use momo_compositor::{
    CompositorAction, CompositorBackend, CompositorCommand, CompositorEvent,
    ConnectionConfiguration,
};
use serde_json::{Value, json};

use crate::WayfireBackend;

#[test]
fn registers_super_and_forwards_its_binding_event() {
    let temporary_directory = tempfile::tempdir().expect("temporary directory should be created");
    let socket_path = temporary_directory.path().join("wayfire.socket");
    let listener = UnixListener::bind(&socket_path).expect("fake Wayfire socket should bind");
    let server_thread = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("backend should connect");
        let request = read_json_frame(&mut stream);
        assert_eq!(request["method"], "command/register-binding");
        assert_eq!(request["data"]["binding"], "<super>");

        write_json_frame(
            &mut stream,
            &json!({"event": "command-binding", "binding-id": 42}),
        );
        write_json_frame(&mut stream, &json!({"result": "ok", "binding-id": 42}));
    });

    let mut backend = WayfireBackend::disconnected();
    backend
        .connect(&ConnectionConfiguration {
            socket_path: Some(socket_path),
        })
        .expect("backend should connect");
    backend
        .dispatch(CompositorCommand::RegisterAction(
            CompositorAction::ToggleLauncher,
        ))
        .expect("binding should register");

    let events = backend.poll_events().expect("events should be readable");
    assert_eq!(
        events,
        vec![
            CompositorEvent::Connected,
            CompositorEvent::ActionActivated(CompositorAction::ToggleLauncher),
        ]
    );
    server_thread.join().expect("fake server should finish");
}

fn read_json_frame(stream: &mut UnixStream) -> Value {
    let mut header = [0_u8; 4];
    stream
        .read_exact(&mut header)
        .expect("request header should be readable");
    let payload_length = u32::from_le_bytes(header) as usize;
    let mut payload = vec![0_u8; payload_length];
    stream
        .read_exact(&mut payload)
        .expect("request payload should be readable");
    serde_json::from_slice(&payload).expect("request should be valid JSON")
}

fn write_json_frame(stream: &mut UnixStream, value: &Value) {
    let payload = serde_json::to_vec(value).expect("response should encode");
    let payload_length = u32::try_from(payload.len()).expect("test response should fit in a frame");
    stream
        .write_all(&payload_length.to_le_bytes())
        .expect("response header should write");
    stream
        .write_all(&payload)
        .expect("response payload should write");
}
