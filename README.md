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
[![Actix](https://img.shields.io/badge/Actix--Web-4.0-blue?style=for-the-badge)](https://actix.rs/)
[![Status](https://img.shields.io/badge/Status-Beta-green?style=for-the-badge)](#)

**Part of the Titan Protocol Initiative — System 03/300**

*HTTP API for Vector Storage and Similarity Search*

</div>

---

## 🚀 Quick Start

```bash
# Start the server
cd ~/NEXUS-L4-HighPerf-Vector-DB
cargo run --release

# Server runs on http://127.0.0.1:8081
```

---

## 📚 API Reference

### Insert Vector

```bash
curl -X POST http://127.0.0.1:8081/upsert \
  -H "Content-Type: application/json" \
  -d '{
    "id": "doc-001",
    "embedding": [0.1, 0.2, 0.3, ...],
    "metadata": {"title": "Example Document"}
  }'
```

### Search Nearest

```bash
curl -X POST http://127.0.0.1:8081/query \
  -H "Content-Type: application/json" \
  -d '{
    "embedding": [0.1, 0.2, 0.3, ...],
    "top_k": 5
  }'
```

### Health Check

```bash
curl http://127.0.0.1:8081/health
```

### Statistics

```bash
curl http://127.0.0.1:8081/stats
```

---

## 🏗️ Architecture

```
┌──────────────────────────────────────────────────────────────┐
│  Layer 1: Interface (HTTP API)                               │
│  └── /upsert, /query, /stats, /health                        │
├──────────────────────────────────────────────────────────────┤
│  Layer 2: Engine (Search)                                    │
│  └── search_nearest() - Top-K similarity search              │
├──────────────────────────────────────────────────────────────┤
│  Layer 3: Core (Math Kernels)                                │
│  └── cosine_similarity() - SIMD optimized                    │
├──────────────────────────────────────────────────────────────┤
│  Layer 4: Storage (In-Memory)                                │
│  └── VectorStore with RwLock                                 │
└──────────────────────────────────────────────────────────────┘
```

---

## ⚡ Performance

| Metric | Value |
|--------|-------|
| Cosine Similarity | ~500 ns/op |
| Port | 8081 |
| Concurrency | RwLock (parking_lot) |

---

<div align="center">

**Built with 🦀 Rust by [Davi Bonetto](https://github.com/DaviBonetto)**

</div>
