#![no_main]
sp1_zkvm::entrypoint!(main);

use sp1_zkvm::io;
use private_context_core::math::{cosine_similarity, to_fixed};
use fixed::types::I32F32;

pub fn main() {
    // Read public inputs
    // For MVP, we pass vectors directly. In prod, we'd pass hashes or encrypted data.
    let query_vec: Vec<f32> = io::read();
    let chunk_vec: Vec<f32> = io::read();
    let threshold_f32: f32 = io::read();
    // Read document hash for binding (proof of specific document)
    let document_hash: [u8; 32] = io::read();

    // Convert to fixed-point
    let query_fixed = to_fixed(&query_vec);
    let chunk_fixed = to_fixed(&chunk_vec);
    let threshold = I32F32::from_num(threshold_f32);

    // Compute similarity
    let similarity = cosine_similarity(&query_fixed, &chunk_fixed);

    // Verify condition
    let is_relevant = similarity >= threshold;

    // Commit the result (public output)
    // Order matters: must match verify_and_mint decoding
    // z-rwa expects public_values bytes. 
    // SP1 commits are appended.
    io::commit(&document_hash);
    io::commit(&is_relevant);
    
    // Also commit the similarity score for verification/debugging
    // We convert it back to f32 for output clarity, or keep as bits
    let similarity_f32: f32 = similarity.to_num();
    io::commit(&similarity_f32);
}
