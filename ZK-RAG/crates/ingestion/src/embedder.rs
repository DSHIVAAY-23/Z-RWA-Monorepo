use anyhow::{Result, Context};
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use tokenizers::Tokenizer;
use std::path::Path;
use std::fs;

pub trait Embedder {
    fn embed(&self, text: &str) -> Result<Vec<f32>>;
    fn dimension(&self) -> usize;
}

pub struct CandleEmbedder {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl CandleEmbedder {
    pub fn new(model_dir: &Path) -> Result<Self> {
        let device = Device::Cpu; // Default to CPU for privacy/portability
        
        let config_path = model_dir.join("config.json");
        let tokenizer_path = model_dir.join("tokenizer.json");
        let weights_path = model_dir.join("model.safetensors");

        let config = fs_err::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config)?;
        
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow::anyhow!("Tokenizer error: {}", e))?;
            
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_path], DTYPE, &device)?
        };
        
        let model = BertModel::load(vb, &config)?;

        Ok(Self { model, tokenizer, device })
    }

    pub fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        let tokens = self.tokenizer.encode_batch(texts.to_vec(), true)
            .map_err(|e| anyhow::anyhow!("Tokenization error: {}", e))?;
            
        let token_ids = tokens.iter()
            .map(|t| Tensor::new(t.get_ids(), &self.device))
            .collect::<Result<Vec<_>, _>>()?;
            
        let token_ids = Tensor::stack(&token_ids, 0)?;
        let token_type_ids = token_ids.zeros_like()?; // BERT expects token type ids
        
        let embeddings = self.model.forward(&token_ids, &token_type_ids, None)?;
        
        // Mean pooling
        let (_n_batch, n_tokens, _hidden_size) = embeddings.dims3()?;
        let embeddings = (embeddings.sum(1)? / (n_tokens as f64))?;
        
        let results = embeddings.to_vec2::<f32>()?;
        Ok(results)
    }
}

impl Embedder for CandleEmbedder {
    fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let batch = self.embed_batch(&[text])?;
        Ok(batch.into_iter().next().unwrap())
    }

    fn dimension(&self) -> usize {
        // BERT base usually has 768 or 384 for MiniLM
        768 // This should be dynamic based on config, but for MVP...
    }
}
