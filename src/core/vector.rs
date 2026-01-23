//! Vector types and operations

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector {
    pub id: String,
    pub data: Vec<f32>,
    pub metadata: Option<String>,
}

impl Vector {
    pub fn new(id: String, data: Vec<f32>) -> Self {
        Self { id, data, metadata: None }
    }
    
    pub fn dimension(&self) -> usize {
        self.data.len()
    }
}
