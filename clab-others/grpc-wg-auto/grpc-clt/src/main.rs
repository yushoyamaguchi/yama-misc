mod wireguard1;

use std::env;
use std::fs;
use std::io::Read;

use tonic::transport::Channel;

// Bring in generated code from mod.rs
use wireguard1::wireguard1::WireGuardRequest;
use wireguard1::wireguard1::wire_guard_service_client::WireGuardServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Read path from CLI
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <json_file_path>", args[0]);
        std::process::exit(1);
    }
    let json_file_path = &args[1];

    // 2) Load JSON from file
    let mut file = fs::File::open(json_file_path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;

    // 3) Deserialize into generated WireGuardRequest
    //    (works because we derived Serialize + Deserialize in build.rs)
    let wireguard_request: WireGuardRequest = serde_json::from_str(&json_content)?;

    // 4) Create a gRPC client
    let channel = Channel::from_static("http://192.168.1.2:50051")
        .connect()
        .await?;
    let mut client = WireGuardServiceClient::new(channel);

    // 5) Send the request
    let response = client.configure_wire_guard(wireguard_request).await?;

    // 6) Print response
    println!("RESPONSE: {:?}", response.into_inner());

    Ok(())
}
