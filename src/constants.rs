//! Application-wide constants and path utilities.

use std::path::PathBuf;

/// Directory name for quddy data
pub const QUDDY_DIR: &str = "quddy";

/// Socket filename
pub const SOCKET_NAME: &str = "quddy.sock";

/// Output filename
pub const OUTPUT_FILENAME: &str = "output.txt";

/// Log filename
pub const LOG_FILENAME: &str = "quddy.log";

/// Buffer size for socket communication
pub const BUFFER_SIZE: usize = 65536;

/// Socket timeout in seconds
pub const SOCKET_TIMEOUT_SECS: u64 = 30;

/// Write timeout in seconds
pub const WRITE_TIMEOUT_SECS: u64 = 5;

/// Gets the quddy data directory (~/.local/share/quddy)
pub fn data_dir() -> PathBuf {
    dirs::data_local_dir()
        .map(|d| d.join(QUDDY_DIR))
        .unwrap_or_else(|| PathBuf::from("/tmp").join(QUDDY_DIR))
}

/// Gets the runtime directory for socket (~/.local/share/quddy or /run/user/UID/quddy)
pub fn runtime_dir() -> PathBuf {
    dirs::runtime_dir()
        .or_else(dirs::data_local_dir)
        .map(|d| d.join(QUDDY_DIR))
        .unwrap_or_else(|| PathBuf::from("/tmp").join(QUDDY_DIR))
}

/// Gets the full path to the socket file
pub fn socket_path() -> PathBuf {
    runtime_dir().join(SOCKET_NAME)
}

/// Gets the full path to the output file
pub fn output_path() -> PathBuf {
    data_dir().join(OUTPUT_FILENAME)
}

/// Gets the full path to the log file
pub fn log_path() -> PathBuf {
    data_dir().join(LOG_FILENAME)
}

/// Gets the config directory (~/.config/quddy)
pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .map(|d| d.join(QUDDY_DIR))
        .unwrap_or_else(|| PathBuf::from(".").join(QUDDY_DIR))
}
