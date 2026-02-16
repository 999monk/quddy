//! Client module for communicating with the daemon.

use crate::constants::{BUFFER_SIZE, SOCKET_TIMEOUT_SECS, WRITE_TIMEOUT_SECS};
use anyhow::{bail, Context, Result};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

/// Sends a command to the daemon and waits for a response.
pub fn send_command<P: AsRef<Path>>(socket_path: P, command: &str) -> Result<String> {
    // Connect to daemon
    let mut stream = UnixStream::connect(socket_path.as_ref())
        .context("Failed to connect to daemon (is it running?)")?;

    // Set timeouts
    stream
        .set_read_timeout(Some(Duration::from_secs(SOCKET_TIMEOUT_SECS)))
        .context("Failed to set read timeout")?;
    stream
        .set_write_timeout(Some(Duration::from_secs(WRITE_TIMEOUT_SECS)))
        .context("Failed to set write timeout")?;

    // Send command
    stream
        .write_all(command.as_bytes())
        .context("Failed to send command")?;

    // Read response
    let mut buf = [0u8; BUFFER_SIZE];
    let len = stream
        .read(&mut buf)
        .context("Failed to read response from daemon")?;

    let response = String::from_utf8_lossy(&buf[..len]).to_string();

    Ok(response)
}

/// Checks if the daemon is running.
pub fn check_daemon<P: AsRef<Path>>(socket_path: P) -> Result<bool> {
    match send_command(socket_path, "PING") {
        Ok(response) => Ok(response == "PONG"),
        Err(_) => Ok(false),
    }
}

/// Sends a capture command to the daemon.
pub fn capture<P: AsRef<Path>>(socket_path: P) -> Result<()> {
    let response = send_command(socket_path, "CAPTURE")?;

    match response.as_str() {
        "OK" => Ok(()),
        "CANCELLED" => Ok(()), // User cancelled, not an error
        _ => {
            // Check for error prefix safely using strip_prefix
            if let Some(error_msg) = response.strip_prefix("ERROR:") {
                bail!("{}", error_msg.trim());
            }
            bail!("Unexpected response: {}", response)
        }
    }
}

/// Stops the daemon.
pub fn stop_daemon<P: AsRef<Path>>(socket_path: P) -> Result<()> {
    send_command(socket_path, "STOP")?;
    Ok(())
}
