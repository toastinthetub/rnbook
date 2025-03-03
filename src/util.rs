use chrono::Local;

use std::{fmt, fs::OpenOptions, io::Write};

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
    BROWSE,
    OPEN(OpenMode),
}

#[derive(Debug, Clone)]
pub struct CommandBar {
    pub user_buffer: String,
    pub buffer: String,
}

impl CommandBar {
    pub fn stringify(&mut self, x: u32) -> String {
        let mut str = String::new();
        let max_chars = x - 2;

        for (index, character) in self.buffer.char_indices() {
            if index < max_chars as usize {
                str.push(character)
            }
        }

        while str.len() < max_chars as usize {
            str.push(' ')
        }

        str
    }
    pub fn push_char(&mut self, c: char) {
        self.buffer.push(c)
    }
    pub fn push_str(&mut self, s: &str) {
        self.buffer.push_str(s)
    }
    pub fn pop_char(&mut self) {
        self.buffer.pop();
    }
    pub fn clear(&mut self) {
        self.buffer.clear()
    }
    pub fn get_buffer_contents(&self) -> String {
        self.buffer.clone()
    }
    /*
        pub fn push_char_usr(&mut self, c: char) {
            self.user_buffer.push(c);
        }
        pub fn push_str_usr(&mut self, s: &str) {
            self.user_buffer.push_str(s);
        }
        pub fn pop_char_usr(&mut self) {
            self.user_buffer.pop();
        }
        pub fn clear_usr(&mut self) {
            self.user_buffer.clear();
        }
        pub fn get_usr_buffer_contents(&self) -> String {
            self.user_buffer.clone()
        }
        pub fn push_char_sys(&mut self, c: char) {
            self.buffer.push(c);
        }
        pub fn push_str_sys(&mut self, s: &str) {
            self.buffer.push_str(s);
        }
        pub fn pop_char_sys(&mut self) {
            self.buffer.pop();
        }
        pub fn clear_sys(&mut self) {
            self.buffer.clear();
        }
        pub fn get_sys_buffer_contents(&self) -> String {
            self.buffer.clone()
        }
    */
    /// swaps the two buffers so that we can either be displaying user buf (in command mode) or sys buf which program can write to
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.buffer, &mut self.user_buffer);
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

impl fmt::Display for ModeT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModeT::BROWSE => write!(f, "BROWSE"),
            ModeT::OPEN(open_mode) => write!(f, "OPEN({})", open_mode),
        }
    }
}

impl fmt::Display for OpenMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenMode::EDIT => write!(f, "EDIT"),
            OpenMode::READ => write!(f, "READ"),
        }
    }
}
