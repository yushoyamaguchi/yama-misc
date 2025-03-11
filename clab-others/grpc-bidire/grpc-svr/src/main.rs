use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{MyMessage, HelloRequest, HelloResponse, SecondRequest, SecondResponse};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    type ChatStream = ReceiverStream<Result<MyMessage, Status>>;

    async fn chat(
        &self,
        request: Request<tonic::Streaming<MyMessage>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        let mut inbound_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(10);

        tokio::spawn(async move {
            // ストリームから順番にメッセージを取り出す
            while let Some(Ok(in_msg)) = inbound_stream.next().await {
                if let Some(my_message) = in_msg.msg {
                    match my_message {
                        // HelloRequest の場合
                        hello_world::my_message::Msg::HelloRequest(HelloRequest { name }) => {
                            println!("Received HelloRequest: {}", name);
                            let res = MyMessage {
                                msg: Some(hello_world::my_message::Msg::HelloResponse(
                                    HelloResponse {
                                        message: format!("Hello, {}!", name),
                                    },
                                )),
                            };
                            // メッセージを返す
                            if tx.send(Ok(res)).await.is_err() {
                                // クライアントとの接続が切れているなど
                                break;
                            }
                        }

                        // SecondRequest の場合
                        hello_world::my_message::Msg::SecondRequest(SecondRequest { payload }) => {
                            println!("Received SecondRequest: {}", payload);
                            let res = MyMessage {
                                msg: Some(hello_world::my_message::Msg::SecondResponse(
                                    SecondResponse {
                                        message: format!("Final response for: {}", payload),
                                    },
                                )),
                            };
                            if tx.send(Ok(res)).await.is_err() {
                                break;
                            }
                        }

                        // 予期しないメッセージ
                        _ => {
                            eprintln!("Unexpected message");
                        }
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
