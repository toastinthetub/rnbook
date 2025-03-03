use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::util::Entry;

pub struct Parser;

const START_TOKEN: &str = "<ENTRY_START_abcd1234>";
const END_TOKEN: &str = "<ENTRY_END_abcd1234>";

impl Parser {
    pub fn get_entries(&self, file_path: &PathBuf) -> Result<Vec<Entry>, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut entries = Vec::new();

        let mut in_entry = false;
        let mut in_content = false;

        let mut label = String::new();
        let mut date = String::new();
        let mut content = String::new();

        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed = line.trim();

            if trimmed == START_TOKEN {
                in_entry = true;
                in_content = false;
                label.clear();
                date.clear();
                content.clear();
                continue;
            } else if trimmed == END_TOKEN {
                if in_entry {
                    entries.push(Entry {
                        label: label.clone(),
                        date: date.clone(),
                        content: content.trim_end().to_string(),
                    });
                }
                in_entry = false;
                in_content = false;
                continue;
            }

            if !in_entry {
                continue;
            }

            if line.starts_with("LABEL:") {
                label = line["LABEL:".len()..].trim().to_string();
                in_content = false;
            } else if line.starts_with("DATE:") {
                date = line["DATE:".len()..].trim().to_string();
                in_content = false;
            } else if line.starts_with("CONTENT:") {
                in_content = true;
            } else if in_content {
                content.push_str(&line);
                content.push('\n');
            }
        }

        Ok(entries)
    }

    pub fn add_entry(&self, file_path: &PathBuf, entry: &Entry) -> Result<(), Box<dyn Error>> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        writeln!(file, "{}", START_TOKEN)?;

        writeln!(file, "LABEL: {}", entry.label)?;
        writeln!(file, "DATE: {}", entry.date)?;
        writeln!(file, "CONTENT:")?;

        for line in entry.content.lines() {
            writeln!(file, "{}", line)?;
        }

        writeln!(file, "{}", END_TOKEN)?;
        writeln!(file)?;

        Ok(())
    }
}
