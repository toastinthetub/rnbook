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
    /// this function assumes that there is a rectangle surrounding the
    /// terminal window and will operate with |x| = x - 2
    pub fn entry_preview(&mut self, x: usize) -> String {
        /*let mut s: String = */
        String::new()

        // TODO
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
