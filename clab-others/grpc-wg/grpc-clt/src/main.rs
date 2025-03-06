use std::env;
use std::fs;
use std::io::Read;
use wireguard1::wire_guard_service_client::WireGuardServiceClient;
use wireguard1::{WireGuardRequest, Overlay, WireGuardConfiguration, WireGuardPeer};
use serde::{Deserialize, Serialize};

pub mod wireguard1 {
    tonic::include_proto!("wireguard1");
}

// JSON構造体の定義 - serde_deriveを使用
#[derive(Debug, Deserialize, Serialize)]
struct WireGuardRequestJson {
    id: i32,
    overlays: Vec<OverlayJson>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OverlayJson {
    #[serde(rename = "type")]
    overlay_type: String,
    myself: WireGuardConfigurationJson,
    peer: WireGuardPeerJson,
}

#[derive(Debug, Deserialize, Serialize)]
struct WireGuardConfigurationJson {
    wg_address: String,
    port: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct WireGuardPeerJson {
    endpoint_address: String,
    endpoint_port: i32,
    publickey: String,
    allowips: Vec<String>,
    persistentkeepalive: i32,
}

// JSONモデルからprotobufモデルへの変換機能
impl From<WireGuardRequestJson> for WireGuardRequest {
    fn from(json: WireGuardRequestJson) -> Self {
        let overlays = json.overlays.into_iter()
            .map(|overlay_json| {
                Overlay {
                    r#type: overlay_json.overlay_type,
                    myself: Some(WireGuardConfiguration {
                        wg_address: overlay_json.myself.wg_address,
                        port: overlay_json.myself.port,
                    }),
                    peer: Some(WireGuardPeer {
                        endpoint_address: overlay_json.peer.endpoint_address,
                        endpoint_port: overlay_json.peer.endpoint_port,
                        publickey: overlay_json.peer.publickey,
                        allowips: overlay_json.peer.allowips,
                        persistentkeepalive: overlay_json.peer.persistentkeepalive,
                    }),
                }
            })
            .collect();

        WireGuardRequest {
            id: json.id,
            overlays,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // コマンドライン引数からJSONファイルのパスを取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <json_file_path>", args[0]);
        std::process::exit(1);
    }
    
    let json_file_path = &args[1];
    
    // JSONファイルを読み込む
    let mut file = fs::File::open(json_file_path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;
    
    // JSONを直接デシリアライズ
    let wireguard_json: WireGuardRequestJson = serde_json::from_str(&json_content)?;
    
    // ProtobufモデルへRustの型変換トレイトを使って変換
    let wireguard_request = WireGuardRequest::from(wireguard_json);
    
    // サーバーへのチャネルを作成
    let channel = tonic::transport::Channel::from_static("http://192.168.1.2:50051")
        .connect()
        .await?;

    // クライアントを作成
    let mut client = WireGuardServiceClient::new(channel);
    
    println!("Sending WireGuard configuration request...");
    
    // WireGuard設定リクエストを送信
    let request = tonic::Request::new(wireguard_request);
    let response = client.configure_wire_guard(request).await?;
    
    // レスポンスを表示
    println!("RESPONSE={:?}", response);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_json_to_wireguard_request() {
        let json_str = r#"{
            "id": 12345,
            "overlays": [
                {
                    "type": "wireguard",
                    "myself": {
                        "wg_address": "10.0.0.2",
                        "port": 51820
                    },
                    "peer": {
                            "endpoint_address": "192.168.1.1",
                            "endpoint_port" : 51820,
                            "publickey": "pseudo-public-key",
                            "allowips": [
                                "10.0.0.0/24"
                            ],
                            "persistentkeepalive": 25
                    }
                }
            ]
        }"#;

        // JSONを直接デシリアライズ
        let wireguard_json: WireGuardRequestJson = serde_json::from_str(json_str).unwrap();
        
        // JSONモデルを検証
        assert_eq!(wireguard_json.id, 12345);
        assert_eq!(wireguard_json.overlays.len(), 1);
        assert_eq!(wireguard_json.overlays[0].overlay_type, "wireguard");
        
        // ProtobufモデルへRustの型変換トレイトを使って変換
        let result = WireGuardRequest::from(wireguard_json);
        
        // Protobufモデルを検証
        assert_eq!(result.id, 12345);
        assert_eq!(result.overlays.len(), 1);
        
        let overlay = &result.overlays[0];
        assert_eq!(overlay.r#type, "wireguard");
        
        // myselfを検証
        let myself = overlay.myself.as_ref().unwrap();
        assert_eq!(myself.wg_address, "10.0.0.2");
        assert_eq!(myself.port, 51820);
        
        // peerを検証
        let peer = overlay.peer.as_ref().unwrap();
        assert_eq!(peer.endpoint_address, "192.168.1.1");
        assert_eq!(peer.endpoint_port, 51820);
        assert_eq!(peer.publickey, "pseudo-public-key");
        assert_eq!(peer.allowips.len(), 1);
        assert_eq!(peer.allowips[0], "10.0.0.0/24");
        assert_eq!(peer.persistentkeepalive, 25);
        
        println!("All JSON deserialization and conversion tests passed!");
    }
}