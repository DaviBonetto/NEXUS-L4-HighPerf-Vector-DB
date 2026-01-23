//! In-Memory Vector Storage with Mutex protection

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorRecord {
    pub id: String,
    pub embedding: Vec<f32>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

pub struct VectorStore {
    data: RwLock<HashMap<String, VectorRecord>>,
}

impl VectorStore {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert(&self, record: VectorRecord) {
        let mut data = self.data.write();
        data.insert(record.id.clone(), record);
    }

    pub fn get(&self, id: &str) -> Option<VectorRecord> {
        let data = self.data.read();
        data.get(id).cloned()
    }

    pub fn list_all(&self) -> Vec<VectorRecord> {
        let data = self.data.read();
        data.values().cloned().collect()
    }

    pub fn len(&self) -> usize {
        let data = self.data.read();
        data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for VectorStore {
    fn default() -> Self {
        Self::new()
    }
}
