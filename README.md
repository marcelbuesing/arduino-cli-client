![arduino-cli-client](https://socialify.git.ci/marcelbuesing/arduino-cli-client/image?description=1&descriptionEditable=arduino-cli%20%20gRPC%20client%20bindings%20for%20Rust&font=Inter&forks=1&issues=1&language=1&pattern=Circuit%20Board&pulls=1&stargazers=1&theme=Light)

gRPC bindings for [arduino-cli](https://github.com/arduino/arduino-cli).
Bindings are generated based on the protobuf definitions defined [here](https://github.com/arduino/arduino-cli/tree/master/rpc).


## Example

This examples demonstrates retrieving the list of connected boards.
Make sure you run `arduino-cli daemon` before running this example.

You can run the example via `cargo run --example list_boards`.

```Rust
use arduino_cli_client::commands::arduino_core_client::ArduinoCoreClient;
use arduino_cli_client::commands::{BoardListReq, InitReq};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ArduinoCoreClient::connect("http://localhost:50051").await?;

    // Start a new instance of the Arduino Core Service
    let mut init_stream = client
        .init(InitReq {
            library_manager_only: false,
        })
        .await?
        .into_inner();

    let resp_instance = init_stream.message().await?.expect("Failed to init");

    // List the boards currently connected to the computer.
    let resp_boards = client
        .board_list(BoardListReq {
            instance: resp_instance.instance,
        })
        .await?
        .into_inner();

    print!("Boards: {:?}", resp_boards.ports);

    Ok(())
}
```

## Example - Client

The `examples/` folder contains a client example that is analogue to `arduino-cli/client-example`.
