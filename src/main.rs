//! NEXUS-L4 Performance Benchmark
//! Tests math kernel with OpenAI Ada-002 dimension (1536)

use nexus_vector_db::core::math::{cosine_similarity, euclidean_distance};
use nexus_vector_db::interface::cli::print_banner;
use rand::Rng;
use std::time::Instant;

const VECTOR_DIM: usize = 1536; // OpenAI Ada-002 embedding dimension
const WARMUP_RUNS: usize = 100;
const BENCHMARK_RUNS: usize = 1000;

fn generate_random_vector(dim: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..dim).map(|_| rng.gen::<f32>()).collect()
}

fn main() {
    print_banner();
    
    println!("\n📊 Benchmark Configuration:");
    println!("   Dimension: {} (OpenAI Ada-002)", VECTOR_DIM);
    println!("   Warmup Runs: {}", WARMUP_RUNS);
    println!("   Benchmark Runs: {}", BENCHMARK_RUNS);
    
    // Generate test vectors
    let vec_a = generate_random_vector(VECTOR_DIM);
    let vec_b = generate_random_vector(VECTOR_DIM);
    
    // Warmup
    println!("\n🔥 Warming up...");
    for _ in 0..WARMUP_RUNS {
        let _ = cosine_similarity(&vec_a, &vec_b);
    }
    
    // Benchmark Cosine Similarity
    println!("⏱️ Benchmarking Cosine Similarity...");
    let start = Instant::now();
    let mut cos_result = 0.0;
    for _ in 0..BENCHMARK_RUNS {
        cos_result = cosine_similarity(&vec_a, &vec_b).unwrap();
    }
    let cos_duration = start.elapsed();
    let cos_ns_per_op = cos_duration.as_nanos() / BENCHMARK_RUNS as u128;
    
    // Benchmark Euclidean Distance
    println!("⏱️ Benchmarking Euclidean Distance...");
    let start = Instant::now();
    let mut euc_result = 0.0;
    for _ in 0..BENCHMARK_RUNS {
        euc_result = euclidean_distance(&vec_a, &vec_b).unwrap();
    }
    let euc_duration = start.elapsed();
    let euc_ns_per_op = euc_duration.as_nanos() / BENCHMARK_RUNS as u128;
    
    // Results
    println!("\n══════════════════════════════════════════════════════════════════════");
    println!("📈 BENCHMARK RESULTS");
    println!("══════════════════════════════════════════════════════════════════════");
    println!("🧬 NEXUS-L4 Math Kernel | Dim: {} | Runs: {}", VECTOR_DIM, BENCHMARK_RUNS);
    println!("──────────────────────────────────────────────────────────────────────");
    println!("   Cosine Similarity:   {:>8} ns/op | Result: {:.6}", cos_ns_per_op, cos_result);
    println!("   Euclidean Distance:  {:>8} ns/op | Result: {:.6}", euc_ns_per_op, euc_result);
    println!("══════════════════════════════════════════════════════════════════════");
    println!("✅ Benchmark Complete!");
}
