use serde::Serialize;
use serde_json::Value;
use std::{
    collections::VecDeque,
    io::{self, Read, Write},
    os::unix::net::UnixStream,
    path::Path,
    thread,
    time::{Duration, Instant},
};
use thiserror::Error;

const HEADER_LENGTH: usize = 4;
const MAXIMUM_MESSAGE_LENGTH: usize = 1 << 20;
const IO_RETRY_INTERVAL: Duration = Duration::from_millis(1);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(3);

pub(super) struct WayfireClient {
    stream: UnixStream,
    read_buffer: Vec<u8>,
    queued_events: VecDeque<Value>,
}

impl WayfireClient {
    pub(super) fn connect(socket_path: &Path) -> Result<Self, WayfireClientError> {
        let stream = UnixStream::connect(socket_path).map_err(WayfireClientError::Connect)?;
        stream
            .set_nonblocking(true)
            .map_err(WayfireClientError::ConfigureSocket)?;
        Ok(Self {
            stream,
            read_buffer: Vec::new(),
            queued_events: VecDeque::new(),
        })
    }

    pub(super) fn request(
        &mut self,
        request: &impl Serialize,
    ) -> Result<Value, WayfireClientError> {
        let payload = serde_json::to_vec(request).map_err(WayfireClientError::Encode)?;
        self.write_frame(&payload, REQUEST_TIMEOUT)?;

        let deadline = Instant::now() + REQUEST_TIMEOUT;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(WayfireClientError::ResponseTimeout);
            }
            let message = self
                .read_message(remaining)?
                .ok_or(WayfireClientError::ResponseTimeout)?;
            if message.get("event").is_some() {
                self.queued_events.push_back(message);
            } else if let Some(error) = message.get("error").and_then(Value::as_str) {
                return Err(WayfireClientError::Method(error.to_string()));
            } else {
                return Ok(message);
            }
        }
    }

    pub(super) fn poll_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Value>, WayfireClientError> {
        if let Some(event) = self.queued_events.pop_front() {
            return Ok(Some(event));
        }
        self.read_message(timeout)
    }

    fn write_frame(&mut self, payload: &[u8], timeout: Duration) -> Result<(), WayfireClientError> {
        let payload_length = u32::try_from(payload.len())
            .map_err(|_| WayfireClientError::MessageTooLarge(payload.len()))?;
        let mut frame = Vec::with_capacity(HEADER_LENGTH + payload.len());
        frame.extend_from_slice(&payload_length.to_le_bytes());
        frame.extend_from_slice(payload);

        let deadline = Instant::now() + timeout;
        let mut written = 0;
        while written < frame.len() {
            match self.stream.write(&frame[written..]) {
                Ok(0) => return Err(WayfireClientError::Disconnected),
                Ok(byte_count) => written += byte_count,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    if Instant::now() >= deadline {
                        return Err(WayfireClientError::WriteTimeout);
                    }
                    thread::sleep(IO_RETRY_INTERVAL);
                }
                Err(error) => return Err(WayfireClientError::Write(error)),
            }
        }
        Ok(())
    }

    fn read_message(&mut self, timeout: Duration) -> Result<Option<Value>, WayfireClientError> {
        let deadline = Instant::now() + timeout;
        loop {
            if let Some(message) = self.decode_message()? {
                return Ok(Some(message));
            }

            let mut buffer = [0_u8; 8192];
            match self.stream.read(&mut buffer) {
                Ok(0) => return Err(WayfireClientError::Disconnected),
                Ok(byte_count) => self.read_buffer.extend_from_slice(&buffer[..byte_count]),
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    if Instant::now() >= deadline {
                        return Ok(None);
                    }
                    thread::sleep(IO_RETRY_INTERVAL);
                }
                Err(error) => return Err(WayfireClientError::Read(error)),
            }
        }
    }

    fn decode_message(&mut self) -> Result<Option<Value>, WayfireClientError> {
        if self.read_buffer.len() < HEADER_LENGTH {
            return Ok(None);
        }
        let payload_length = u32::from_le_bytes([
            self.read_buffer[0],
            self.read_buffer[1],
            self.read_buffer[2],
            self.read_buffer[3],
        ]) as usize;
        if payload_length > MAXIMUM_MESSAGE_LENGTH {
            return Err(WayfireClientError::MessageTooLarge(payload_length));
        }
        let frame_length = HEADER_LENGTH + payload_length;
        if self.read_buffer.len() < frame_length {
            return Ok(None);
        }
        let message = serde_json::from_slice(&self.read_buffer[HEADER_LENGTH..frame_length])
            .map_err(WayfireClientError::Decode)?;
        self.read_buffer.drain(..frame_length);
        Ok(Some(message))
    }
}

#[derive(Debug, Error)]
pub(super) enum WayfireClientError {
    #[error("failed to connect to the Wayfire IPC socket: {0}")]
    Connect(#[source] io::Error),
    #[error("failed to configure the Wayfire IPC socket: {0}")]
    ConfigureSocket(#[source] io::Error),
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
