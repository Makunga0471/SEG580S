// src/backend.rs
use burn::backend::ndarray::NdArrayBackend;

// Choose f32 or f64 depending on your needs
pub type Backend = NdArrayBackend<f32>;