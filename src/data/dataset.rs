// use crate::data::docx_loader::extract_text_from_docx;

// pub fn load_calendars() {
//     let mut combined = String::new();

//     combined.push_str(&extract_text_from_docx("calendar_2025.docx"));
//     combined.push_str(" ");
//     combined.push_str(&extract_text_from_docx("calader_2026.docx"));

//     combined
// }
use crate::data::docx_loader::extract_text_from_docx;

pub fn load_all_calendars() -> Vec<String> {
    let files = vec![
        "calendar_2024.docx",
        "calendar_2025.docx",
        "calader_2026.docx",
    ];

    let mut documents = Vec::new();

    for file in files {
        if std::path::Path::new(file).exists() {
            let text = extract_text_from_docx(file);
            documents.push(text);
        }
    }

    documents
}