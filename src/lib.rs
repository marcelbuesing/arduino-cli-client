//! # Example
//!
//! Make sure to run the following before:
//! ```text
//! arduino-cli daemon
//! ```
//!
//! ## Code
//!
//! ```rust
//! use arduino_cli_client::commands::arduino_core_client::ArduinoCoreClient;
//! use arduino_cli_client::commands::{BoardListReq, InitReq};
//! use tonic::Request;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let mut client = ArduinoCoreClient::connect("http://localhost:50051").await?;
//!
//!    // Start a new instance of the Arduino Core Service
//!    let resp_instance = client
//!        .init(Request::new(InitReq {
//!            library_manager_only: false,
//!        }))
//!        .await?
//!        .into_inner()
//!        .message()
//!        .await?
//!        .expect("Failed to init");
//!
//!    // List the boards currently connected to the computer.
//!    let resp_boards = client
//!        .board_list(Request::new(BoardListReq {
//!            instance: resp_instance.instance,
//!        }))
//!        .await?
//!        .into_inner();
//!
//!    print!("Boards: {:?}", resp_boards.ports);
//!    Ok(())
//! }
//! ```
//!

/// Main Arduino Platform service
pub mod commands {
    tonic::include_proto!("cc.arduino.cli.commands");
}

/// Service that abstract a debug Session usage
pub mod debug {
    tonic::include_proto!("cc.arduino.cli.debug");
}

/// Service that abstracts a Monitor usage
pub mod monitor {
    tonic::include_proto!("cc.arduino.cli.monitor");
}

/// The Settings service provides an interface to Arduino CLI's configuration options
pub mod settings {
    tonic::include_proto!("cc.arduino.cli.settings");
}

pub use crate::commands::arduino_core_client::ArduinoCoreClient;
pub use crate::debug::debug_client::DebugClient;
pub use crate::monitor::monitor_client::MonitorClient;
pub use crate::settings::settings_client::SettingsClient;
