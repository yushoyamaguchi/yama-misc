use paho_mqtt as mqtt;
use std::process;
use std::time::Duration;

fn main() {
    let host = "mqtt://localhost:1883";
    let client = mqtt::Client::new(host).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    if let Err(e) = client.connect(conn_opts) {
        println!("Unable to connect: {:?}", e);
        process::exit(1);
    }

    let topic = "test/topic";

    if let Err(e) = client.subscribe(topic, 0) {
        println!("Error subscribing to topic: {:?}", e);
        process::exit(1);
    }

    println!("Subscribed to topic: {}", topic);

    for message in client.start_consuming() {
        if let Some(msg) = message {
            println!("Received message: {:?}", msg);
        } else {
            println!("Stream ended");
            break;
        }
    }

    client.disconnect(None).unwrap();
}