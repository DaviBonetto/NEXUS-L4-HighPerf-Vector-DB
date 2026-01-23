//! NEXUS-L4 Vector Database Server
//! High-Performance Vector Search API

use actix_web::{middleware, web, App, HttpServer};
use std::sync::Arc;

use nexus_vector_db::interface::configure_routes;
use nexus_vector_db::storage::VectorStore;

const SERVER_HOST: &str = "127.0.0.1";
const SERVER_PORT: u16 = 8081;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("══════════════════════════════════════════════════════════════════════");
    println!("🧬 NEXUS-L4 // High-Performance Vector Database");
    println!("══════════════════════════════════════════════════════════════════════");

    // Initialize Vector Store
    let store = Arc::new(VectorStore::new());
    println!("💾 VectorStore initialized (in-memory)");

    println!(
        "🌐 Starting HTTP server on http://{}:{}",
        SERVER_HOST, SERVER_PORT
    );
    println!("══════════════════════════════════════════════════════════════════════");
    println!("");
    println!("📌 Endpoints:");
    println!("   POST /upsert  - Insert/update vector");
    println!("   POST /query   - Search nearest vectors");
    println!("   GET  /stats   - Database statistics");
    println!("   GET  /health  - Health check");
    println!("");
    println!("══════════════════════════════════════════════════════════════════════");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(store.clone()))
            .wrap(middleware::Logger::default())
            .configure(configure_routes)
    })
    .bind((SERVER_HOST, SERVER_PORT))?
    .run()
    .await
}
