mod embedder;
mod db;
mod parser;

use clap::{Parser, Subcommand};
use anyhow::{Result, Context};
use std::path::Path;
use crate::embedder::{CandleEmbedder, Embedder};
use crate::parser::{RecursiveCharacterParser, DocumentParser};
use crate::db::LocalStore;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ingest a document into the private database
    Ingest {
        /// Path to the document
        path: String,
        /// Model directory (contains config.json, tokenizer.json, model.safetensors)
        #[arg(long, default_value = "./models/all-MiniLM-L6-v2")]
        model_dir: String,
        /// Path to Reclaim Protocol proof JSON file
        #[arg(long)]
        proof: Option<String>,
    },
    /// Search for relevant documents
    Search {
        /// Query text
        text: String,
        /// Number of results to return
        #[arg(short, default_value_t = 5)]
        k: usize,
        /// Model directory
        #[arg(long, default_value = "./models/all-MiniLM-L6-v2")]
        model_dir: String,
    },
    /// Prove relevance (experimental)
    Prove {
        /// Query text
        text: String,
        /// Model directory
        #[arg(long, default_value = "./models/all-MiniLM-L6-v2")]
        model_dir: String,
        /// Threshold for similarity (default 0.7)
        #[arg(long, default_value_t = 0.7)]
        threshold: f32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let data_dir = "data";
    
    // Ensure data directory exists
    std::fs::create_dir_all(data_dir)?;

    match cli.command {
        Commands::Ingest { path, model_dir, proof } => {
            // ... (existing ingestion code)
            println!("🚀 Ingesting document from: {}", path);
            
            // 1. Verify Proof if provided
            let mut proof_id = None;
            let mut provider = None;

            if let Some(proof_path) = proof {
                println!("🔒 Verifying Reclaim proof: {}", proof_path);
                let proof_json = std::fs::read_to_string(&proof_path)
                    .context("Failed to read proof file")?;
                
                let proof_obj = private_context_core::verifier::Proof::new(&proof_json);
                
                if private_context_core::verifier::verify_proof(&proof_obj).await? {
                    println!("✅ Proof verified successfully!");
                    // In a real app, we'd extract these from the decoded proof
                    proof_id = Some("verified-proof-id".to_string()); 
                    provider = Some("reclaim-provider".to_string());
                } else {
                    return Err(anyhow::anyhow!("❌ Proof verification failed!"));
                }
            }

            let embedder = CandleEmbedder::new(Path::new(&model_dir))
                .context("Failed to initialize embedder. Ensure model files are in the specified directory.")?;
            
            let parser = RecursiveCharacterParser::new(500); // 500 chars chunk size
            let mut chunks = parser.parse(Path::new(&path))?;
            println!("📄 Split into {} chunks", chunks.len());

            // Attach Proof Metadata
            if proof_id.is_some() {
                 for chunk in &mut chunks {
                     chunk.metadata.proof_id = proof_id.clone();
                     chunk.metadata.provider = provider.clone();
                 }
            }

            let texts: Vec<&str> = chunks.iter().map(|c| c.content.as_str()).collect();
            let embeddings = embedder.embed_batch(&texts)?;

            let store = LocalStore::new(data_dir)?;
            store.add_chunks(chunks, embeddings)?;
            println!("✅ Ingestion complete!");
        }
        Commands::Search { text, k, model_dir } => {
            println!("🔍 Searching for: '{}'", text);
            
            let embedder = CandleEmbedder::new(Path::new(&model_dir))
                .context("Failed to initialize embedder")?;
            
            let query_vector = embedder.embed(&text)?;
            
            let store = LocalStore::new(data_dir)?;
            let results = store.search(&query_vector, k)?;
            
            println!("🎯 Found {} results:", results.len());
            for (i, res) in results.iter().enumerate() {
                println!("\n[{}] Relevance rank", i + 1);
                println!("Source: {}", res.metadata.source);
                println!("Content: {}...", res.content.chars().take(200).collect::<String>());
            }
        }
        Commands::Prove { text, model_dir, threshold } => {
            use sp1_sdk::{ProverClient, SP1Stdin, HashableKey};

            println!("🧪 Generating Proof for: '{}'", text);

            // 1. Get embedding for query
            let embedder = CandleEmbedder::new(Path::new(&model_dir))
                .context("Failed to initialize embedder")?;
            let query_vector = embedder.embed(&text)?;

            // 2. Fetch top result (mockup: using search to get a chunk)
            let store = LocalStore::new(data_dir)?;
            let results = store.search(&query_vector, 1)?;
            
            if results.is_empty() {
                println!("❌ No documents found to prove against.");
                return Ok(());
            }
            let chunk = &results[0];
            println!("📄 Proving relevance against chunk from: {}", chunk.metadata.source);
            
            // Note: In a real scenario we'd fetch the vector from DB, but search result currently only returns chunks.
            // For this MVP, we re-embed or modify search to return vectors? 
            // Or simpler: We simulate the vector since we verify the *logic* of the circuit.
            // Let's re-embed the chunk content for now (inefficient but works for MVP).
            let chunk_vector = embedder.embed(&chunk.content)?;

            // 3. Setup SP1 Prover inputs
            let mut stdin = SP1Stdin::new();
            stdin.write(&query_vector);
            stdin.write(&chunk_vector);
            stdin.write(&threshold);
            
            // Calculate document hash
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(chunk.content.as_bytes());
            let document_hash: [u8; 32] = hasher.finalize().into();
            stdin.write(&document_hash);

            // 4. Generate Proof
            let elf_path = "crates/circuits/elf/riscv32im-succinct-zkvm-elf"; 
            
            if !std::path::Path::new(elf_path).exists() {
                 println!("⚠️  Guest ELF not found at {}.", elf_path);
                 return Ok(());
            }

            println!("🔨 Generating proof...");
            
            // Check SP1_PROVER env var
            let prover_mode = std::env::var("SP1_PROVER").unwrap_or("local".to_string());
            println!("🚀 Active Mode: {}", prover_mode.to_uppercase());

            let client = ProverClient::new();
            let (pk, vk) = client.setup(&std::fs::read(elf_path)?);

            // Export VKEY Hash for Solana (Do this before Mock check)
            let vkey_hash = vk.bytes32();
            println!("🔑 VKey Hash: {}", vkey_hash);
            std::fs::write("vkey_hash.txt", &vkey_hash).context("Failed to write vkey hash")?;

            if prover_mode.to_lowercase() == "mock" {
                println!("⚠️  Running in MOCK mode. Skipping Groth16 generation.");
                let (public_values, _report) = client.execute(&pk.elf, stdin).run().unwrap();
                println!("✅ Mock execution successful!");
                
                // Save Public Values
                let public_values_bytes = public_values.to_vec();
                let pub_path = "public_values.bin";
                std::fs::write(pub_path, &public_values_bytes).context("Failed to write public values")?;
                println!("💾 Public Values saved to {}", pub_path);
                
                // Verify public output
                let mut pv = public_values.clone();
                let _committed_hash: [u8; 32] = pv.read();
                let is_relevant: bool = pv.read();
                let similarity: f32 = pv.read();
                
                println!("🎉 Verification Result: Relevant? {}", is_relevant);
                println!("📊 Computed Similarity: {}", similarity);
                return Ok(());
            }
            
            // Generate Groth16 proof
            println!("🔒 Generating full Groth16 proof...");
            let mut proof = client.prove(&pk, stdin).groth16().run().unwrap();

            println!("✅ Proof generated successfully!");
            
            // Save proof for Solana
            let proof_bytes = proof.bytes();
            let proof_path = "proof_groth16.bin";
            std::fs::write(proof_path, &proof_bytes).context("Failed to write proof file")?;
            println!("💾 Proof saved to {}", proof_path);

            // Save Public Values
            let public_values_bytes = proof.public_values.to_vec();
            let pub_path = "public_values.bin";
            std::fs::write(pub_path, &public_values_bytes).context("Failed to write public values")?;
            println!("💾 Public Values saved to {}", pub_path);

            // Export VKEY Hash for Solana
            let vkey_hash = vk.bytes32();
            println!("🔑 VKey Hash: {}", vkey_hash);
            std::fs::write("vkey_hash.txt", vkey_hash).context("Failed to write vkey hash")?;

            // Verify public output
            // Read in order: document_hash, is_relevant, similarity
            let _committed_hash: [u8; 32] = proof.public_values.read();
            let is_relevant: bool = proof.public_values.read();
            let similarity: f32 = proof.public_values.read();
            
            println!("🎉 Verification Result: Relevant? {}", is_relevant);
            println!("📊 Computed Similarity: {}", similarity);
        }
    }

    Ok(())
}
