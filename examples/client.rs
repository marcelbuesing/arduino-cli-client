//! Analogue to arduino-cli `client_example`.
//!
//! Make sure to run the following before:
//! `arduino-cli daemon`

use arduino_cli_client::commands::arduino_core_client::ArduinoCoreClient;
use arduino_cli_client::commands::{BoardListReq, InitReq, LoadSketchReq, VersionReq};
use arduino_cli_client::settings::{settings_client::SettingsClient, GetAllRequest, Value};
use serde_json::json;
use std::{env, fs};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data_dir = env::temp_dir();
    data_dir.push("arduino-rpc-client");
    let data_dir = data_dir.as_path();

    if !data_dir.exists() {
        fs::create_dir(&data_dir)?;
    }

    let mut client = ArduinoCoreClient::connect("http://localhost:50051").await?;
    let mut settings_client = SettingsClient::connect("http://localhost:50051").await?;

    let resp = client
        .version(Request::new(VersionReq {}))
        .await?
        .into_inner();
    println!("arduino-cli version: {}", resp.version);

    let mut sketch_path = env::current_dir()?;
    sketch_path.push("examples/hello");
    let sketch_path = sketch_path.to_string_lossy().to_string();
    let resp = client
        .load_sketch(Request::new(LoadSketchReq {
            sketch_path,
            ..Default::default()
        }))
        .await?
        .into_inner();
    println!("Sketch main file: {}", resp.main_file);
    println!("Sketch location: {}", resp.location_path);
    println!("Other sketch files: {:?}", resp.other_sketch_files);
    println!("Sketch additional files: {:?}", resp.additional_files);

    // Use SetValue to configure the arduino-cli directories.
    let json_data = json!({
        "data": data_dir,
        "downloads": data_dir.join("staging"),
        "user": data_dir.join("sketchbook"),
    })
    .to_string();
    settings_client
        .set_value(Request::new(Value {
            key: "directories".to_string(),
            json_data,
        }))
        .await?;

    // List all the settings.
    let resp = settings_client
        .get_all(Request::new(GetAllRequest {}))
        .await?
        .into_inner();
    println!("Settings: {:?}", resp.json_data);

    // Start a new instance of the Arduino Core Service
    let resp_instance = client
        .init(Request::new(InitReq {
            library_manager_only: false,
        }))
        .await?
        .into_inner()
        .message()
        .await?
        .expect("Failed to init");

    // List the boards currently connected to the computer.
    let resp = client
        .board_list(Request::new(BoardListReq {
            instance: resp_instance.instance,
        }))
        .await?
        .into_inner();

    print!("Boards: {:?}", resp.ports);

    Ok(())
}
