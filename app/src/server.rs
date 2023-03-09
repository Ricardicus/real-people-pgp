mod keys;
use keys::{hash_string, secp256k1_decrypt, Database, KeyMaster, RootCerts};

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
use server_mod::{Challenge, RequestData, Session, SessionState};

#[allow(dead_code)]

impl Session {
    pub fn new(public_key: &str) -> Self {
        Session {
            keys: KeyMaster::new(None),
            state: SessionState::Initialize,
            valid_seconds: 60 * 5, // By default, 5 minutes
            time: Instant::now(),
            key_initialized: public_key.to_string(),
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

struct ProcessResult {
    msg: String,
    session_key: String,
}

fn process(
    sessions: Option<&mut HashMap<String, Session>>,
    challenges: Option<&mut HashMap<String, Challenge>>,
    public_key: &str,
    state: SessionState,
    request_data: Option<RequestData>,
) -> Result<ProcessResult, &'static str> {
    match state {
        SessionState::Initialize => {
            let sessions = sessions.unwrap();
            let s = Session::new(public_key);
            let key = s.keys.public_key.to_string();
            sessions.insert(key.to_string(), s);
            Ok(ProcessResult {
                session_key: key,
                msg: "Initialized a new session".to_string(),
            })
        }
        SessionState::ChallengeCreate => {
            let challenges = challenges.unwrap();
            let public_key_hash = public_key; // It is given as a hash
            if challenges.contains_key(public_key_hash) {
                Ok(ProcessResult {
                    session_key: "".to_string(),
                    msg: "Challenge already created".to_string(),
                })
            } else {
                match request_data.unwrap() {
                    RequestData::Challenge(challenge) => {
                        challenges.insert(public_key_hash.to_string(), challenge);
                        return Ok(ProcessResult {
                            session_key: "".to_string(),
                            msg: "Challenge created".to_string(),
                        });
                    }
                    _other => return Err("Failed to create challenge"),
                };
            }
        }
        SessionState::ChallengeReply => Err("Not implemented yet"),
    }
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
        let msg = req.message.get_msg();
        let msg_signature = req.message.get_msg_sig();
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
            }
        }

        if valid {
            valid =
                self.keymaster
                    .verify_with_public_key(&pub_key, &hash_string(msg), msg_signature);
            if !valid {
                println!("Message signature didn't match");
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

        println!("Received message {}, valid: {}", msg, valid);
        r.set_valid(valid);
        r.set_msg("Checked client validity".to_string());

        if valid {
            let access = self.sessions.clone();
            let map: &mut HashMap<String, Session> = &mut access.lock().unwrap();
            let res: ProcessResult =
                process(Some(map), None, &pub_key, SessionState::Initialize, None).unwrap();
            r.set_session_key(res.session_key.to_string());
            r.set_msg(res.msg);
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
        let session_key = req.message.get_session_key();
        r.set_valid(false);

        let access = self.sessions.clone();
        let map: &mut HashMap<String, Session> = &mut access.lock().unwrap();

        if map.contains_key(session_key) {
            let session = map.get(session_key).unwrap();
            let access = self.challenges.clone();
            let challenges: &mut HashMap<String, Challenge> = &mut access.lock().unwrap();
            let pub_hash_enc = req.message.get_pub_hash_enc();
            let pub_hash_key =
                secp256k1_decrypt(&session.keys.secret_key.to_string(), pub_hash_enc);
            if pub_hash_key.is_ok() {
                let pub_hash_key = pub_hash_key.unwrap();
                let data: RequestData = RequestData::Challenge(Challenge {
                    pub_hash: pub_hash_key.to_string(),
                    session_key: session_key.to_string(),
                    valid_seconds: req.message.get_valid_time_sec(),
                    time: Instant::now(),
                });
                if process(
                    Some(map),
                    Some(challenges),
                    &pub_hash_key.as_ref(),
                    SessionState::ChallengeCreate,
                    Some(data),
                )
                .is_ok()
                {
                    r.set_valid(true);
                    r.set_info("challenge created".to_string());
                    println!(
                        "Challenge created {} {}",
                        pub_hash_key,
                        Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    );
                } else {
                    println!("Failed attempt at creating a challenge");
                    r.set_info("Failed to create challenge".to_string());
                }
            } else {
                println!("Decryption failed..");
            }
        } else {
            println!("Invalid session key {}", session_key);
        }
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
