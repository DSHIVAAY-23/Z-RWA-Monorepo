mod db;
mod embedder;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser)]
#[command(name = "private-context")]
#[command(about = "Private Context Ingestion CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ingest a text file
    Ingest {
        /// Path to the text file
        file: String,
        /// Path to the model directory
        #[arg(long, default_value = "../../models/all-MiniLM-L6-v2")]
        model_dir: String,
        /// Database URI
        #[arg(long, default_value = "../../data/lancedb")]
        db_uri: String,
    },
    /// Search for relevant context
    Search {
        /// Query text
        query: String,
         /// Path to the model directory
        #[arg(long, default_value = "../../models/all-MiniLM-L6-v2")]
        model_dir: String,
        /// Database URI
        #[arg(long, default_value = "../../data/lancedb")]
        db_uri: String,
    },
    /// Prove relevance (Mock ZK)
    Prove {
        /// Query text
        query: String,
         /// Path to the model directory
        #[arg(long, default_value = "../../models/all-MiniLM-L6-v2")]
        model_dir: String,
        /// Database URI
        #[arg(long, default_value = "../../data/lancedb")]
        db_uri: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ingest { file, model_dir, db_uri } => {
            println!("🚀 Starting Ingestion...");
            println!("📂 File: {}", file);
            println!("🤖 Model: {}", model_dir);
            println!("💾 DB: {}", db_uri);

            // 1. Read File
            let text = std::fs::read_to_string(&file).context("Failed to read input file")?;

            // 2. Initialize Embedder & Tokenizer
            let embedder_path = Path::new(&model_dir);
            let embedder = embedder::CandleEmbedder::new(&embedder_path)
                .context("Failed to initialize embedder")?;
            
            // 3. Spilt & Embed (Manual Chunking)
            let chunk_size = 500;
            let chunks_vec: Vec<String> = text
                .chars()
                .collect::<Vec<char>>()
                .chunks(chunk_size)
                .map(|c| c.iter().collect::<String>())
                .collect();
            
            let mut ids = Vec::new();
            let mut texts = Vec::new();
            let mut sources = Vec::new();
            let mut vectors = Vec::new();

            for (i, chunk) in chunks_vec.iter().enumerate() {
                let chunk = chunk.as_str(); // Use slice
                println!("   - Processing chunk {}: {} chars", i, chunk.len());
                let embedding = embedder.embed(chunk)?;
                
                ids.push(format!("{}-{}", file, i));
                texts.push(chunk.to_string());
                sources.push(file.clone());
                vectors.push(embedding);
            }

            // 4. Store
            let store = db::VectorStore::new(&db_uri).await?;
            store.add(ids, texts, sources, vectors).await?;
            
            println!("✅ Ingestion Complete! Stored {} chunks.", chunks_vec.len());
        }
        Commands::Search { query, model_dir, db_uri } => {
            println!("🔍 Searching: '{}'", query);

            let embedder_path = Path::new(&model_dir);
            let embedder = embedder::CandleEmbedder::new(&embedder_path)
                .context("Failed to initialize embedder")?;
            
            let query_vec = embedder.embed(&query)?;
            
            let store = db::VectorStore::new(&db_uri).await?;
            let results = store.search(&query_vec, 3).await?;

            println!("🎯 Found {} results:", results.len());
            for (text, _, _) in results {
                println!("--------------------------------------------------");
                println!("{}", text);
            }
            println!("--------------------------------------------------");
        }
        Commands::Prove { query, model_dir, db_uri } => {
            println!("🔒 Starting Private Proof Generation for: '{}'", query);

            // 1. Embed Query
            let embedder_path = Path::new(&model_dir);
            let embedder = embedder::CandleEmbedder::new(&embedder_path)
                .context("Failed to initialize embedder")?;
            let query_vec = embedder.embed(&query)?;

            // 2. Fetch relevant chunk from DB
            let store = db::VectorStore::new(&db_uri).await?;
            let results = store.search(&query_vec, 1).await?;
            
            if results.is_empty() {
                println!("❌ No documents found.");
                return Ok(());
            }
            let (chunk_text, _, chunk_vec) = &results[0];
            println!("📄 Found relevant chunk: \n'{}...'", chunk_text.chars().take(50).collect::<String>());

            // 3. Mock Prover Execution (Host verification of ZK logic)
            // In a real SP1 setup, we would write these inputs to SP1Stdin and run the prover.
            // Here we run the same logic to verify correctness.
            
            use private_context_core::math::{cosine_similarity, to_fixed, Fixed};
            
            let query_fixed = to_fixed(&query_vec);
            let chunk_fixed = to_fixed(&chunk_vec);
            let threshold = Fixed::from_num(0.7); // Mock threshold
            
            let similarity = cosine_similarity(&query_fixed, &chunk_fixed);
            let is_relevant = similarity >= threshold;
            
            let sim_f32: f32 = similarity.to_num();
            
            println!("⚙️  [MockProver] Execute ZK Circuit Logic...");
            println!("   - Similarity: {}", sim_f32);
            println!("   - Threshold:  0.7");
            
            if is_relevant {
                println!("✅ Proof Valid: Chunk is relevant.");
            } else {
                println!("❌ Proof Valid: Chunk is NOT relevant.");
            }
        }
    }

    Ok(())
}
