use anyhow::{Error, Result};
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::Tokenizer;
use std::path::Path;

pub struct CandleEmbedder {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl CandleEmbedder {
    pub fn new(model_path: &Path) -> Result<Self> {
        let device = Device::Cpu;

        let config_path = model_path.join("config.json");
        let weights_path = model_path.join("model.safetensors");
        let tokenizer_path = model_path.join("tokenizer.json");

        let config: Config = serde_json::from_str(&std::fs::read_to_string(config_path)?)?;
        let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(Error::msg)?;
        
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_path], candle_core::DType::F32, &device)? };
        let model = BertModel::load(vb, &config)?;

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    pub fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let tokens = self.tokenizer
            .encode(text, true)
            .map_err(Error::msg)?;
        
        // Handle max length (384 for MiniLM usually, or 512)
        let token_ids = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        let token_type_ids = Tensor::new(tokens.get_type_ids(), &self.device)?.unsqueeze(0)?;

        let embeddings = self.model.forward(&token_ids, &token_type_ids)?;
        
        // Mean pooling: take average of all tokens
        let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
        let embeddings = (embeddings.sum(1)? / (n_tokens as f64))?;
        let embeddings = embeddings.get(0)?;
        
        // Normalize (optional but good for cosine similarity)
        // For simplicity and matching typical sentence-transformers, we might normalize.
        // Let's stick to raw valid output first.
        
        let vec: Vec<f32> = embeddings.to_vec1()?;
        Ok(vec)
    }
}
