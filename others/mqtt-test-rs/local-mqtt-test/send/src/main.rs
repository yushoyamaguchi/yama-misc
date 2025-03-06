use paho_mqtt as mqtt;
use std::{process, thread, time::Duration};

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

    loop {
        let msg = mqtt::Message::new(topic, "Hello World!", 0);
        client.publish(msg).unwrap_or_else(|err| {
            println!("Error sending message: {:?}", err);
        });
        println!("Message sent");
        thread::sleep(Duration::from_secs(5));
    }
}
