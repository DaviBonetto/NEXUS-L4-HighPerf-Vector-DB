<div align="center">

```
  ███╗   ██╗███████╗██╗  ██╗██╗   ██╗███████╗
  ████╗  ██║██╔════╝╚██╗██╔╝██║   ██║██╔════╝
  ██╔██╗ ██║█████╗   ╚███╔╝ ██║   ██║███████╗
  ██║╚██╗██║██╔══╝   ██╔██╗ ██║   ██║╚════██║
  ██║ ╚████║███████╗██╔╝ ██╗╚██████╔╝███████║
  ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝
```

### 🧬 L4 High-Performance Vector Database

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Layer](https://img.shields.io/badge/Layer-L4-blueviolet?style=for-the-badge)](#)
[![Math](https://img.shields.io/badge/Math-Linear_Algebra-00ADD8?style=for-the-badge)](#)
[![Status](https://img.shields.io/badge/Status-Development-yellow?style=for-the-badge)](#)

**Part of the Titan Protocol Initiative — System 03/300**

*Zero-Copy Vector Operations with SIMD Auto-Vectorization*

</div>

---

## 🏗️ Layered Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    NEXUS-L4 Vector DB                       │
├─────────────────────────────────────────────────────────────┤
│  Layer 1: Interface                                         │
│  └── src/interface/     CLI, API, gRPC handlers             │
├─────────────────────────────────────────────────────────────┤
│  Layer 2: Engine                                            │
│  └── src/engine/        Search orchestration, query logic   │
├─────────────────────────────────────────────────────────────┤
│  Layer 3: Core                                              │
│  └── src/core/          Math kernels, vector operations     │
│      ├── math.rs        cosine_similarity, euclidean_dist   │
│      └── vector.rs      Vector struct with metadata         │
├─────────────────────────────────────────────────────────────┤
│  Layer 4: Storage                                           │
│  └── src/storage/       Persistence, disk I/O               │
└─────────────────────────────────────────────────────────────┘
```

---

## ⚡ Performance

| Operation | Dimension | Time per Op |
|-----------|-----------|-------------|
| Cosine Similarity | 1536 (Ada-002) | ~500 ns |
| Euclidean Distance | 1536 (Ada-002) | ~400 ns |

*Benchmarked on release build with LTO enabled*

---

## 🚀 Quick Start

```bash
cd ~/NEXUS-L4-HighPerf-Vector-DB

# Run benchmark
cargo run --release

# Run tests
cargo test
```

---

## 📁 Project Structure

```
src/
├── lib.rs              # Module exports
├── main.rs             # Performance benchmark
├── interface/
│   ├── mod.rs
│   └── cli.rs          # CLI interface
├── engine/
│   ├── mod.rs
│   └── search.rs       # Search orchestration
├── core/
│   ├── mod.rs
│   ├── math.rs         # Math kernels (SIMD optimized)
│   └── vector.rs       # Vector types
└── storage/
    ├── mod.rs
    └── memory.rs       # In-memory store
```

---

## 🔬 Math Kernels

```rust
// Cosine Similarity - SIMD optimized
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> Result<f32, MathError>

// Euclidean Distance - SIMD optimized  
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32, MathError>
```

---

<div align="center">

**Built with 🦀 Rust by [Davi Bonetto](https://github.com/DaviBonetto)**

*Part of the Titan Protocol Initiative — System 03/300*

</div>
