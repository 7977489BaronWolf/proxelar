use std::sync::{Arc, Mutex};
use crate::logger::entry::LogEntry;

#[derive(Debug, Clone)]
pub struct LogStore {
    entries: Arc<Mutex<Vec<LogEntry>>>,
    next_id: Arc<Mutex<u64>>,
}

impl LogStore {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn next_id(&self) -> u64 {
        let mut id = self.next_id.lock().unwrap();
        let current = *id;
        *id += 1;
        current
    }

    pub fn insert(&self, entry: LogEntry) {
        let mut entries = self.entries.lock().unwrap();
        entries.push(entry);
    }

    pub fn update<F>(&self, id: u64, f: F)
    where
        F: FnOnce(&mut LogEntry),
    {
        let mut entries = self.entries.lock().unwrap();
        if let Some(entry) = entries.iter_mut().find(|e| e.id == id) {
            f(entry);
        }
    }

    pub fn get_all(&self) -> Vec<LogEntry> {
        self.entries.lock().unwrap().clone()
    }

    pub fn get_by_id(&self, id: u64) -> Option<LogEntry> {
        self.entries.lock().unwrap().iter().find(|e| e.id == id).cloned()
    }

    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }

    pub fn len(&self) -> usize {
        self.entries.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for LogStore {
    fn default() -> Self {
        Self::new()
    }
}
