use crate::keys;
use std::time::Instant;

pub enum SessionState {
    Initialize,
    ChallengeCreate,
    ChallengeReply,
}

pub struct Session {
    pub state: SessionState,
    pub keys: keys::KeyMaster,
    pub key_initialized: String,
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
