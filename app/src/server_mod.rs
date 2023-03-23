use crate::keys;
use std::time::Instant;

use std::collections::HashMap;

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
                    let key = s.client_pub_key.to_string();
                    if sessions.contains_key(&key.to_string()) {
                        Ok(ProcessResult {
                            session_key: key,
                            msg: "Session has already been initialized".to_string(),
                        })
                    } else {
                        sessions.insert(key.to_string(), s);
                        Ok(ProcessResult {
                            session_key: key,
                            msg: "Initialized a new session".to_string(),
                        })
                    }
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
                        session_key: "".to_string(),
                        msg: "Challenge created".to_string(),
                    });
                }
                _other => return Err("Failed to create challenge"),
            };
        }
        SessionState::ChallengeReply => Err("Not implemented yet"),
    }
}
