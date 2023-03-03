mod keys;
use keys::{hash_string, secp256k1_decrypt, secp256k1_encrypt, Database, KeyMaster, RootCerts};

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

use std::collections::HashMap;

enum SessionState {
    Initialized,
    Verify,
    Sign,
}

struct Session {
    state: SessionState,
    keys: KeyMaster,
    time: Instant,
}

impl Session {
    pub fn new() -> Self {
        Session {
            keys: KeyMaster::new(None),
            state: SessionState::Initialized,
            time: Instant::now(),
        }
    }
}

struct MyPoH {
    rootcerts: RootCerts,
    keymaster: KeyMaster,
    database: Database,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
}

struct ProcessResult {
    msg: String,
    session_key: String,
}

fn process(sessions: &mut HashMap<String, Session>, public_key: &str) -> ProcessResult {
    if sessions.contains_key(public_key) {
        let s: &Session = sessions.get(public_key).unwrap();
        ProcessResult {
            session_key: s.keys.public_key.to_string(),
            msg: "Session already initialized".to_string(),
        }
    } else {
        let s = Session::new();
        let key = s.keys.public_key.to_string();
        sessions.insert(public_key.to_string(), s);
        ProcessResult {
            session_key: key,
            msg: "Initialized a new session".to_string(),
        }
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
            println!(
                "pub key: {}, cert: {}, issuer: {}",
                pub_key, cert, cert_issuer
            );
        }

        if valid {
            valid =
                self.keymaster
                    .verify_with_public_key(&pub_key, &hash_string(msg), msg_signature);
            if !valid {
                println!("Message signature didn't match");
            }
        }

        if self.database.entries.contains_key(pub_key) {
            // Verify that the issuer from certificate matches the database
            if !cert_issuer.eq(&self.database.entries[pub_key].issuer) {
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
            let res: ProcessResult = process(map, &pub_key);
            r.set_session_key(res.session_key);
            r.set_msg(res.msg);
        }

        // sent the response
        resp.finish(r)
    }
}

fn session_cleanup(sessions: &mut HashMap<String, Session>, seconds: u64) {
    let mut remove: Vec<String> = Vec::<String>::new();
    for (key, session) in &mut *sessions {
        let age: Duration = session.time.elapsed();
        if age.as_secs() > seconds {
            remove.push(key.to_string());
        }
    }
    for key in remove {
        println!("Removing session {}", key);
        sessions.remove(&key.to_string());
    }
}

fn launch_cleanup(server_data: Arc<Mutex<HashMap<String, Session>>>) {
    let cleanup_time_secs = 30;
    let data = server_data.clone();
    thread::Builder::new()
        .name("cleanup".to_string())
        .spawn(move || {
            while true {
                {
                    let mut data = data.lock().unwrap();
                    session_cleanup(&mut data, cleanup_time_secs * 3);
                }
                thread::sleep(Duration::from_secs(cleanup_time_secs));
            }
        })
        .expect("Failed to create thread");
}

fn main() {
    let port = 50051;
    // creating server
    let mut server = grpc::ServerBuilder::new_plain();
    let server_data: Arc<Mutex<HashMap<String, Session>>> =
        Arc::new(HashMap::<String, Session>::new().into());
    // adding port to server for http
    server.http.set_port(port);
    // adding say service to server
    server.add_service(PoHServer::new_service_def(MyPoH {
        rootcerts: RootCerts::read_rootcert(),
        keymaster: KeyMaster::new(None),
        database: Database::from_file(Database::get_std_db()),
        sessions: server_data.clone(),
    }));
    // running the server
    let _server = server.build().expect("server");
    println!("greeter server started on port {}", port,);
    // starting session cleanup thread
    launch_cleanup(server_data.clone());

    // stopping the program from finishing
    loop {
        std::thread::park();
    }
}
