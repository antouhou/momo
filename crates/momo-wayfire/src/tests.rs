use crate::{WayfireBackend, WayfireIpcConfiguration};
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use momo_compositor::{
    CompositorBackend, CompositorEvent, CompositorStartupConfiguration, Key, ShortcutId,
    ShortcutRegistration, ShortcutTrigger,
};
use serde_json::{Value, json};
use std::{sync::mpsc, thread, time::Duration};
use tokio::net::{UnixListener, UnixStream};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

const HEADER_LENGTH: usize = 4;
const MAXIMUM_MESSAGE_LENGTH: usize = 1 << 20;
const TEST_TIMEOUT: Duration = Duration::from_secs(1);
const TEST_SHORTCUT_ID: ShortcutId = ShortcutId::new(7);

type FramedUnixStream = Framed<UnixStream, LengthDelimitedCodec>;

#[test]
fn registers_configured_shortcut_and_forwards_queued_and_live_binding_events() {
    let temporary_directory = tempfile::tempdir().expect("temporary directory should be created");
    let socket_path = temporary_directory.path().join("wayfire.socket");
    let server_socket_path = socket_path.clone();
    let (socket_ready_sender, socket_ready_receiver) = mpsc::channel();
    let (send_live_event, wait_to_send_live_event) = tokio::sync::oneshot::channel();
    let server_thread = thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .expect("fake Wayfire runtime should be created");
        runtime.block_on(async move {
            let listener =
                UnixListener::bind(&server_socket_path).expect("fake Wayfire socket should bind");
            socket_ready_sender
                .send(())
                .expect("fake Wayfire server should report readiness");
            let (stream, _) = listener.accept().await.expect("backend should connect");
            let mut framed_stream = framed_stream(stream);
            let request = read_json_frame(&mut framed_stream).await;
            assert_eq!(request["method"], "command/register-binding");
            assert_eq!(request["data"]["binding"], "<ctrl> <alt> KEY_SPACE KEY_L");

            write_json_frame(
                &mut framed_stream,
                &json!({"event": "command-binding", "binding-id": 42}),
            )
            .await;
            write_json_frame(
                &mut framed_stream,
                &json!({"result": "ok", "binding-id": 42}),
            )
            .await;
            wait_to_send_live_event
                .await
                .expect("test should release the live event");
            write_json_frame(
                &mut framed_stream,
                &json!({"event": "command-binding", "binding-id": 42}),
            )
            .await;

            let next_frame = tokio::time::timeout(TEST_TIMEOUT, framed_stream.next())
                .await
                .expect("backend should close the socket during shutdown");
            assert!(next_frame.is_none(), "shutdown should close the IPC stream");
        });
    });
    socket_ready_receiver
        .recv_timeout(TEST_TIMEOUT)
        .expect("fake Wayfire server should become ready");

    let backend = WayfireBackend::new(WayfireIpcConfiguration {
        socket_path: Some(socket_path),
    });
    let mut session = backend
        .start(CompositorStartupConfiguration {
            shortcuts: vec![ShortcutRegistration {
                id: TEST_SHORTCUT_ID,
                trigger: ShortcutTrigger {
                    keys: vec![Key::Control, Key::Alt, Key::Space, Key::L],
                },
            }],
        })
        .expect("backend should start");
    let event_receiver = session
        .take_event_receiver()
        .expect("session should expose compositor events");
    let events = vec![
        event_receiver
            .recv_timeout(TEST_TIMEOUT)
            .expect("connected event should be forwarded"),
        event_receiver
            .recv_timeout(TEST_TIMEOUT)
            .expect("queued binding event should be forwarded"),
    ];
    assert_eq!(
        events,
        vec![
            CompositorEvent::Connected,
            CompositorEvent::ShortcutActivated(TEST_SHORTCUT_ID),
        ]
    );
    send_live_event
        .send(())
        .expect("fake server should accept the live event signal");
    assert_eq!(
        event_receiver
            .recv_timeout(TEST_TIMEOUT)
            .expect("live binding event should be forwarded"),
        CompositorEvent::ShortcutActivated(TEST_SHORTCUT_ID)
    );
    session.stop();
    server_thread.join().expect("fake server should finish");
}

