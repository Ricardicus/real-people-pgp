// importing generated gRPC code
use poh_grpc::*;
// importing types for messages
use poh::*;
mod poh;
mod poh_grpc;
extern crate serde;
extern crate serde_derive;

mod keys;
use keys::{RootCerts, Cert, KeyMaster};
use clap::Parser;
use rpassword::read_password;
use std::io::Write;
use grpc::ClientStubExt;

#[derive(Parser)]
struct Cli {
    /// The path to the file to store
    cert: String,
    keys: String
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "rickard";
    let port = 50051;
    let root_cert: RootCerts = RootCerts::read_rootcert();
    let args = Cli::parse();

    print!("Enter the passphrase for your keys: ");
    std::io::stdout().flush().unwrap();
    let passphrase = read_password().unwrap();
    
    let km: KeyMaster = KeyMaster::import_from_file(&args.keys, passphrase.as_str());
    let cert: Cert = Cert::from_file(&args.cert);

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
