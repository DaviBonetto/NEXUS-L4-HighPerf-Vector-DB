//! Search engine orchestration

use crate::core::math::{cosine_similarity, euclidean_distance};
use anyhow::Result;

pub struct SearchResult {
    pub index: usize,
    pub score: f32,
}

pub fn find_most_similar(query: &[f32], vectors: &[Vec<f32>]) -> Result<SearchResult> {
    let mut best_idx = 0;
    let mut best_score = f32::MIN;
    
    for (i, vec) in vectors.iter().enumerate() {
        let score = cosine_similarity(query, vec)?;
        if score > best_score {
            best_score = score;
            best_idx = i;
        }
    }
    
    Ok(SearchResult { index: best_idx, score: best_score })
}
