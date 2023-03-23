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
use keys::{hash_string, rsa_decrypt, rsa_encrypt, Cert, KeyMaster};
use rpassword::read_password;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    /// Your client certificate
    cert: String,
    /// The keys
    keys: String,
    // What to do [challenge, init]
    command: String,
}

struct Client {
    keymaster: KeyMaster,
    cert: Cert,
    client: PoHClient,
}
#[derive(Clone)]
enum Response {
    InitializeResponse(poh::InitializeResponse),
}

#[derive(Clone)]
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

        req.set_pub_key(self.keymaster.public_key.to_string());
        req.set_cert(self.cert.signature.to_string());
        // send the request
        let resp = self
            .client
            .initialize(grpc::RequestOptions::new(), req)
            .join_metadata_result()
            .await?;

        let session_key = &resp.1.session_key.to_string();
        let mut session = Session {
            session_key: session_key.to_string(),
            responses: Vec::<Response>::new(),
            keys: keys,
        };
        session.responses.push(Response::InitializeResponse(resp.1));
        Ok(session)
    }

    pub async fn send_challenge(
        &self,
        _session: &Session,
        _who: &str,
    ) -> Result<&'static str, Box<dyn std::error::Error>> {
        Err(Into::into("Not implemented yet"))
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
    let r = response.clone();
    for resp in response.responses {
        match resp {
            Response::InitializeResponse(InitializeResponse {
                msg,
                valid,
                session_key,
                ..
            }) => {
                let session_key = response.session_key.clone();
                println!("{:?}: valid: {}, session key: {}", msg, valid, session_key);
                if args.command == "challenge" {
                    let made_up_id = hash_string("made up id");
                    println!("Challenging fake ID: {}", made_up_id);
                    let res = client.send_challenge(&r, &made_up_id).await?;
                    println!("challenge sent response: {}", res);
                } else if args.command == "init" {
                    println!("Initialization complete");
                } else {
                    println!("In this early stage, I have not support for {}\nTry 'challenge' (all I know).", args.command);
                }
            }
        }
    }
    Ok(())
}
