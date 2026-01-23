//! High-performance mathematical operations
//! Optimized for auto-vectorization by LLVM

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MathError {
    #[error("Dimension mismatch: {0} vs {1}")]
    DimensionMismatch(usize, usize),
    #[error("Zero magnitude vector")]
    ZeroMagnitude,
}

/// Cosine similarity between two vectors.
/// Returns value in range [-1, 1] where 1 = identical direction.
///
/// Uses iterator chaining for SIMD auto-vectorization.
#[inline]
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> Result<f32, MathError> {
    if a.len() != b.len() {
        return Err(MathError::DimensionMismatch(a.len(), b.len()));
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if mag_a == 0.0 || mag_b == 0.0 {
        return Err(MathError::ZeroMagnitude);
    }

    Ok(dot / (mag_a * mag_b))
}

/// Euclidean (L2) distance between two vectors.
/// Returns non-negative value where 0 = identical vectors.
#[inline]
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32, MathError> {
    if a.len() != b.len() {
        return Err(MathError::DimensionMismatch(a.len(), b.len()));
    }

    let sum: f32 = a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum();

    Ok(sum.sqrt())
}

/// Dot product of two vectors.
#[inline]
pub fn dot_product(a: &[f32], b: &[f32]) -> Result<f32, MathError> {
    if a.len() != b.len() {
        return Err(MathError::DimensionMismatch(a.len(), b.len()));
    }

    Ok(a.iter().zip(b.iter()).map(|(x, y)| x * y).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_identical() {
        let v = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&v, &v).unwrap();
        assert!((sim - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_identical() {
        let v = vec![1.0, 2.0, 3.0];
        let dist = euclidean_distance(&v, &v).unwrap();
        assert!(dist.abs() < 1e-6);
    }
}
