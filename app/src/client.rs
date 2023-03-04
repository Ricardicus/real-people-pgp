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
}

struct Client {
    keymaster: KeyMaster,
    cert: Cert,
    client: PoHClient,
}

enum Response {
    InitializeResponse(poh::InitializeResponse),
}

struct Session {
    session_key: String,
    keys: KeyMaster,
    responses: Vec<Response>,
}

impl Client {
    pub fn new(host: &str, port: u16, keysfile: &str, certfile: &str, passphrase: &str) -> Self {
        Self {
            keymaster: KeyMaster::import_from_file(keysfile, passphrase),
            cert: Cert::from_file(certfile),
            client: PoHClient::new_plain(host, port, Default::default()).unwrap(),
        }
    }

    pub async fn initialize_session(&self) -> Result<Session, Box<dyn std::error::Error>> {
        let keys = KeyMaster::new(None);
        let mut req = InitializeRequest::new();

        req.set_msg(keys.public_key.to_string());
        req.set_pub_key(self.keymaster.public_key.to_string());
        req.set_cert(self.cert.signature.to_string());
        req.set_msg_sig(self.keymaster.sign(hash_string(keys.public_key.as_str())));
        // send the request
        let resp = self
            .client
            .initialize(grpc::RequestOptions::new(), req)
            .join_metadata_result()
            .await?;

        let mut session = Session {
            session_key: resp.1.session_key.to_string(),
            responses: Vec::<Response>::new(),
            keys: keys,
        };
        session.responses.push(Response::InitializeResponse(resp.1));
        Ok(session)
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

    let response: Session = client.initialize_session().await?;

    for resp in response.responses {
        match resp {
            Response::InitializeResponse(InitializeResponse {
                msg,
                valid,
                session_key,
                ..
            }) => {
                println!("{:?}: valid: {}, session key: {}", msg, valid, session_key);
            }
        }
    }
    Ok(())
}
