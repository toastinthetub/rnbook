use crate::util::Entry;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};

pub struct Parser {
    pub file: File,
}

impl Parser {
    pub fn new(f: File) -> Self {
        Self { file: f }
    }

    pub fn get_entries(&mut self) -> Result<Vec<Entry>, Box<dyn Error>> {
        let mut content = String::new();
        self.file.read_to_string(&mut content)?;

        let mut entries = Vec::new();
        let mut label = String::new();
        let mut date = String::new();
        let mut entry_content = String::new();
        let mut in_entry = false;

        for line in content.lines() {
            if line.trim().is_empty() {
                if in_entry {
                    entries.push(Entry {
                        label: label.clone(),
                        date: date.clone(),
                        content: entry_content.trim().to_string(),
                    });
                    in_entry = false;
                    label.clear();
                    date.clear();
                    entry_content.clear();
                }
            } else if !in_entry {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    label = parts[0].trim().to_string();
                    date = parts[1].trim().to_string();
                    in_entry = true;
                }
            } else {
                entry_content.push_str(line);
                entry_content.push('\n');
            }
        }

        if in_entry {
            entries.push(Entry {
                label,
                date,
                content: entry_content.trim().to_string(),
            });
        }

        Ok(entries)
    }
    pub fn add_entry(&self, file_path: &str, entry: &Entry) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        writeln!(file, "{}: {}", entry.label, entry.date)?;
        writeln!(file, "{}", entry.content)?;
        writeln!(file)?;

        Ok(())
    }
}
