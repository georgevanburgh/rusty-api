use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Session {
    token: String,
    refresh_token: Option<String>,
    email: String
}

#[derive(Clone)]
pub struct SessionStore {
    session_cache: Arc<Mutex<HashMap<String, Session>>>,
}

impl Session {
    pub fn new(token: String, refresh_token: Option<String>, email: String) -> Session {
        Session {
            token: token,
            refresh_token: refresh_token,
            email: email,
        }
    }
}

impl SessionStore {
    pub fn new() -> SessionStore {
        SessionStore {
            session_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        self.session_cache.lock().unwrap().get(session_id).cloned()
    }

    pub fn create_session(&self, s: Session) -> String {
        let new_session_id = format!("{}", Uuid::new_v4());
        self.session_cache
            .lock()
            .unwrap()
            .insert(new_session_id.clone(), s);
        new_session_id
    }
}
