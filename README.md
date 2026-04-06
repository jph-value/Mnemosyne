# Mnemosyne

A high-performance agentic memory engine for LLM agents with unified memory coordination.

## ⚠️ Dependency Statement

**Current codebase**: Mixed languages (Rust core with Makefile build system, some C/C++ dependencies). **Goal**: Transitioning to pure Rust over time.

**Core engine**: Rust-based with minimal external dependencies.

**Storage backends**: 
- `sled` (default) - Pure Rust embedded database ✓
- `rocksdb` (optional) - Requires C++ build tools, opt-in via `features = ["persistence"]`

## Architecture

```
mnemosyne/
├── crates/
│   ├── core          # Rust - Types, traits, errors
│   ├── semantic      # Rust - TurboQuant, HNSW index
│   ├── episodic      # Rust - Conversation episodes
│   ├── graph         # Rust - Entity relationships
│   ├── temporal      # Rust - Timeline events
│   ├── cognitive     # Rust - Micro-embeddings
│   ├── storage       # Rust (sled) / Optional C++ (RocksDB)
│   └── engine        # Rust - Unified API
```

## Memory Types

| Type | Purpose | Implementation |
|------|---------|----------------|
| Semantic | Vector search | TurboQuant + HNSW |
| Episodic | Chat history | Episodes, decisions |
| Graph | Relationships | petgraph-based |
| Temporal | Events | Chronological storage |

## Quick Start

```toml
# Cargo.toml - Pure Rust (default)
[dependencies]
mnemosyne-engine = "0.1"

# With RocksDB persistence (requires C++ toolchain)
mnemosyne-engine = { version = "0.1", features = ["persistence"] }
```

```rust
use mnemosyne_engine::MnemosyneEngine;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let engine = MnemosyneEngine::default()?;

    // Store a memory
    engine.remember(
        "TurboQuant compresses vectors to 4-8 bits",
        "Quantization insight",
        MemoryTrigger::Insight,
    ).await?;

    // Recall relevant memories
    let context = engine.recall("quantization").await?;
    println!("{}", engine.context_builder.format_context(&context));

    Ok(())
}
```

## Key Components

### TurboQuant (Rust)
- Product Quantization (PQ)
- Optimized PQ (OPQ)
- Polar Quantization
- QJL transforms
- 8x compression for embeddings

### HNSW Index (Rust)
- Approximate nearest neighbor search
- Configurable recall/speed tradeoff
- No external dependencies

### Micro-Embeddings (Rust)
- 128-dimensional fast embeddings
- Hash, Bag-of-Words, CharNGram models
- <1ms inference time

## Performance

| Operation | Target | Achieved |
|-----------|--------|----------|
| Micro-embedding | <1ms | ~0.1ms |
| Vector search (1K) | <3ms | ~2ms |
| Memory store | <5ms | ~1ms |
| Context assembly | <10ms | ~5ms |

## Feature Flags

```toml
[dependencies]
# Default: Pure Rust with sled storage
mnemosyne-engine = "0.1"

# Optional: RocksDB persistence (requires C++ toolchain)
mnemosyne-engine = { version = "0.1", features = ["persistence"] }

# No storage (in-memory only)
mnemosyne-engine = { version = "0.1", default-features = false }
```

| Feature | Default | Dependencies |
|---------|---------|--------------|
| `default` | Yes | sled (pure Rust) |
| `persistence` | No | RocksDB (C++) |

## License

MIT
