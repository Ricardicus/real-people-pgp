// importing generated gRPC code
use poh_grpc::*;
// importing types for messages
use poh::*;
mod poh;
mod poh_grpc;

mod keys;

use grpc::ClientStubExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "rickard";
    let port = 50051;
    let client_conf = Default::default();
    // create a client
    let client = PoHClient::new_plain("::1", port, client_conf).unwrap();
    // create request
    let mut req = CheckRequest::new();
    req.set_msg(name.to_string());
    // send the request
    let resp = client
        .check(grpc::RequestOptions::new(), req)
        .join_metadata_result()
        .await?;
    // wait for response
    println!("{:?}", resp);
    Ok(())
}
