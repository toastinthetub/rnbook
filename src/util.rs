use chrono::Local;

pub struct Entry {
    pub label: String,
    pub date: String, // this shouldnt be a string its going to bite me later
    pub content: String,
}

impl Entry {
    pub fn from_str(label: &str) -> Self {
        let label = label.to_string();
        let date: String = Local::now().format("%Y-%m-%d").to_string();
        let content = String::new();

        Self {
            label,
            date,
            content,
        }
    }
}
/*
    let now = Local::now(); // Gets the current local time
    println!("Current local time: {}", now.format("%Y-%m-%d %H:%M:%S"));
*/

pub enum OpenMode {
    EDIT,
    READ,
}

pub enum ModeT {
    BROWSE,
    OPEN(OpenMode),
}
