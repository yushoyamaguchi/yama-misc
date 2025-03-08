use std::env;
use std::fs;
use wireguard1::wire_guard_service_client::WireGuardServiceClient;
use wireguard1::WireGuardRequest;

pub mod wireguard1 {
    tonic::include_proto!("wireguard1");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <json_file_path>", args[0]);
        std::process::exit(1);
    }
    
    let json_file_path = &args[1];
    
    // JSONファイルを読み込む
    let json_content = fs::read_to_string(json_file_path)?;
    
    // WireGuardRequest メッセージを構築
    let wireguard_request = WireGuardRequest {
        json_payload: json_content,
    };
    
    // gRPCクライアントを作成
    let channel = tonic::transport::Channel::from_static("http://192.168.1.2:50051")
        .connect()
        .await?;
    let mut client = WireGuardServiceClient::new(channel);
    
    println!("Sending raw JSON WireGuard configuration request...");
    
    // JSONをそのまま送信
    let request = tonic::Request::new(wireguard_request);
    let response = client.configure_wire_guard(request).await?;
    
    println!("RESPONSE={:?}", response);

    Ok(())
}
