use tonic::{transport::Server, Request, Response, Status};
use serde::{Deserialize, Serialize};
use wireguard1::wire_guard_service_server::{WireGuardService, WireGuardServiceServer};
use wireguard1::{HelloReply, HelloRequest, WireGuardRequest, WireGuardResponse};

pub mod wireguard1 {
    tonic::include_proto!("wireguard1");
}

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

fn parse_wireguard_json(json_str: &str) -> Result<WireGuardRequestJson, String> {
    serde_json::from_str(json_str).map_err(|_| "Invalid JSON format".to_string())
}

#[derive(Debug, Default)]
pub struct MyWireGuardService {}

#[tonic::async_trait]
impl WireGuardService for MyWireGuardService {
    async fn configure_wire_guard(
        &self,
        request: Request<WireGuardRequest>,
    ) -> Result<Response<WireGuardResponse>, Status> {
        let req = request.into_inner();

        println!("Received raw JSON payload: {}", req.json_payload);

        // JSONのパース処理
        let wireguard_json = match parse_wireguard_json(&req.json_payload) {
            Ok(json) => json,
            Err(err_msg) => {
                return Ok(Response::new(WireGuardResponse {
                    success: false,
                    message: err_msg,
                }));
            }
        };

        println!("Parsed WireGuard configuration request with ID: {}", wireguard_json.id);

        // IDのチェック
        if wireguard_json.id <= 0 {
            println!("Invalid ID: {}", wireguard_json.id);
            return Ok(Response::new(WireGuardResponse {
                success: false,
                message: "Invalid ID provided".to_string(),
            }));
        }

        // オーバーレイのログ出力
        for (i, overlay) in wireguard_json.overlays.iter().enumerate() {
            println!("Overlay {}: type={}", i, overlay.overlay_type);
            println!("  Myself: wg_address={}, port={}", overlay.myself.wg_address, overlay.myself.port);
            println!("  Peer: endpoint_address={}, endpoint_port={}", overlay.peer.endpoint_address, overlay.peer.endpoint_port);
            println!("    publickey={}", overlay.peer.publickey);
            println!("    allowips={:?}", overlay.peer.allowips);
            println!("    persistentkeepalive={}", overlay.peer.persistentkeepalive);
        }

        Ok(Response::new(WireGuardResponse {
            success: true,
            message: format!("Successfully configured WireGuard with ID: {}", wireguard_json.id),
        }))
    }

    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a hello request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "192.168.1.2:50051".parse()?;
    let wire_guard_service = MyWireGuardService::default();

    println!("WireGuard gRPC server starting on {}", addr);

    Server::builder()
        .add_service(WireGuardServiceServer::new(wire_guard_service))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wireguard_json_valid() {
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
                        "endpoint_port": 51820,
                        "publickey": "pseudo-public-key",
                        "allowips": ["10.0.0.0/24"],
                        "persistentkeepalive": 25
                    }
                }
            ]
        }"#;

        let result = parse_wireguard_json(json_str);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_wireguard_json_invalid() {
        let invalid_json_str = r#"{ "id": "invalid_id" }"#;
        let result = parse_wireguard_json(invalid_json_str);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid JSON format".to_string());
    }
}
