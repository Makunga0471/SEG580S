

use burn::tensor::Tensor;
use burn::backend::ndarray::NdArray;

pub type MyBackend = NdArray<f32>;

pub fn text_to_embedding(text: &str) -> Tensor<MyBackend, 1> {
    let mut values = vec![0.0f32; 128];

    for (i, byte) in text.bytes().enumerate() {
        let index = i % 128;
        values[index] += byte as f32 / 255.0;
    }

    let device = Default::default();

    // pass a slice (TensorData::From<&[E]>) and return the tensor
    Tensor::<MyBackend, 1>::from_floats(values.as_slice(), &device)
}