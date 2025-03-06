use paho_mqtt as mqtt;
use std::process;
use std::time::Duration;
use serde_json::{Value, Error as JsonError};

fn main() {
    let host = "ssl://localhost:8883";
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("rust_subscriber")
        .finalize();

    let client = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store("../cert/ca.crt").unwrap()
        .enable_server_cert_auth(false)  // サーバー証明書の認証を無効化（テスト用）
        .verify(false)  // 証明書の検証を無効化（テスト用）
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .ssl_options(ssl_opts)
        .finalize();

    println!("Attempting to connect...");
    match client.connect(conn_opts) {
        Ok(_) => println!("Connection successful"),
        Err(e) => {
            println!("Unable to connect: {:?}", e);
            println!("Detailed error: {}", e.to_string());
            process::exit(1);
        }
    }

    let topic = "test/topic_json1";
    if let Err(e) = client.subscribe(topic, 0) {
        println!("Error subscribing to topic: {:?}", e);
        process::exit(1);
    }
    println!("Subscribed to topic: {}", topic);

    for message in client.start_consuming() {
        if let Some(msg) = message {
            // メッセージのペイロードを取得してString型に変換
            let payload = match String::from_utf8(msg.payload().to_vec()) {
                Ok(s) => s,
                Err(e) => {
                    println!("Error converting payload to string: {:?}", e);
                    continue;
                }
            };
    
            // JSONをパース
            match serde_json::from_str::<Value>(&payload) {
                Ok(json_data) => {
                    // ip_addrとportの値を取り出す
                    let ip = json_data.get("ip_addr").and_then(Value::as_str);
                    let port = json_data.get("port").and_then(Value::as_number).map(|n| n.as_u64()).flatten();
    
                    match (ip, port) {
                        (Some(ip_val), Some(port_val)) => {
                            println!("Received IP Address: {}", ip_val);
                            println!("Received Port: {}", port_val);
                        },
                        _ => {
                            println!("Missing or invalid fields in JSON");
                        }
                    }
                }
                Err(e) => {
                    println!("Error parsing JSON: {:?}", e);
                }
            }
        } else {
            println!("Stream ended");
            break;
        }
    }
        
    client.disconnect(None).unwrap();
}