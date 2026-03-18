//! The daemon module, responsible for running the background service.

pub mod ipc;

use crate::{capture, config::Config, constants, ocr, translate};
use anyhow::{Context, Result};
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

/// Logs a message to the log file with strict error handling.
fn log(message: &str) -> Result<()> {
    let log_path = constants::log_path();
    let log_dir = log_path
        .parent()
        .context("Log path has no parent directory")?;

    // Create directory if it doesn't exist
    std::fs::create_dir_all(log_dir)
        .with_context(|| format!("Failed to create log directory at {:?}", log_dir))?;

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_line = format!("[{}] {}\n", timestamp, message);

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .with_context(|| format!("Failed to open log file at {:?}", log_path))?;

    file.write_all(log_line.as_bytes())
        .with_context(|| format!("Failed to write to log file at {:?}", log_path))?;

    Ok(())
}

/// Writes translation to output file (overwrites previous content).
fn write_translation(translation: &str) -> Result<()> {
    let output_path = constants::output_path();
    let output_dir = output_path
        .parent()
        .context("Output path has no parent directory")?;

    // Create directory if it doesn't exist
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory at {:?}", output_dir))?;

    std::fs::write(&output_path, translation)
        .with_context(|| format!("Failed to write translation to {:?}", output_path))?;

    Ok(())
}

/// Runs the daemon loop, listening for commands on the Unix socket.
pub fn run_daemon(config: &Config, socket_path: &PathBuf) -> Result<()> {
    // Ensure the runtime directory exists
    if let Some(parent) = socket_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create runtime directory at {:?}", parent))?;
    }

    // Remove stale socket if it exists
    if socket_path.exists() {
        std::fs::remove_file(socket_path)
            .with_context(|| format!("Failed to remove stale socket at {:?}", socket_path))?;
    }

    let listener = ipc::bind_socket(socket_path).context("Failed to bind Unix socket")?;

    println!("Quddy daemon started. Listening on {:?}", socket_path);
    println!("Output file: {:?}", constants::output_path());
    log("Daemon started")?;

    loop {
        match ipc::accept_command(&listener) {
            Ok((command, mut stream)) => match command.as_str() {
                "CAPTURE" => {
                    let response = handle_capture(config);
                    if let Err(e) = ipc::send_response(&mut stream, &response) {
                        eprintln!("Error sending response: {}", e);
                        let _ = log(&format!("Error sending response: {}", e));
                    }
                }
                "PING" => {
                    if let Err(e) = ipc::send_response(&mut stream, "PONG") {
                        eprintln!("Error sending PONG response: {}", e);
                    }
                }
                "STOP" => {
                    if let Err(e) = ipc::send_response(&mut stream, "OK") {
                        eprintln!("Error sending OK response: {}", e);
                    }
                    log("Daemon stopping (STOP command received)")?;
                    println!("Stop command received. Shutting down...");
                    break;
                }
                _ => {
                    let _ = ipc::send_response(&mut stream, "ERROR: Unknown command");
                }
            },
            Err(e) => {
                eprintln!("Error accepting command: {}", e);
                let _ = log(&format!("Error accepting command: {}", e));
            }
        }
    }

    // Cleanup socket
    if let Err(e) = std::fs::remove_file(socket_path) {
        eprintln!("Warning: Failed to remove socket file: {}", e);
    }

    let _ = log("Daemon stopped");
    println!("Quddy daemon stopped.");
    Ok(())
}

/// Handles the capture command: capture → OCR → translate → write to file.
fn handle_capture(config: &Config) -> String {
    if let Err(e) = log("CAPTURE: Starting capture process") {
        eprintln!("Failed to log: {}", e);
    }

    let start = Instant::now();

    match capture::select_and_capture_region(&config.capture) {
        Ok(image) => {
            let elapsed = start.elapsed();
            if let Err(e) = log(&format!("CAPTURE: Image captured in {:?}", elapsed)) {
                eprintln!("Failed to log: {}", e);
            }

            match ocr::perform_ocr(image, &config.ocr) {
                Ok(ocr_text) => {
                    let elapsed = start.elapsed();
                    if let Err(e) = log(&format!(
                        "CAPTURE: OCR completed in {:?}, text length: {}",
                        elapsed,
                        ocr_text.len()
                    )) {
                        eprintln!("Failed to log: {}", e);
                    }

                    match translate::translate_text(&ocr_text, &config.translation) {
                        Ok(translation) => {
                            let elapsed = start.elapsed();
                            if let Err(e) =
                                log(&format!("CAPTURE: Translation completed in {:?}", elapsed))
                            {
                                eprintln!("Failed to log: {}", e);
                            }

                            // Write translation to output file
                            if let Err(e) = write_translation(&translation) {
                                let err_msg = format!("Failed to write output - {}", e);
                                if let Err(log_e) = log(&format!("CAPTURE ERROR: {}", err_msg)) {
                                    eprintln!("Failed to log: {}", log_e);
                                }
                                return format!("ERROR: {}", err_msg);
                            }

                            if let Err(e) = log(&format!(
                                "CAPTURE: Translation written to output file ({} bytes)",
                                translation.len()
                            )) {
                                eprintln!("Failed to log: {}", e);
                            }
                            "OK".to_string()
                        }
                        Err(e) => {
                            let err_msg = format!("Translation failed - {}", e);
                            if let Err(log_e) = log(&format!("CAPTURE ERROR: {}", err_msg)) {
                                eprintln!("Failed to log: {}", log_e);
                            }
                            format!("ERROR: {}", err_msg)
                        }
                    }
                }
                Err(e) => {
                    if e.to_string().contains("did not detect any text") {
                        if let Err(log_e) = log("CAPTURE: No text detected") {
                            eprintln!("Failed to log: {}", log_e);
                        }
                        "ERROR: No text detected".to_string()
                    } else {
                        let err_msg = format!("OCR failed - {}", e);
                        if let Err(log_e) = log(&format!("CAPTURE ERROR: {}", err_msg)) {
                            eprintln!("Failed to log: {}", log_e);
                        }
                        format!("ERROR: {}", err_msg)
                    }
                }
            }
        }
        Err(e) => {
            if e.to_string().contains("cancelled") {
                if let Err(log_e) = log("CAPTURE: User cancelled") {
                    eprintln!("Failed to log: {}", log_e);
                }
                "CANCELLED".to_string()
            } else {
                let err_msg = format!("Capture failed - {}", e);
                if let Err(log_e) = log(&format!("CAPTURE ERROR: {}", err_msg)) {
                    eprintln!("Failed to log: {}", log_e);
                }
                format!("ERROR: {}", err_msg)
            }
        }
    }
}
