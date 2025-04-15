use std::f32::consts::PI;
use std::pin::Pin;
use std::time::Duration;

use pb::ping_ponger_client::PingPongerClient;
use pb::Ping;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::transport::Channel;
use tonic::Request;

pub mod pb {
    tonic::include_proto!("pingpong.streaming");
}

async fn play_ping_pong(client: &mut PingPongerClient<Channel>) {
    let (tx, rx) = mpsc::channel(10000);
    let ack = ReceiverStream::new(rx);
    let response = client.ping_pong(Request::new(ack)).await.unwrap();

    // kick start: 最初のダミー ping
    tx.send(Ping { message: "init".into() }).await.unwrap();

    let mut pong_stream = response.into_inner();
    while let Some(pong) = pong_stream.next().await {
        let pong = pong.unwrap();
        println!("last seen pong from server: {}", pong.pong);
        let message = format!("pongを受信したので返信: {}", pong.pong);
        tx.send(Ping { message }).await.unwrap();
    }
}


#[tokio::main]
async fn main() {
    let mut client = PingPongerClient::connect("http://192.168.1.2:10001")
        .await
        .unwrap();
    play_ping_pong(&mut client).await;
}