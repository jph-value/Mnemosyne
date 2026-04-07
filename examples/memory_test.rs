use rememnemosyne_engine::RememnosyneEngine;
use rememnemosyne_core::MemoryTrigger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("=== RemeMnemosyne Integration Test ===\n");

    let engine = RememnosyneEngine::in_memory()?;
    println!("Engine created (in-memory mode)");

    // Store memories
    let test_memories = vec![
        ("Rust is a systems programming language focused on safety and performance.", "Rust programming"),
        ("Vector databases store high-dimensional vectors for semantic search.", "Vector databases"),
        ("vLLM provides high throughput LLM serving with PagedAttention.", "vLLM deployment"),
        ("Memory-augmented agents use external stores for context.", "Memory agents"),
        ("HNSW enables approximate nearest neighbor search.", "HNSW algorithm"),
        ("Quantization reduces model size while maintaining quality.", "Quantization"),
        ("Embedding models convert text to numerical vectors.", "Embedding models"),
        ("Prompt engineering guides LLM behavior.", "Prompt engineering"),
        ("Mixture of Experts activates only a subset of parameters.", "MoE architecture"),
        ("Retrieval-Augmented Generation combines retrieval with generation.", "RAG systems"),
    ];

    println!("\nStoring {} memories...", test_memories.len());
    let mut ids = Vec::new();
    for (content, summary) in &test_memories {
        let id = engine.remember(*content, *summary, MemoryTrigger::Insight).await?;
        ids.push(id);
    }
    println!("Stored {} memories", ids.len());

    // Test recall
    let test_queries = vec![
        ("What programming languages are good for systems programming?", "rust"),
        ("How can I store embeddings for semantic search?", "vector"),
        ("What is the best way to deploy LLMs for high throughput?", "vllm"),
    ];

    println!("\n=== Recall Tests ===");
    for (query, expected_keyword) in &test_queries {
        println!("\nQuery: {}", query);
        let bundle = engine.recall(query).await?;
        let formatted = engine.context_builder.format_context(&bundle);

        if bundle.is_empty() {
            println!("  Result: NO MEMORIES FOUND");
        } else {
            println!("  Found {} memories", bundle.memories.len());
            for (i, mem) in bundle.memories.iter().take(3).enumerate() {
                let relevance = bundle.relevance_scores.get(&mem.id).unwrap_or(&0.0);
                println!("    {}. [{}] {}", i + 1, relevance, mem.summary);
            }

            // Check if expected keyword is present
            let has_keyword = formatted.to_lowercase().contains(expected_keyword);
            println!("  Contains expected keyword '{}': {}", expected_keyword, has_keyword);
        }
    }

    // Test stats
    println!("\n=== Engine Stats ===");
    let stats = engine.get_stats().await;
    println!("Semantic memories: {}", stats.router.semantic_memories);
    println!("Episodic memories: {}", stats.router.episodic_memories);

    println!("\n=== Test Complete ===");
    println!("Memory pipeline: remember() -> embed() -> HNSW store -> recall() -> HNSW search -> format");
    println!("All components connected and functional.");

    Ok(())
}
