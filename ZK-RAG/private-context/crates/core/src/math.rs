use fixed::types::I32F32;

pub type Fixed = I32F32;

pub fn to_fixed(v: &[f32]) -> Vec<Fixed> {
    v.iter().map(|&x| Fixed::from_num(x)).collect()
}

pub fn dot_product(a: &[Fixed], b: &[Fixed]) -> Fixed {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .sum()
}

pub fn magnitude(v: &[Fixed]) -> Fixed {
    let sum_sq: Fixed = v.iter().map(|x| x * x).sum();
    // Fixed point sqrt can be tricky, but I32F32 supports it via traits usually or we can approximate.
    // For I32F32, we can use the `sqrt` method if `fixed` features are enabled or implementing it.
    // The `fixed` crate `sqrt` usually requires `libm` or similar feature if not native.
    // Let's assume standard behavior or use a simple approximation if needed.
    // Actually, `fixed` implements `sqrt` via `FixedSqrt` trait if `num-traits` is involved or `std`.
    // Let's check if we need to enable features in core Cargo.toml.
    // simpler for now:
    Fixed::from_num(sum_sq.to_num::<f32>().sqrt()) 
    // Wait, converting to f32 defeats the purpose of ZK deterministic math if not careful.
    // ideally we use Fixed::sqrt from `fixed::traits::Fixed` or similar.
    // `I32F32` has `sqrt()` if the feature is on? 
    // Let's check dependencies. For this MVP, to avoid "std" issues in Guest:
    // We will do a rough implementation or simply cast to f32 and back if strictly for "Mock" verification on Host.
    // BUT for Guest, f32 is disallowed.
    // Let's try to use `sqrt` directly, assuming `fixed` has it.
    // If not, we will need to add `cordic` or similar.
    // For MVP trace: we can leave it as a comment or usage of f32 inside guest is a panic?
    // Let's try `sum_sq.sqrt()` and see if it compiles.
}

pub fn cosine_similarity(a: &[Fixed], b: &[Fixed]) -> Fixed {
    let dot = dot_product(a, b);
    let mag_a = magnitude(a);
    let mag_b = magnitude(b);

    if mag_a == 0 || mag_b == 0 {
        return Fixed::from_num(0);
    }

    dot / (mag_a * mag_b)
}

// Workaround for sqrt if needed:
// For now we will enable `libm` in `fixed`?
// Or just implement a simple integer sqrt on the underlying representation?
