use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf
}

use crate::util::Entry;

#[derive(Debug, Clone)]
pub struct Parser;

impl Parser {
    pub fn get_entries(&self, file_path: &PathBuf) -> Result<Vec<Entry>, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let lines = BufReader::new(file).lines();

        let mut entries = Vec::new();
        let mut label = String::new();
        let mut date = String::new();
        let mut entry_content = String::new();
        let mut in_entry = false;

        for line_result in lines {
            let line = line_result?;
            if line.trim().is_empty() && in_entry {
                entries.push(Entry {
                    label,
                    date,
                    content: entry_content.trim().to_string(),
                });
                label = String::new();
                date = String::new();
                entry_content.clear();
                in_entry = false;
            } else if !in_entry {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    label = parts[0].trim().to_string();
                    date = parts[1].trim().to_string();
                    in_entry = true;
                }
            } else {
                entry_content.push_str(&line);
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

    pub fn add_entry(&self, file_path: &PathBuf, entry: &Entry) -> Result<(), Box<dyn Error>> {
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
