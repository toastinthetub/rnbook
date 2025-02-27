use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

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
    COMMAND,
    BROWSE,
    OPEN(OpenMode),
}

pub struct CommandBar {
    pub mode: String,
    pub buffer: String,
}

impl CommandBar {
    pub fn str(&mut self) -> String {
        let s = String::new();
        s
    }
}

pub fn log_message(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")
        .expect("Failed to open log file");

    writeln!(file, "[{}] {}", 'b', message).expect("Failed to write to log file");
}
