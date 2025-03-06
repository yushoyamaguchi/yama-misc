use paho_mqtt as mqtt;
use std::process;
use std::time::Duration;

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

    let topic = "test/topic";
    if let Err(e) = client.subscribe(topic, 0) {
        println!("Error subscribing to topic: {:?}", e);
        process::exit(1);
    }
    println!("Subscribed to topic: {}", topic);

    for message in client.start_consuming() {
        if let Some(msg) = message {
            println!("Received secure message: {:?}", msg);
        } else {
            println!("Stream ended");
            break;
        }
    }
    client.disconnect(None).unwrap();
}