use tonic::{transport::Server, Request, Response, Status};

use wireguard1::wire_guard_service_server::{WireGuardService, WireGuardServiceServer};
use wireguard1::{HelloReply, HelloRequest, WireGuardRequest, WireGuardResponse};

pub mod wireguard1 {
    tonic::include_proto!("wireguard1");
}

#[derive(Debug, Default)]
pub struct MyWireGuardService {}

#[tonic::async_trait]
impl WireGuardService for MyWireGuardService {
    async fn configure_wire_guard(
        &self,
        request: Request<WireGuardRequest>,
    ) -> Result<Response<WireGuardResponse>, Status> {
        let req = request.into_inner();
        
        println!("Received WireGuard configuration request with ID: {}", req.id);
        
        // Perform ID check
        if req.id <= 0 {
            println!("Invalid ID: {}", req.id);
            return Ok(Response::new(WireGuardResponse {
                success: false,
                message: "Invalid ID provided".to_string(),
            }));
        }
        
        // Log the received overlays
        for (i, overlay) in req.overlays.iter().enumerate() {
            println!("Overlay {}: type={}", i, overlay.r#type);
            
            if let Some(myself) = &overlay.myself {
                println!("  Myself: wg_address={}, port={}", myself.wg_address, myself.port);
            }
            
            if let Some(peer) = &overlay.peer {
                println!("  Peer: endpoint_address={}, endpoint_port={}", 
                    peer.endpoint_address, peer.endpoint_port);
                println!("    publickey={}", peer.publickey);
                println!("    allowips={:?}", peer.allowips);
                println!("    persistentkeepalive={}", peer.persistentkeepalive);
            }
        }
        
        // Return success response
        Ok(Response::new(WireGuardResponse {
            success: true,
            message: format!("Successfully configured WireGuard with ID: {}", req.id),
        }))
    }
    
    // Implementing the SayHello method to maintain compatibility
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a hello request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "192.168.1.2:50051".parse()?;
    let wire_guard_service = MyWireGuardService::default();

    println!("WireGuard gRPC server starting on {}", addr);

    Server::builder()
        .add_service(WireGuardServiceServer::new(wire_guard_service))
        .serve(addr)
        .await?;

    Ok(())
}