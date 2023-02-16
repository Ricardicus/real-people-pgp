// importing generated gRPC code
use poh_grpc::*;
// importing types for messages
use poh::*;
mod poh;
mod poh_grpc;
extern crate serde;
extern crate serde_derive;

mod keys;
use clap::Parser;
use grpc::ClientStubExt;
use keys::{hash_string, Cert, KeyMaster};
use rpassword::read_password;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    /// The path to the file to store
    cert: String,
    keys: String,
    message: String,
}

struct Client {
    keymaster: KeyMaster,
    cert: Cert,
    client: PoHClient,
}

impl Client {
    pub fn new(host: &str, port: u16, keysfile: &str, certfile: &str, passphrase: &str) -> Self {
        Self {
            keymaster: KeyMaster::import_from_file(keysfile, passphrase),
            cert: Cert::from_file(certfile),
            client: PoHClient::new_plain(host, port, Default::default()).unwrap(),
        }
    }

    pub async fn send_msg(&self, msg: &str) -> Result<CheckResponse, Box<dyn std::error::Error>> {
        let mut req = CheckRequest::new();
        req.set_msg(msg.to_string());
        req.set_pub_key(self.keymaster.public_key.to_string());
        req.set_cert(self.cert.signature.to_string());
        req.set_msg_sig(self.keymaster.sign(hash_string(msg)));
        // send the request
        let resp = self
            .client
            .check(grpc::RequestOptions::new(), req)
            .join_metadata_result()
            .await?;
        Ok(resp.1)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 50051;
    let host: &str = "::1";
    let args = Cli::parse();

    print!("Enter the passphrase for your keys: ");
    std::io::stdout().flush().unwrap();
    let passphrase = read_password().unwrap();

    let client: Client = Client::new(host, port, &args.keys, &args.cert, passphrase.as_str());

    let response: CheckResponse = client.send_msg(&args.message).await?;
    // wait for response
    println!("{:?}: valid: {}", response.msg, response.valid);
    Ok(())
}
