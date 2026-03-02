# SEG580S Assignment 1 – Intelligent Calendar Question Answering System

This project is a Rust-based command-line application that demonstrates how Artificial Intelligence concepts can be applied to a simple Question Answering (QA) system over textual calendar data. The system allows users to train a basic model structure and then ask natural language questions about stored calendar information such as meeting dates, deadlines, and project-related events.

The application is built using the Rust programming language and the Burn machine learning framework. The goal of the project is not to build a production-ready AI model, but to showcase the architecture and workflow of an AI-powered system, including data loading, embedding generation, similarity comparison, and inference. The project also demonstrates modular Rust design with separate components for training, inference, models, utilities, and datasets.

### Features
- Command-line interface (CLI) with two main commands:
  - `train` – initializes the model architecture and simulates a training pipeline.
  - `ask "<question>"` – searches calendar documents and returns the most relevant answer.
- Text embedding and cosine similarity for semantic matching.
- Modular project structure following Rust best practices.
- Extensible design for future improvements such as real training, model saving/loading, and GPU support.

### Example Usage

```bash
cargo run -- train
cargo run -- ask "When is the next meeting?"
