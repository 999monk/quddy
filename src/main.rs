pub mod capture;
pub mod client;
pub mod config;
pub mod constants;
pub mod daemon;
pub mod ocr;
pub mod translate;

use crate::constants::socket_path;
use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "quddy")]
#[command(about = "OCR screen capture and translation tool")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the background daemon
    Start,
    /// Capture screen region and translate (sends command to daemon)
    Capture,
    /// Check if daemon is running
    Status,
    /// Stop the daemon
    Stop,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let socket_path = socket_path();

    match cli.command {
        Commands::Start => {
            // Check if daemon already running
            if client::check_daemon(&socket_path)? {
                bail!("Quddy daemon is already running");
            }

            let config = config::Config::load().context("Failed to load configuration")?;
            println!("Starting Quddy daemon...");
            println!("Output file: {:?}", constants::output_path());
            daemon::run_daemon(&config, &socket_path)?;
        }
        Commands::Capture => {
            // Check if daemon is running
            if !client::check_daemon(&socket_path)? {
                bail!("Quddy daemon is not running. Start it with: quddy start");
            }

            client::capture(&socket_path).context("Failed to capture")?;
            println!("Translation saved to: {:?}", constants::output_path());
        }
        Commands::Status => {
            if client::check_daemon(&socket_path)? {
                println!("Quddy daemon is running");
            } else {
                println!("Quddy daemon is not running");
            }
        }
        Commands::Stop => {
            if !client::check_daemon(&socket_path)? {
                println!("Quddy daemon is not running");
                return Ok(());
            }

            client::stop_daemon(&socket_path).context("Failed to stop daemon")?;
            println!("Quddy daemon stopped");
        }
    }

    Ok(())
}
