use chrono::Local;

use std::{fs::OpenOptions, io::Write};

#[derive(Debug, Clone)]
pub struct Entry {
    pub label: String,
    pub date: String, // this shouldnt be a string its going to bite me later
    pub content: String,
}

impl Entry {
    pub fn from_str(label: &str) -> Self {
        let label = label.to_string();
        let date: String = Local::now().format("%Y/%m/%d").to_string();
        let content = String::new();

        Self {
            label,
            date,
            content,
        }
    }
    pub fn stringify(&self, x: usize) -> String {
        let mut str = String::new();

        let hunnid = x - 2;

        let eighty = (hunnid * 80) / 100;
        let max_num_label_chars = eighty - 1;
        let c: Vec<char> = self.label.chars().collect();

        for (index, character) in c.iter().enumerate() {
            if index < max_num_label_chars {
                str.push(*character)
            } else {
                for _ in index..=max_num_label_chars {
                    str.push(' ')
                }
            }
        }

        str.push(crate::constant::VERTICAL_LINE);
        str.push_str(&self.date);
        // let str: String = format!("{}{}{}{}", self.label) //{label}{space}{VERT_LINE}{date}{whitespace}{VERT_LINE
        str
    }
}

/*
    let now = Local::now(); // Gets the current local time
    println!("Current local time: {}", now.format("%Y-%m-%d %H:%M:%S"));
*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenMode {
    EDIT,
    READ,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeT {
    COMMAND,
    BROWSE,
    OPEN(OpenMode),
}

#[derive(Debug, Clone)]
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
