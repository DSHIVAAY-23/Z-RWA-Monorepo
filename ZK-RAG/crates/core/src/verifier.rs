use anyhow::{Result, anyhow};
use reclaim_rust_sdk::{verify_proof as sdk_verify_proof, Proof as ReclaimProof};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub proof_json: String,
}

impl Proof {
    pub fn new(json: &str) -> Self {
        Self {
            proof_json: json.to_string(),
        }
    }
}

pub async fn verify_proof(proof: &Proof) -> Result<bool> {
    // Deserialize the JSON into the SDK's Proof struct
    let reclaim_proof: ReclaimProof = serde_json::from_str(&proof.proof_json)
        .map_err(|e| anyhow!("Failed to parse proof JSON: {}", e))?;
    
    // Verify using the SDK
    match sdk_verify_proof(&reclaim_proof).await {
        Ok(true) => Ok(true),
        Ok(false) => Ok(false),
        Err(e) => Err(anyhow!("Reclaim verification error: {:?}", e)),
    }
}
