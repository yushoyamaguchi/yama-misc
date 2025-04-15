use pb::ping_ponger_server::{PingPonger, PingPongerServer};
use pb::{Ping, Pong, ping::Kind};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status};
use tokio::time::sleep;
use std::time::Duration;

pub mod pb {
    tonic::include_proto!("pingpong.streaming");
}

#[derive(Debug)]
pub struct PingPongService {
    index: Arc<RwLock<u32>>,
}

#[tonic::async_trait]
impl PingPonger for PingPongService {
    type PingPongStream = ReceiverStream<Result<Pong, Status>>;

    async fn ping_pong(
        &self,
        request: Request<tonic::Streaming<Ping>>,
    ) -> Result<Response<Self::PingPongStream>, Status> {
        let mut req_stream = request.into_inner();
        let index = self.index.clone();
        let (tx, rx) = mpsc::channel(1000);

        // ping の受信をログ出力する（ただし dummy は無視）
        tokio::spawn(async move {
            while let Some(ping) = req_stream.next().await {
                let ping = ping.unwrap();
                match ping.kind {
                    Some(Kind::Dummy(d)) => {
                        println!("Got dummy ping: {}", d.message);
                    }
                    Some(Kind::TruePing(tp)) => {
                        println!("Got true ping: {}", tp.message);
                    }
                    None => {
                        println!("No kind in ping");
                    }
                }
            }
        });

        let index_clone = index.clone();
        tokio::spawn(async move {
            loop {
                let pong = {
                    let mut idx = index_clone.write().unwrap();
                    *idx += 1;
                    *idx
                };

                if tx.send(Ok(Pong { pong })).await.is_err() {
                    break;
                }

                sleep(Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:10001".parse().unwrap();
    println!("PingPongServer listening on: {}", addr);

    let ping_ponger = PingPongService {
        index: Arc::new(RwLock::from(0)),
    };

    let service = PingPongerServer::new(ping_ponger);
    tonic::transport::Server::builder()
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
