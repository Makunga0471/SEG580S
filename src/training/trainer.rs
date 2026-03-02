
pub(crate) fn load_all_calendars() -> Vec<String> {
    vec![
        String::from("The meeting is scheduled for 3 PM."),
        String::from("Please review the quarterly report."),
        String::from("The deadline for submissions is Friday."),
    ]
}

pub fn train() {
    println!("Loading documents...");
    let docs = load_all_calendars();
    println!("Loaded {} documents.", docs.len());

    println!("Preprocessing complete.");
}