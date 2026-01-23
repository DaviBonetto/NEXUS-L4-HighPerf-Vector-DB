//! In-memory vector storage

use crate::core::vector::Vector;
use std::collections::HashMap;

pub struct MemoryStore {
    vectors: HashMap<String, Vector>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self { vectors: HashMap::new() }
    }
    
    pub fn insert(&mut self, vector: Vector) {
        self.vectors.insert(vector.id.clone(), vector);
    }
    
    pub fn get(&self, id: &str) -> Option<&Vector> {
        self.vectors.get(id)
    }
    
    pub fn len(&self) -> usize {
        self.vectors.len()
    }
    
    pub fn all_vectors(&self) -> Vec<&Vector> {
        self.vectors.values().collect()
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}
