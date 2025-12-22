use anyhow::{Result, Context};
use private_context_core::{DocumentChunk, ChunkMetadata};
use std::path::Path;
use text_splitter::TextSplitter;
use std::fs;

pub trait DocumentParser {
    fn parse(&self, path: &Path) -> Result<Vec<DocumentChunk>>;
}

pub struct RecursiveCharacterParser {
    chunk_size: usize,
}

impl RecursiveCharacterParser {
    pub fn new(chunk_size: usize) -> Self {
        Self { chunk_size }
    }
}

impl DocumentParser for RecursiveCharacterParser {
    fn parse(&self, path: &Path) -> Result<Vec<DocumentChunk>> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file at {:?}", path))?;

        let splitter = TextSplitter::new(self.chunk_size);

        let source = path.to_string_lossy().to_string();
        
        let chunks = splitter.chunks(&content)
            .map(|chunk| DocumentChunk {
                content: chunk.to_string(),
                metadata: ChunkMetadata {
                    source: source.clone(),
                    proof_id: None,
                    provider: None,
                },
            })
            .collect();

        Ok(chunks)
    }
}
