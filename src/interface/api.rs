//! HTTP API Routes for NEXUS Vector DB

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::engine::search_nearest;
use crate::storage::{VectorRecord, VectorStore};

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct UpsertRequest {
    pub id: String,
    pub embedding: Vec<f32>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct UpsertResponse {
    pub success: bool,
    pub id: String,
    pub dimension: usize,
}

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub embedding: Vec<f32>,
    #[serde(default = "default_top_k")]
    pub top_k: usize,
}

fn default_top_k() -> usize {
    10
}

#[derive(Debug, Serialize)]
pub struct QueryResponse {
    pub results: Vec<MatchResult>,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct MatchResult {
    pub id: String,
    pub score: f32,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total_vectors: usize,
    pub status: String,
}

// ============================================================================
// Route Handlers
// ============================================================================

/// POST /upsert - Insert or update a vector
pub async fn upsert(
    store: web::Data<Arc<VectorStore>>,
    body: web::Json<UpsertRequest>,
) -> impl Responder {
    let dimension = body.embedding.len();

    let record = VectorRecord {
        id: body.id.clone(),
        embedding: body.embedding.clone(),
        metadata: body.metadata.clone(),
    };

    store.insert(record);

    println!("📥 UPSERT: id={}, dim={}", body.id, dimension);

    HttpResponse::Ok().json(UpsertResponse {
        success: true,
        id: body.id.clone(),
        dimension,
    })
}

/// POST /query - Search for nearest vectors
pub async fn query(
    store: web::Data<Arc<VectorStore>>,
    body: web::Json<QueryRequest>,
) -> impl Responder {
    let results = search_nearest(&store, &body.embedding, body.top_k);

    let matches: Vec<MatchResult> = results
        .into_iter()
        .map(|r| MatchResult {
            id: r.id,
            score: r.score,
            metadata: r.metadata,
        })
        .collect();

    let count = matches.len();

    println!(
        "🔍 QUERY: dim={}, top_k={}, found={}",
        body.embedding.len(),
        body.top_k,
        count
    );

    HttpResponse::Ok().json(QueryResponse {
        results: matches,
        count,
    })
}

/// GET /stats - Get database statistics
pub async fn stats(store: web::Data<Arc<VectorStore>>) -> impl Responder {
    let total = store.len();

    HttpResponse::Ok().json(StatsResponse {
        total_vectors: total,
        status: "operational".to_string(),
    })
}

/// GET /health - Health check
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("NEXUS-L4 Vector DB: OPERATIONAL 🟢")
}

// ============================================================================
// Route Configuration
// ============================================================================

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/health", web::get().to(health))
            .route("/stats", web::get().to(stats))
            .route("/upsert", web::post().to(upsert))
            .route("/query", web::post().to(query)),
    );
}
