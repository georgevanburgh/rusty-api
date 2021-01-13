use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Copy, Clone)]
pub struct Session {}

pub struct SessionStore {
    session_cache: HashMap<String, Session>,
}

pub type SharedSessionStore = Arc<Mutex<SessionStore>>;

impl Session {
    pub fn new() -> Session {
        Session {
        }
    }
}

impl SessionStore {
    pub fn new() -> SharedSessionStore {
        println!("SessionStore");
        Arc::new(Mutex::new(SessionStore {
            session_cache: HashMap::new(),
        }))
    }
}

pub fn new_session_store() -> SharedSessionStore {
    SessionStore::new()
}

pub struct SafeSessionStore {
    acc: SharedSessionStore,
}

impl SafeSessionStore {
    pub fn new(a: SharedSessionStore) -> SafeSessionStore {
        SafeSessionStore { acc: a }
    }

    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        self.acc
            .lock()
            .unwrap()
            .session_cache
            .get(session_id)
            .copied()
    }

    pub fn create_session(&mut self, s: Session) -> String {
        let new_session_id = format!("{}", Uuid::new_v4());
        self.acc
            .lock()
            .unwrap()
            .session_cache
            .insert(new_session_id.clone(), s);
        new_session_id
    }
}
