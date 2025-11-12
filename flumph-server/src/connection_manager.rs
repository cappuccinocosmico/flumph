use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<String, mpsc::Sender<Vec<u8>>>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_connection(&self, id: String, sender: mpsc::Sender<Vec<u8>>) {
        self.connections.write().unwrap().insert(id, sender);
    }

    pub fn remove_connection(&self, id: &str) {
        self.connections.write().unwrap().remove(id);
    }

    pub fn get_connection(&self, id: &str) -> Option<mpsc::Sender<Vec<u8>>> {
        self.connections.read().unwrap().get(id).cloned()
    }
}
