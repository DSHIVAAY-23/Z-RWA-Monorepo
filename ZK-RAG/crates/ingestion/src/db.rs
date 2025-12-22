use anyhow::{Result, Context};
use private_context_core::{DocumentChunk, ChunkMetadata};
use hnsw_rs::prelude::*;
use sled::Db;
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufReader, BufWriter};

/// Pure-Rust Vector Store using HNSW (Index) + Sled (Storage)
pub struct LocalStore {
    index: Arc<Mutex<Hnsw<'static, f32, DistL2>>>,
    db: Db,
    index_path: String,
}

impl LocalStore {
    pub fn new(data_dir: &str) -> Result<Self> {
        let path = Path::new(data_dir);
        let db_path = path.join("doc_store");
        let index_path = path.join("vector_index.bin");

        // 1. Initialize Sled (KV Store)
        let db = sled::open(db_path).context("Failed to open Sled DB")?;
        let vectors_tree = db.open_tree("vectors").context("Failed to open vectors tree")?;

        println!("Initializing vector index...");
        // Max elements, M (connections), ef_construction (search depth during build)
        let max_elements = 1_000_000;
        let m = 24; 
        let max_layer = 16;
        let ef_construction = 400;
        let idx: Hnsw<'static, f32, DistL2> = Hnsw::new(m, max_elements, max_layer, ef_construction, DistL2);
        
        // Rebuild index from Sled if exists
        if !vectors_tree.is_empty() {
             println!("Rebuilding index from storage ({} items)...", vectors_tree.len());
             for item in vectors_tree.iter() {
                 let (key, value) = item?;
                 let id = u64::from_be_bytes(key.as_ref().try_into()?);
                 let vector: Vec<f32> = serde_json::from_slice(&value)?;
                 idx.insert((&vector, id as usize));
             }
        }

        let index = Arc::new(Mutex::new(idx));

        Ok(Self {
            index,
            db,
            index_path: index_path.to_string_lossy().to_string(),
        })
    }

    pub fn add_chunks(&self, chunks: Vec<DocumentChunk>, embeddings: Vec<Vec<f32>>) -> Result<()> {
        let mut index = self.index.lock().unwrap();
        let vectors_tree = self.db.open_tree("vectors")?;
        
        let start_id = index.get_nb_point(); 

        // Batch insert into Sled and Index
        for (i, (chunk, vector)) in chunks.into_iter().zip(embeddings.into_iter()).enumerate() {
            let id = start_id + i;
            
            // 1. Insert into HNSW
            index.insert((&vector, id));

            // 2. Insert into Sled (Docs and Vectors)
            let key = (id as u64).to_be_bytes();
            
            // Store Chunk
            let chunk_bytes = serde_json::to_vec(&chunk)?;
            self.db.insert(key, chunk_bytes)?;
            
            // Store Vector (for index rebuilding)
            let vec_bytes = serde_json::to_vec(&vector)?;
            vectors_tree.insert(key, vec_bytes)?;
        }

        // Flush to disk
        self.db.flush()?;
        vectors_tree.flush()?;
        
        Ok(())
    }

    pub fn search(&self, query_vector: &[f32], k: usize) -> Result<Vec<DocumentChunk>> {
        let index = self.index.lock().unwrap();
        
        // Search returns Vec<Neighbour { d_id: usize, distance: f32, p_id: PointId }>
        let results = index.search(query_vector, k, 16); 

        let mut documents = Vec::new();
        for neighbor in results {
            let id = neighbor.d_id; // Fixed field name
            let key = (id as u64).to_be_bytes();

            if let Some(data) = self.db.get(key)? {
                let chunk: DocumentChunk = serde_json::from_slice(&data)?;
                documents.push(chunk);
            }
        }

        Ok(documents)
    }
}
