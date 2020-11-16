# Arduino CLI client

Rust gRPC bindings for [arduino-cli](https://github.com/arduino/arduino-cli).

Bindings are generated based on the protobuf definitions defined [here](https://github.com/arduino/arduino-cli/tree/master/rpc).


## Example

This examples demonstrates retrieving the list of connected boards.
Make sure you run `arduino-cli daemon` before running this example.

You can run the example via `cargo run --example list_boards`.

```Rust
use arduino_cli_client::commands::arduino_core_client::ArduinoCoreClient;
use arduino_cli_client::commands::{BoardListReq, InitReq};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ArduinoCoreClient::connect("http://localhost:50051").await?;

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
    let resp_boards = client
        .board_list(Request::new(BoardListReq {
            instance: resp_instance.instance,
        }))
        .await?
        .into_inner();

    print!("Boards: {:?}", resp_boards.ports);

    Ok(())
}

```

## Example - Client

The `examples/` folder contains a client example that is analogue to `arduino-cli/client-example`.
