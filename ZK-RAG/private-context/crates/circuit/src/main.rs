#![no_main]
sp1_zkvm::entrypoint!(main);

use private_context_core::math::{cosine_similarity, to_fixed, Fixed};

pub fn main() {
    // Read inputs: Query Vector, Chunk Vector, Threshold
    // We read them as Vec<f32> for simplicity in IO, then convert to Fixed.
    // Ideally we pass Fixed directly but serde support for Fixed is binary/specific.
    // Let's assume we pass Vec<f32> and convert inside guest (costly but simple for MVP).
    // Or better: Pass Vec<Fixed> if serde works.
    
    let query_vec: Vec<f32> = sp1_zkvm::io::read();
    let chunk_vec: Vec<f32> = sp1_zkvm::io::read();
    let threshold_f32: f32 = sp1_zkvm::io::read();

    let query_fixed = to_fixed(&query_vec);
    let chunk_fixed = to_fixed(&chunk_vec);
    let threshold = Fixed::from_num(threshold_f32);

    let similarity = cosine_similarity(&query_fixed, &chunk_fixed);

    // Commit boolean result
    let is_relevant = similarity >= threshold;
    sp1_zkvm::io::commit(&is_relevant);
    
    // Commit similarity for debugging/verification
    let sim_f32: f32 = similarity.to_num();
    sp1_zkvm::io::commit(&sim_f32);
}
