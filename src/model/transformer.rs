use burn::module::Module;
use burn::tensor::backend::Backend;

#[allow(dead_code)]
#[derive(Module, Debug)]
pub struct TransformerEncoder<B: Backend> {
    _phantom: core::marker::PhantomData<B>,
}