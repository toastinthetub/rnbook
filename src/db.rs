use crate::{
    config::{self, Config, ENTRIES_DIR, MASTER_INDEX_FILE},
    util::{log_message, CommandBar, Entry, EntryMeta, MasterIndex, ModeT, OpenMode},
};

use serde_json;
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{stdout, BufRead, BufReader, Write},
    path::Path,
    time::{Duration, Instant},
};
use uuid::Uuid;

impl crate::state::State {
    /// Load the master index from disk (or create an empty one if not present)
    pub fn load_master_index(&mut self) -> std::io::Result<()> {
        if Path::new(MASTER_INDEX_FILE).exists() {
            let data = fs::read_to_string(MASTER_INDEX_FILE)?;
            self.master_index =
                serde_json::from_str(&data).unwrap_or_else(|_| MasterIndex::default());
        } else {
            self.master_index = MasterIndex::default();
        }
        Ok(())
    }

    /// Save the master index to disk
    pub fn save_master_index(&self) -> std::io::Result<()> {
        let file = File::create(MASTER_INDEX_FILE)?;
        serde_json::to_writer_pretty(file, &self.master_index)?;
        Ok(())
    }

    /// Load all entries from disk into the in-memory mapping.
    /// This uses the master index to locate and deserialize each entry.
    pub fn load_all_entries(&mut self) -> std::io::Result<()> {
        self.load_master_index()?;
        self.loaded.clear();
        self.entries_map.clear();
        for meta in &self.master_index.entries {
            let file_path = Path::new(ENTRIES_DIR).join(&meta.file);
            if file_path.exists() {
                let file = File::open(&file_path)?;
                if let Ok(entry) = serde_json::from_reader::<_, Entry>(file) {
                    self.loaded.push(entry.clone());
                    self.entries_map.insert(entry.id.clone(), entry);
                }
            }
        }
        self.no_entry_flag = self.loaded.is_empty();
        Ok(())
    }

    /// Add a new entry in memory (modification stays unsaved until :w is executed)
    pub fn add_entry(&mut self, label: &str) {
        // Generate a UUID for the new entry
        let new_id = Uuid::new_v4().to_string();
        let current_date = chrono::Local::now().format("%Y/%m/%d").to_string();

        // Create the new entry; start with empty content.
        let new_entry = Entry {
            id: new_id.clone(),
            label: label.to_string(),
            date: current_date.clone(),
            content: String::new(),
            is_dirty: true, // mark as unsaved
        };

        // Create corresponding metadata
        let meta = EntryMeta {
            id: new_id,
            label: label.to_string(),
            date: current_date,
            file: format!("entry_{}.json", Uuid::new_v4().simple()), // new file name using UUID
        };

        // Update in-memory data structures
        self.loaded.push(new_entry.clone());
        self.entries_map.insert(new_entry.id.clone(), new_entry);
        self.master_index.entries.push(meta);
    }

    /// Save the currently edited entry to disk (triggered by :w)
    pub fn save_current_entry(&mut self) -> std::io::Result<()> {
        if let (Some(current), Some(meta)) = (&mut self.current_entry, &mut self.current_entry_meta)
        {
            // Construct the file path from config and meta.file
            let file_path = Path::new(config::ENTRIES_DIR).join(&meta.file);
            let file = File::create(&file_path)?;
            serde_json::to_writer_pretty(file, current)?;
            current.is_dirty = false;

            // Update metadata if necessary (e.g., label may have changed)
            meta.label = current.label.clone();
            meta.date = current.date.clone();

            // Update master index on disk
            self.save_master_index()?;
        }
        Ok(())
    }

    /// Delete an entry immediately from disk and in-memory.
    /// `entry_id` is the UUID of the entry to delete.
    pub fn delete_entry(&mut self, entry_id: &str) -> std::io::Result<()> {
        if let Some(pos) = self
            .master_index
            .entries
            .iter()
            .position(|e| e.id == entry_id)
        {
            let meta = &self.master_index.entries[pos];
            let file_path = Path::new(config::ENTRIES_DIR).join(&meta.file);
            // Remove file from disk (if it exists)
            if file_path.exists() {
                fs::remove_file(&file_path)?;
            }
            // Remove from master index
            self.master_index.entries.remove(pos);
            self.save_master_index()?;

            // Remove from in-memory collections
            self.entries_map.remove(entry_id);
            self.loaded.retain(|entry| entry.id != entry_id);
            // Clear current entry if it was deleted
            if let Some(current) = &self.current_entry {
                if current.id == entry_id {
                    self.current_entry = None;
                    self.current_entry_meta = None;
                }
            }
        }
        Ok(())
    }

    /// Write all loaded entries to disk.
    /// This can be called on exit or via a bulk :w command.
    pub fn write_loaded_entries(&self) -> std::io::Result<()> {
        for entry in self.entries_map.values() {
            // Find corresponding metadata to get the file name.
            if let Some(meta) = self.master_index.entries.iter().find(|m| m.id == entry.id) {
                let file_path = Path::new(config::ENTRIES_DIR).join(&meta.file);
                let file = File::create(&file_path)?;
                serde_json::to_writer_pretty(file, entry)?;
            }
        }
        // Finally, write out the master index
        self.save_master_index()?;
        Ok(())
    }

    // ... keep your existing methods such as init, event_loop, render, etc.
    // (They should call load_all_entries() instead of the old parser get_entries.)
}
