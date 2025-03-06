use wireguard1::greeter_client::GreeterClient;
use wireguard1::HelloRequest;

pub mod wireguard1 {
    tonic::include_proto!("wireguard1");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a channel to the server
    let channel = tonic::transport::Channel::from_static("http://192.168.1.2:50051")
        .connect()
        .await?;

    // Create the client
    let mut client = GreeterClient::new(channel);

    // Create a request
    let request = tonic::Request::new(HelloRequest {
        name: "Yusho".into(),
    });

    // Send the request and await the response
    let response = client.say_hello(request).await?;

    // Print the response
    println!("RESPONSE={:?}", response);

    Ok(())
}