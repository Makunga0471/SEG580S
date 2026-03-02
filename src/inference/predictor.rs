

// use crate::training::trainer::load_calendars;

// pub fn predict(question: &str) {
//     let data: String = String::from_utf8(load_calendars()).expect("Invalid UTF-8");
//     let question_lower = question.to_lowercase();

//     for sentence in data.split('.') {
//         if sentence
//             .to_ascii_lowercase()
//             .contains(&question_lower)
//         {
//             println!("\nAnswer Found:\n{}\n", sentence.trim());
//             return;
//         }
//     }

//     println!("No direct answer found in calendars.");
// }

use crate::{model::embeddings::text_to_embedding, training::trainer::load_all_calendars, utils::similarity::cosine_similarity};

pub fn predict(question: &str) {
    let documents = load_all_calendars();

    if documents.is_empty() {
        println!("No calendar documents found.");
        return;
    }

    let question_embedding = text_to_embedding(question);

    let mut best_score = -1.0;
    let mut best_answer: String = String::new();

    for doc in documents {
        let sentences: Vec<&str> = doc.split('.').collect();

        for sentence in sentences {
            let sentence_embedding = text_to_embedding(sentence);
            let score = cosine_similarity(
                question_embedding.clone(),
                sentence_embedding,
            );

            if score > best_score {
                best_score = score;
                best_answer = sentence.trim().to_string();
            }
        }
    }

    println!("\nBest Match (score {:.3}):\n{}\n", best_score, best_answer);
}