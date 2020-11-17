//! Analogue to arduino-cli `client_example`.
//!
//! Make sure to run the following before:
//! `arduino-cli daemon`

use arduino_cli_client::commands::arduino_core_client::ArduinoCoreClient;
use arduino_cli_client::commands::{
    InitReq, LoadSketchReq, PlatformInstallReq, PlatformListReq, PlatformSearchReq, UpdateIndexReq,
    VersionReq,
};
use arduino_cli_client::settings::{
    settings_client::SettingsClient, GetAllRequest, GetValueRequest, RawData, Value,
};
use serde_json::json;
// use futures_util::{future, stream, StreamExt};
use tokio::stream::StreamExt;
// use futures_util::stream::StreamExt;
use std::{env, fs};

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

    let resp = client.version(VersionReq {}).await?.into_inner();
    println!("arduino-cli version: {}", resp.version);

    let mut sketch_path = env::current_dir()?;
    sketch_path.push("examples/hello");
    let sketch_path = sketch_path.to_string_lossy().to_string();
    let resp = client
        .load_sketch(LoadSketchReq {
            sketch_path,
            ..Default::default()
        })
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
        .set_value(Value {
            key: "directories".to_string(),
            json_data,
        })
        .await?;

    // List all the settings.
    let resp = settings_client
        .get_all(GetAllRequest {})
        .await?
        .into_inner();
    println!("Settings: {:?}", resp.json_data);

    // Merge applies multiple settings values at once.
    let json_data = json!(
        {"foo": "bar", "daemon":{"port":"422"}}
    )
    .to_string();
    settings_client.merge(RawData { json_data }).await?;

    // Get the value of the foo key.
    settings_client
        .get_value(GetValueRequest {
            key: "foo".to_string(),
        })
        .await?;

    // Before we can do anything with the CLI, an "instance" must be created.
    // We keep a reference to the created instance because we will need it to
    // run subsequent commands.
    let mut init_stream = client
        .init(InitReq {
            library_manager_only: false,
        })
        .await?
        .into_inner();
    let resp_instance = init_stream.message().await?.expect("Failed to init");

    // With a brand new instance, the first operation should always be updating
    // the index.
    let mut update_index_stream = client
        .update_index(UpdateIndexReq {
            instance: resp_instance.instance.clone(),
        })
        .await?
        .into_inner();
    while let Some(update_index_resp) = update_index_stream.next().await {
        let resp = update_index_resp?;
        println!("DOWNLOAD: {:?}", resp.download_progress.unwrap_or_default());
    }
    println!("Update index done");

    // Let's search for a platform (also known as 'core') called 'samd'.
    let resp = client
        .platform_search(PlatformSearchReq {
            instance: resp_instance.instance.clone(),
            search_args: "samd".to_string(),
            all_versions: true,
        })
        .await?
        .into_inner();

    for platform in resp.search_output {
        println!("Search result: {} - {}", platform.id, platform.latest);
    }

    // Install arduino:samd@1.6.19
    let mut resp = client
        .platform_install(PlatformInstallReq {
            instance: resp_instance.instance.clone(),
            platform_package: "arduino".to_string(),
            architecture: "samd".to_string(),
            version: "1.6.19".to_string(),
            ..Default::default()
        })
        .await?
        .into_inner();

    while let Some(install_resp) = resp.next().await {
        let resp = install_resp?;
        if let Some(progress) = resp.progress {
            println!("DOWNLOAD: {:?}", progress);
        }
        if let Some(task_progress) = resp.task_progress {
            println!("TASK: {:?}", task_progress);
        }
    }
    println!("Install done");

    // Now list the installed platforms to double check previous installation
    // went right.
    let resp = client
        .platform_list(PlatformListReq {
            instance: resp_instance.instance.clone(),
            ..Default::default()
        })
        .await?
        .into_inner();

    for platform in resp.installed_platform {
        println!(
            "Installed platform: {} - {}",
            platform.id, platform.installed
        );
    }

    Ok(())
}
