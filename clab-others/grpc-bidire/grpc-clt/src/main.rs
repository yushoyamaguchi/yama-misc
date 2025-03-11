use tonic::transport::Channel;
use tonic::Request;
use tokio_stream::StreamExt;
use tokio::sync::mpsc;
use hello_world::greeter_client::GreeterClient;
use hello_world::{MyMessage, HelloRequest, SecondRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://192.168.1.2:50051")
        .connect()
        .await?;

    let mut client = GreeterClient::new(channel);

    // 双方向ストリーム呼び出し（送信用チャネルと受信用ストリーム）
    let (tx, rx) = mpsc::channel(10);
    let mut response_stream = client
        .chat(Request::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
        .await?
        .into_inner();

    //
    // 1st request: HelloRequest
    //
    let hello_request = MyMessage {
        msg: Some(hello_world::my_message::Msg::HelloRequest(HelloRequest {
            name: "Yusho".into(),
        })),
    };

    // 送信
    tx.send(hello_request).await.unwrap();

    // サーバからの HelloResponse を受信
    if let Some(Ok(res_msg)) = response_stream.next().await {
        if let Some(inner_msg) = res_msg.msg {
            match inner_msg {
                hello_world::my_message::Msg::HelloResponse(r) => {
                    println!("Received HelloResponse: {:?}", r.message);

                    //
                    // 2nd request: SecondRequest
                    //
                    let second_request = MyMessage {
                        msg: Some(hello_world::my_message::Msg::SecondRequest(SecondRequest {
                            payload: format!("Acknowledged: {}", r.message),
                        })),
                    };

                    // 送信
                    tx.send(second_request).await.unwrap();
                }
                _ => {
                    eprintln!("Unexpected response (expected HelloResponse).");
                }
            }
        }
    }

    // サーバからの SecondResponse を受信
    if let Some(Ok(res_msg)) = response_stream.next().await {
        if let Some(inner_msg) = res_msg.msg {
            match inner_msg {
                hello_world::my_message::Msg::SecondResponse(r) => {
                    println!("Received SecondResponse: {:?}", r.message);
                }
                _ => {
                    eprintln!("Unexpected response (expected SecondResponse).");
                }
            }
        }
    }

    Ok(())
}
