# RemeMnemosyne Development Strategy

> Architecture decisions, coding standards, and working conventions for contributors

---

## Repository Layout

```
RemeMnemosyne/
├── crates/
│   ├── core/          # Types, traits, errors, config
│   ├── semantic/      # TurboQuant, HNSW, vector operations
│   ├── episodic/      # Conversation episodes, sessions
│   ├── graph/         # Entity relationships (petgraph)
│   ├── temporal/      # Timeline events, chronological access
│   ├── cognitive/     # Micro-embeddings, intent detection
│   ├── storage/       # sled (default) / RocksDB (optional)
│   └── engine/        # Unified public API
├── evaluation/        # Test results, improvement plan
└── target/            # Build artifacts
```

## Coding Standards

### Rust Conventions

- **Edition**: 2021
- **Formatting**: `cargo fmt` (default settings)
- **Linting**: `cargo clippy -- -D warnings`
- **Error handling**: `thiserror` for crate-local errors, `anyhow` at API boundaries
- **Async**: `tokio` with `async-trait` for trait methods
- **Naming**: `snake_case` for functions/variables, `PascalCase` for types, `SCREAMING_SNAKE` for constants

### Type Design Rules

1. **Every public type must derive `Debug`, `Clone`, `Serialize`, `Deserialize`** where possible
2. **Newtypes for IDs**: `MemoryId(Uuid)`, `EntityId(Uuid)`, `SessionId(Uuid)` — never bare `Uuid`
3. **Builder pattern for configs**: Use `builder()` or `default()` constructors
4. **Feature gates for optional deps**: `#[cfg(feature = "...")]` around optional functionality

### Error Patterns

```rust
// GOOD: Typed errors with context
#[derive(Debug, thiserror::Error)]
pub enum SemanticError {
    #[error("embedding dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    
    #[error("HNSW index not built")]
    IndexNotBuilt,
}

// GOOD: Convert to anyhow at boundaries
pub fn recall(query: &str) -> Result<MemoryBundle> {
    let embedding = self.embed(query)
        .context("failed to generate query embedding")?;
    // ...
}
```

### Documentation

- **Every public function** must have a doc comment
- **Every crate** must have a `lib.rs` doc comment explaining its purpose
- **Examples** in doc comments for non-trivial APIs
- **`#[must_use]`** on types that should not be ignored

## Dependency Rules

### Allowed (Pure Rust)

| Crate | Purpose |
|-------|---------|
| `sled` | Embedded storage |
| `parking_lot` | Synchronization primitives |
| `dashmap` | Concurrent hash maps |
| `petgraph` | Graph algorithms |
| `rayon` | Parallel computation |
| `half` | Float16/bfloat16 |
| `thiserror` | Error types |
| `anyhow` | Error handling at boundaries |
| `serde` + `serde_json` | Serialization |
| `uuid` | Unique identifiers |
| `chrono` | Timestamps |
| `tracing` | Structured logging |

### Allowed (Optional)

| Crate | Feature Gate | Notes |
|-------|-------------|-------|
| `rocksdb` | `persistence` | C++ dependency, opt-in only |

### Forbidden

- **No `unsafe` without review** — every `unsafe` block needs a `// SAFETY:` comment
- **No `unwrap()` in library code** — use `?` or `expect("reason")` with clear reason
- **No panicking code in public APIs** — return `Result` instead

## Testing Strategy

### Unit Tests

- **Every module** must have `#[cfg(test)] mod tests`
- **Test edge cases**: empty inputs, zero dimensions, boundary values
- **Use `assert_eq!`** for exact comparisons, `assert!` for conditions
- **Use `approx`** crate for floating-point comparisons

### Integration Tests

- Use the evaluation suite in `remeMnemosyne-hardening/evaluation/`
- Run `scripts/master_test.py` for full evaluation
- Each improvement phase must pass its validation gate

### Benchmarks

- Use `criterion` for micro-benchmarks
- Track: embedding generation, HNSW search, memory store, recall
- Run with `cargo bench` before any performance-sensitive changes

## API Design

### Public API Surface

The public API is ONLY in `rememnemosyne-engine`. Other crates are internal.

```rust
// PUBLIC: Only these types are exposed
pub use crate::{
    engine::RemeMnemosyneEngine,
    builder::EngineBuilder,
    api::MemoryApi,
    context::ContextBuilder,
};

// PRIVATE: Internal types stay internal
// semantic::HNSWIndex, semantic::TurboQuant, etc.
```

### Memory API Contract

```rust
/// Store a memory with automatic embedding generation
pub async fn remember(
    &self,
    content: impl Into<String>,
    summary: impl Into<String>,
    trigger: MemoryTrigger,
) -> Result<MemoryId>;

/// Recall relevant memories for a query
pub async fn recall(
    &self,
    query: impl AsRef<str>,
) -> Result<MemoryBundle>;

/// Recall with formatted output ready for LLM context
pub async fn recall_formatted(
    &self,
    query: impl AsRef<str>,
) -> Result<String>;
```

## Build Commands

```bash
# Format
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all

# Build (default, pure Rust)
cargo build --release

# Build with persistence (RocksDB)
cargo build --release --features persistence

# Benchmark
cargo bench
```

## Release Process

1. All changes go through feature branches
2. PR must pass: `fmt`, `clippy`, `test`, `bench` (no regressions)
3. Semantic versioning: `MAJOR.MINOR.PATCH`
4. Changelog updated with every release

---

*Strategy created: 2026-04-07*
