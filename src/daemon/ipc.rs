//! IPC server implementation using Unix domain sockets.

use crate::constants::BUFFER_SIZE;
use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

/// Creates a Unix socket listener at the given path.
pub fn bind_socket<P: AsRef<Path>>(path: P) -> Result<UnixListener> {
    let listener = UnixListener::bind(path.as_ref()).context("Failed to bind Unix socket")?;
    Ok(listener)
}

/// Accepts a connection and reads the command.
/// Returns the command string and the client stream.
pub fn accept_command(listener: &UnixListener) -> Result<(String, UnixStream)> {
    let (mut stream, _addr) = listener.accept().context("Failed to accept connection")?;

    let mut buf = [0u8; BUFFER_SIZE];
    let len = stream
        .read(&mut buf)
        .context("Failed to read command from client")?;

    let command = String::from_utf8_lossy(&buf[..len]).to_string();
    Ok((command, stream))
}

/// Sends a response to a client.
pub fn send_response(stream: &mut UnixStream, response: &str) -> Result<()> {
    stream
        .write_all(response.as_bytes())
        .context("Failed to send response")?;
    Ok(())
}
