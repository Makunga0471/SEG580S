use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use quick_xml::Reader;
use quick_xml::events::Event;

pub fn extract_text_from_docx(path: &str) -> String {
    let file = File::open(path).expect("Cannot open file");
    let mut archive = ZipArchive::new(file).expect("Invalid docx file");

    let mut document_xml = String::new();
    archive
        .by_name("word/document.xml")
        .expect("Cannot find document.xml")
        .read_to_string(&mut document_xml)
        .unwrap();

    let mut reader = Reader::from_str(&document_xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut text = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                text.push_str(&e.unescape().unwrap());
                text.push(' ');
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }

    text
}