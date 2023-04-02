// importing generated gRPC code
use poh_grpc::*;
// importing types for messages
use poh::*;
mod poh;
mod poh_grpc;

use clap::Parser;
use grpc::ClientStubExt;

#[derive(Parser)]
struct Cli {
    /// host address
    host: String,
    /// host port
    port: u16,
    /// signature to check
    signature: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let host = args.host;
    let signature = args.signature;
    let port = args.port;

    let mut req = VerifyAttachedSignatureRequest::new();
    let signature = std::fs::read_to_string(signature.clone()).expect("Unable to read file");

    let client = PoHClient::new_plain(&host, port, Default::default()).unwrap();
    req.set_file_attached_signature(signature);

    // send the request
    let resp = client
        .verify_attached_signature(grpc::RequestOptions::new(), req)
        .join_metadata_result()
        .await?;
    println!("Valid: {}, info: {}", resp.1.valid, resp.1.info);
    Ok(())
}