#[test]
fn startup_does_not_wait_for_the_binding_registration_response() {
    let temporary_directory = tempfile::tempdir().expect("temporary directory should be created");
    let socket_path = temporary_directory.path().join("wayfire.socket");
    let server_socket_path = socket_path.clone();
    let (socket_ready_sender, socket_ready_receiver) = mpsc::channel();
    let (request_received_sender, request_received_receiver) = mpsc::channel();
    let (release_response_sender, release_response_receiver) = tokio::sync::oneshot::channel();
    let server_thread = thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .expect("fake Wayfire runtime should be created");
        runtime.block_on(async move {
            let listener =
                UnixListener::bind(&server_socket_path).expect("fake Wayfire socket should bind");
            socket_ready_sender
                .send(())
                .expect("fake Wayfire server should report readiness");
            let (stream, _) = listener.accept().await.expect("backend should connect");
            let mut framed_stream = framed_stream(stream);
            read_json_frame(&mut framed_stream).await;
            request_received_sender
                .send(())
                .expect("fake Wayfire server should report the request");
            release_response_receiver
                .await
                .expect("test should release the registration response");
            write_json_frame(
                &mut framed_stream,
                &json!({"result": "ok", "binding-id": 42}),
            )
            .await;

            let next_frame = tokio::time::timeout(TEST_TIMEOUT, framed_stream.next())
                .await
                .expect("backend should close the socket during shutdown");
            assert!(next_frame.is_none(), "shutdown should close the IPC stream");
        });
    });
    socket_ready_receiver
        .recv_timeout(TEST_TIMEOUT)
        .expect("fake Wayfire server should become ready");

    let backend = WayfireBackend::new(WayfireIpcConfiguration {
        socket_path: Some(socket_path),
    });
    let (session_sender, session_receiver) = mpsc::channel();
    let startup_thread = thread::spawn(move || {
        session_sender
            .send(backend.start(CompositorStartupConfiguration {
                shortcuts: vec![ShortcutRegistration {
                    id: TEST_SHORTCUT_ID,
                    trigger: ShortcutTrigger::super_key(),
                }],
            }))
            .expect("test should receive the startup result");
    });
    let mut release_response_sender = Some(release_response_sender);
    let startup_result = match session_receiver.recv_timeout(TEST_TIMEOUT) {
        Ok(startup_result) => startup_result,
        Err(error) => {
            release_response_sender
                .take()
                .expect("registration response should not be released yet")
                .send(())
                .expect("fake Wayfire server should still be waiting");
            let mut session = session_receiver
                .recv_timeout(TEST_TIMEOUT)
                .expect("blocking startup should finish during cleanup")
                .expect("backend should start during cleanup");
            session.stop();
            startup_thread.join().expect("startup thread should finish");
            server_thread.join().expect("fake server should finish");
            panic!("startup waited for the registration response: {error}");
        }
    };
    let mut session = startup_result.expect("backend should start");
    request_received_receiver
        .recv_timeout(TEST_TIMEOUT)
        .expect("fake Wayfire server should receive the request");
    release_response_sender
        .take()
        .expect("registration response should not be released yet")
        .send(())
        .expect("fake Wayfire server should still be waiting");
    let event_receiver = session
        .take_event_receiver()
        .expect("session should expose compositor events");
    assert_eq!(
        event_receiver
            .recv_timeout(TEST_TIMEOUT)
            .expect("connected event should follow registration"),
        CompositorEvent::Connected
    );
    session.stop();
    startup_thread.join().expect("startup thread should finish");
    server_thread.join().expect("fake server should finish");
}

fn framed_stream(stream: UnixStream) -> FramedUnixStream {
    LengthDelimitedCodec::builder()
        .little_endian()
        .length_field_length(HEADER_LENGTH)
        .max_frame_length(MAXIMUM_MESSAGE_LENGTH)
        .new_framed(stream)
}

async fn read_json_frame(stream: &mut FramedUnixStream) -> Value {
    let frame = stream
        .next()
        .await
        .expect("request frame should be present")
        .expect("request frame should be readable");
    serde_json::from_slice(&frame).expect("request should be valid JSON")
}

async fn write_json_frame(stream: &mut FramedUnixStream, value: &Value) {
    let payload = serde_json::to_vec(value).expect("response should encode");
    stream
        .send(Bytes::from(payload))
        .await
        .expect("response frame should write");
}
