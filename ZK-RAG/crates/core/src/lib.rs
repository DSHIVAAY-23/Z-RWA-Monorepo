pub mod math;
#[cfg(feature = "reclaim-rust-sdk")]
pub mod verifier;

use serde::{Serialize, Deserialize};

/// Represents a chunk of a document with its associated metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub content: String,
    pub metadata: ChunkMetadata,
}

/// Metadata associated with a document chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub source: String,
    pub proof_id: Option<String>,
    pub provider: Option<String>,
}

pub trait VectorStore {
    type Vector;
    type Error;

    fn add_chunks(&mut self, chunks: Vec<DocumentChunk>, embeddings: Vec<Self::Vector>) -> Result<(), Self::Error>;
    fn search(&self, query_vector: &Self::Vector, k: usize) -> Result<Vec<DocumentChunk>, Self::Error>;
}
