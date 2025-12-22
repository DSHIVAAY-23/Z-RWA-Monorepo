use fixed::types::I32F32;

/// Calculates the dot product of two fixed-point vectors.
pub fn dot_product(a: &[I32F32], b: &[I32F32]) -> I32F32 {
    a.iter()
        .zip(b.iter())
        .fold(I32F32::from_num(0), |acc, (&x, &y)| acc + x * y)
}

/// Calculates the magnitude (Euclidean norm) of a fixed-point vector.
pub fn magnitude(v: &[I32F32]) -> I32F32 {
    let sum_sq = dot_product(v, v);
    // sqrt is available on FixedI32 type
    if sum_sq == 0 {
        return I32F32::from_num(0);
    }
    sum_sq.sqrt()
}

/// Calculates the Cosine Similarity between two fixed-point vectors.
/// Result is between -1.0 and 1.0.
pub fn cosine_similarity(a: &[I32F32], b: &[I32F32]) -> I32F32 {
    let dot = dot_product(a, b);
    let mag_a = magnitude(a);
    let mag_b = magnitude(b);

    if mag_a == 0 || mag_b == 0 {
        return I32F32::from_num(0);
    }

    dot / (mag_a * mag_b)
}

/// Helper to convert f32 slice to I32F32 slice
pub fn to_fixed(v: &[f32]) -> Vec<I32F32> {
    v.iter().map(|&x| I32F32::from_num(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = to_fixed(&[1.0, 2.0, 3.0]);
        let b = to_fixed(&[4.0, 5.0, 6.0]);
        
        let result = dot_product(&a, &b);
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert_eq!(result, I32F32::from_num(32));
    }

    #[test]
    fn test_cosine_similarity() {
        let a = to_fixed(&[1.0, 0.0, 0.0]);
        let b = to_fixed(&[1.0, 0.0, 0.0]);
        assert_eq!(cosine_similarity(&a, &b), I32F32::from_num(1));

        let c = to_fixed(&[0.0, 1.0, 0.0]);
        assert_eq!(cosine_similarity(&a, &c), I32F32::from_num(0));
    }
}
