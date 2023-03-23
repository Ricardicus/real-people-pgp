mod keys;
use keys::{hash_string, rsa_decrypt, rsa_encrypt, Database, KeyMaster, RootCerts};

use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
// importing generated gRPC code
use poh_grpc::*;
// importing types for messages
use poh::*;
mod poh;
mod poh_grpc;

use chrono::offset::Local;
use std::collections::HashMap;

mod server_mod;
use server_mod::{process, Challenge, ProcessResult, RequestData, Session, SessionState};

#[allow(dead_code)]

impl Session {
    pub fn new(public_key: &str) -> Self {
        Session {
            state: SessionState::Initialize,
            valid_seconds: 60 * 5, // By default, 5 minutes
            time: Instant::now(),
            client_pub_key: public_key.to_string(),
        }
    }
}

struct MyPoH {
    rootcerts: RootCerts,
    keymaster: KeyMaster,
    database: Database,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    challenges: Arc<Mutex<HashMap<String, Challenge>>>,
}

impl PoH for MyPoH {
    // rpc for service
    fn initialize(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<InitializeRequest>,
        resp: ServerResponseUnarySink<InitializeResponse>,
    ) -> grpc::Result<()> {
        // create Response
        let mut r = InitializeResponse::new();
        let cert = req.message.get_cert();
        let pub_key = req.message.get_pub_key();
        let mut valid: bool = false;
        let mut cert_issuer: String = String::new();

        for rootcert in &self.rootcerts.certs {
            if self
                .keymaster
                .verify_with_public_key(&rootcert.public_key, &pub_key, cert)
            {
                valid = true;
                cert_issuer = rootcert.issuer.clone();
            } else {
                println!(
                    "rootcert public key: {}, client pub_key: {}, client cert: {}",
                    rootcert.public_key, pub_key, cert
                );
            }
        }

        let pub_key_hash = hash_string(&pub_key);
        if self.database.entries.contains_key(&pub_key_hash) {
            // Verify that the issuer from certificate matches the database
            if !cert_issuer.eq(&self.database.entries[&pub_key_hash].issuer) {
                valid = false;
            }
        } else {
            valid = false;
        }

        println!("Initiailization attempt valid: {}", valid);
        r.set_valid(valid);
        r.set_msg("Checked client validity".to_string());

        if valid {
            let access = self.sessions.clone();
            let map: &mut HashMap<String, Session> = &mut access.lock().unwrap();
            let session = Session::new(pub_key);
            let data: RequestData = RequestData::Session(session);
            let res: ProcessResult =
                process(Some(map), None, SessionState::Initialize, Some(data)).unwrap();
            r.set_msg(res.msg);
            r.set_session_key_enc(res.session_key_enc.to_string());
            println!(
                "Created session {}, {}",
                res.session_key,
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
            );
        }

        // sent the response
        resp.finish(r)
    }

    fn challenge_create(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<ChallengeCreateRequest>,
        resp: ServerResponseUnarySink<ChallengeCreateResponse>,
    ) -> grpc::Result<()> {
        // create Response
        let mut r = ChallengeCreateResponse::new();
        let _session_key = req.message.get_session_key();
        r.set_valid(false);
        r.set_info("not implemented yet".to_string());
        resp.finish(r)
    }

    fn challenge_reply(
        &self,
        _: ServerHandlerContext,
        _req: ServerRequestSingle<ChallengeReplyRequest>,
        resp: ServerResponseUnarySink<ChallengeReplyResponse>,
    ) -> grpc::Result<()> {
        // create Response
        let r = ChallengeReplyResponse::new();
        resp.finish(r)
    }
}

fn session_cleanup(sessions: &mut HashMap<String, Session>) {
    let mut remove: Vec<String> = Vec::<String>::new();
    for (key, session) in &mut *sessions {
        let age: Duration = session.time.elapsed();
        let seconds = session.valid_seconds;
        if age.as_secs() > seconds {
            remove.push(key.to_string());
        }
    }
    for key in remove {
        println!("Removing session {}", key);
        sessions.remove(&key.to_string());
    }
}

fn challenge_cleanup(challenges: &mut HashMap<String, Challenge>) {
    let mut remove: Vec<String> = Vec::<String>::new();
    for (key, challenge) in &mut *challenges {
        let age: Duration = challenge.time.elapsed();
        let seconds = challenge.valid_seconds;
        if age.as_secs() > seconds {
            remove.push(key.to_string());
        }
    }
    for key in remove {
        println!("Removing session {}", key);
        challenges.remove(&key.to_string());
    }
}

fn launch_cleanup(
    session_data: Arc<Mutex<HashMap<String, Session>>>,
    challenge_data: Arc<Mutex<HashMap<String, Challenge>>>,
) {
    let cleanup_time_secs = 30;
    let session_data = session_data.clone();
    thread::Builder::new()
        .name("cleanup sessions".to_string())
        .spawn(move || loop {
            {
                let mut data = session_data.lock().unwrap();
                if data.len() > 0 {
                    session_cleanup(&mut data);
                }
            }
            thread::sleep(Duration::from_secs(cleanup_time_secs));
        })
        .expect("Failed to create thread");
    let challenge_data = challenge_data.clone();
    thread::Builder::new()
        .name("cleanup challenges".to_string())
        .spawn(move || loop {
            {
                let mut data = challenge_data.lock().unwrap();
                if data.len() > 0 {
                    challenge_cleanup(&mut data);
                }
            }
            thread::sleep(Duration::from_secs(cleanup_time_secs));
        })
        .expect("Failed to create thread");
}

fn main() {
    let port = 50051;
    // creating server
    let mut server = grpc::ServerBuilder::new_plain();
    let session_data: Arc<Mutex<HashMap<String, Session>>> =
        Arc::new(HashMap::<String, Session>::new().into());
    let challenge_data: Arc<Mutex<HashMap<String, Challenge>>> =
        Arc::new(HashMap::<String, Challenge>::new().into());
    // adding port to server for http
    server.http.set_port(port);
    // adding say service to server
    //
    server.add_service(PoHServer::new_service_def(MyPoH {
        rootcerts: RootCerts::read_rootcert(),
        keymaster: KeyMaster::new(None),
        database: Database::from_file(Database::get_std_db()),
        sessions: session_data.clone(),
        challenges: challenge_data.clone(),
    }));
    // running the server
    let _server = server.build().expect("server");
    println!(
        "proof of human server started on port {}, {}",
        port,
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );
    // starting session cleanup thread
    launch_cleanup(session_data.clone(), challenge_data.clone());

    // stopping the program from finishing
    loop {
        std::thread::park();
    }
}
