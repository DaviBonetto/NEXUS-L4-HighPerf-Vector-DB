//! Search Engine - Finds nearest vectors

use crate::core::math::cosine_similarity;
use crate::storage::{VectorRecord, VectorStore};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub metadata: serde_json::Value,
}

pub fn search_nearest(
    store: &VectorStore,
    query: &[f32],
    top_k: usize,
) -> Vec<SearchResult> {
    let vectors = store.list_all();
    
    let mut results: Vec<SearchResult> = vectors
        .iter()
        .filter_map(|record| {
            match cosine_similarity(query, &record.embedding) {
                Ok(score) => Some(SearchResult {
                    id: record.id.clone(),
                    score,
                    metadata: record.metadata.clone(),
                }),
                Err(_) => None,
            }
        })
        .collect();
    
    // Sort by score descending (highest similarity first)
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    
    // Return top-k
    results.truncate(top_k);
    results
}
