
use burn::module::Module;
use burn::tensor::backend::Backend;
use burn::nn::{Embedding, Linear};

use crate::model::transformer::TransformerEncoder;

#[allow(dead_code)]
#[derive(Module, Debug)]
pub struct QAModel<B: Backend> {
    embedding: Embedding<B>,
    transformer: TransformerEncoder<B>,
    output: Linear<B>,
} 