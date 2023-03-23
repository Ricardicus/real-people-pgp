use crate::keys;
use keys::{hash_string, rsa_encrypt};
use std::collections::HashMap;
use std::time::Instant;

pub enum SessionState {
    Initialize,
    ChallengeCreate,
    ChallengeReply,
}

pub struct Session {
    pub state: SessionState,
    pub client_pub_key: String,
    pub valid_seconds: u64,
    pub time: Instant,
}

pub struct Challenge {
    pub pub_hash: String,
    pub session_key: String,
    pub valid_seconds: u64,
    pub time: Instant,
}

pub enum RequestData {
    Challenge(Challenge),
    Session(Session),
}

pub struct ProcessResult {
    pub msg: String,
    pub session_key: String,
    pub session_key_enc: String
}

fn generate_session_key() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let session_key_len: usize = 48;
    let mut rng = rand::thread_rng();

    (0..session_key_len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn process(
    sessions: Option<&mut HashMap<String, Session>>,
    challenges: Option<&mut HashMap<String, Challenge>>,
    state: SessionState,
    request_data: Option<RequestData>,
) -> Result<ProcessResult, &'static str> {
    match state {
        SessionState::Initialize => {
            let sessions = sessions.unwrap();
            match request_data.unwrap() {
                RequestData::Session(s) => {
                    let key = generate_session_key();
                    let key_enc = rsa_encrypt(&s.client_pub_key, &key);
                    sessions.insert(hash_string(&key_enc.to_string()), s);
                    Ok(ProcessResult {
                        session_key: key,
                        session_key_enc: key_enc,
                        msg: "Initialized a new session".to_string(),
                    })
                }
                _ => Err("Invalid arguments"),
            }
        }
        SessionState::ChallengeCreate => {
            match request_data.unwrap() {
                RequestData::Challenge(challenge) => {
                    challenges
                        .unwrap()
                        .insert(challenge.pub_hash.to_string(), challenge);
                    return Ok(ProcessResult {
                        session_key: "".to_string(),     // todo
                        session_key_enc: "".to_string(), // not yet implemeted
                        msg: "Challenge created".to_string(),
                    });
                }
                _other => return Err("Failed to create challenge"),
            };
        }
        SessionState::ChallengeReply => Err("Not implemented yet"),
    }
}
