use burn::{prelude::ToElement, tensor::{Tensor, backend::Backend}};

pub fn cosine_similarity<B: Backend>(
    a: Tensor<B, 1>,
    b: Tensor<B, 1>,
) -> f32 {
    let dot = (a.clone() * b.clone()).sum().into_scalar();
    let norm_a = (a.clone() * a).sum().into_scalar().to_f32().sqrt();
    let norm_b = (b.clone() * b).sum().into_scalar().to_f32().sqrt();

    dot.to_f32() / (norm_a * norm_b + 1e-8)
}