use std::env;
use std::fs;
use std::io::Read;
use wireguard1::wire_guard_service_client::WireGuardServiceClient;
use wireguard1::{WireGuardRequest, Overlay, WireGuardConfiguration, WireGuardPeer};
use serde_json::Value;

pub mod wireguard1 {
    tonic::include_proto!("wireguard1");
}

fn parse_json_to_wireguard_request(json_str: &str) -> Result<WireGuardRequest, Box<dyn std::error::Error>> {
    let v: Value = serde_json::from_str(json_str)?;
    
    // Extract values from JSON
    let id = v["id"].as_i64().unwrap_or(0) as i32;
    
    let mut overlays = Vec::new();
    
    for overlay_value in v["overlays"].as_array().unwrap_or(&Vec::new()) {
        let overlay_type = overlay_value["type"].as_str().unwrap_or("").to_string();
        
        // Extract myself configuration
        let myself_value = &overlay_value["myself"];
        let myself = WireGuardConfiguration {
            wg_address: myself_value["wg_address"].as_str().unwrap_or("").to_string(),
            port: myself_value["port"].as_i64().unwrap_or(0) as i32,
        };
        
        // Extract peer configuration
        let peer_value = &overlay_value["peer"];
        let mut allow_ips = Vec::new();
        if let Some(allow_ips_array) = peer_value["allowips"].as_array() {
            for ip in allow_ips_array {
                allow_ips.push(ip.as_str().unwrap_or("").to_string());
            }
        }
        
        let peer = WireGuardPeer {
            endpoint_address: peer_value["endpoint_address"].as_str().unwrap_or("").to_string(),
            endpoint_port: peer_value["endpoint_port"].as_i64().unwrap_or(0) as i32,
            publickey: peer_value["publickey"].as_str().unwrap_or("").to_string(),
            allowips: allow_ips,
            persistentkeepalive: peer_value["persistentkeepalive"].as_i64().unwrap_or(0) as i32,
        };
        
        let overlay = Overlay {
            r#type: overlay_type,
            myself: Some(myself),
            peer: Some(peer),
        };
        
        overlays.push(overlay);
    }
    
    Ok(WireGuardRequest {
        id,
        overlays,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get JSON file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <json_file_path>", args[0]);
        std::process::exit(1);
    }
    
    let json_file_path = &args[1];
    
    // Read the JSON file
    let mut file = fs::File::open(json_file_path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;
    
    // Create a channel to the server
    let channel = tonic::transport::Channel::from_static("http://192.168.1.2:50051")
        .connect()
        .await?;

    // Create the client
    let mut client = WireGuardServiceClient::new(channel);
    
    // Parse JSON to WireGuardRequest
    let wireguard_request = parse_json_to_wireguard_request(&json_content)?;
    
    println!("Sending WireGuard configuration request...");
    
    // Send the WireGuard configuration request
    let request = tonic::Request::new(wireguard_request);
    let response = client.configure_wire_guard(request).await?;
    
    // Print the response
    println!("RESPONSE={:?}", response);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_to_wireguard_request() {
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

        let result = parse_json_to_wireguard_request(json_str).unwrap();
        
        // Check id
        assert_eq!(result.id, 12345);
        
        // Check overlays
        assert_eq!(result.overlays.len(), 1);
        
        let overlay = &result.overlays[0];
        assert_eq!(overlay.r#type, "wireguard");
        
        // Check myself
        let myself = overlay.myself.as_ref().unwrap();
        assert_eq!(myself.wg_address, "10.0.0.2");
        assert_eq!(myself.port, 51820);
        
        // Check peer
        let peer = overlay.peer.as_ref().unwrap();
        assert_eq!(peer.endpoint_address, "192.168.1.1");
        assert_eq!(peer.endpoint_port, 51820);
        assert_eq!(peer.publickey, "pseudo-public-key");
        assert_eq!(peer.allowips.len(), 1);
        assert_eq!(peer.allowips[0], "10.0.0.0/24");
        assert_eq!(peer.persistentkeepalive, 25);
        
        println!("All JSON parsing tests passed!");
    }
}