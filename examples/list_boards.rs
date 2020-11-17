//! Make sure to run the following before:
//! `arduino-cli daemon`

use arduino_cli_client::commands::arduino_core_client::ArduinoCoreClient;
use arduino_cli_client::commands::{BoardListReq, InitReq};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ArduinoCoreClient::connect("http://localhost:50051").await?;

    // Start a new instance of the Arduino Core Service
    let mut init_stream = client
        .init(Request::new(InitReq {
            library_manager_only: false,
        }))
        .await?
        .into_inner();

    let resp_instance = init_stream.message().await?.expect("Failed to init");

    // List the boards currently connected to the computer.
    let resp_boards = client
        .board_list(Request::new(BoardListReq {
            instance: resp_instance.instance,
        }))
        .await?
        .into_inner();

    print!("Boards: {:?}", resp_boards.ports);

    Ok(())
}
