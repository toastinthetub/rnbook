/*
 * src/state/db/db.rs
 *
 * This file is part of rnbook.
 *
 * rnbook is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * rnbook is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with rnbook. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::{
    state::{self, state::State},
    util::{
        config::{self},
        util::{log_message, Entry, EntryMeta, MasterIndex},
    },
};

use serde_json;
use std::{
    fs::{self, File},
    path::Path,
};
use uuid::Uuid;

impl state::state::State {
    /// Load the master index from disk (or create an empty one if not present)
    pub fn load_master_index(&mut self) -> std::io::Result<()> {
        let master_path = self.config.master_index_path();
        if Path::new(&master_path).exists() {
            let data = fs::read_to_string(&master_path)?;
            self.master_index =
                serde_json::from_str(&data).unwrap_or_else(|_| MasterIndex::default());
        } else {
            self.master_index = MasterIndex::default();
        }
        Ok(())
    }

    /// Save the master index to disk.
    pub fn save_master_index(&self) -> std::io::Result<()> {
        let master_path = self.config.master_index_path();
        let file = fs::File::create(&master_path)?;
        serde_json::to_writer_pretty(file, &self.master_index)?;
        Ok(())
    }

    /// Load all entries from disk into the in-memory mapping.
    /// Uses the master index (which preserves ordering) to load each entry.
    pub fn load_all_entries(&mut self) -> std::io::Result<()> {
        self.load_master_index()?;
        self.entries_map.clear();
        for meta in &self.master_index.entries {
            let file_path = self.config.entries_path.join(&meta.file);
            if file_path.exists() {
                log_message(&format!("file path {:?} exists", file_path));
                let file = fs::File::open(&file_path)?;
                if let Ok(entry) = serde_json::from_reader::<_, Entry>(file) {
                    log_message(&format!("entry {:?} inserted\n", entry));
                    self.entries_map.insert(entry.id.clone(), entry);
                }
            } else {
                log_message(&format!("file path {:?} does not exist", file_path));
            }
        }
        // set the idx_active flag based on whether there are entries.
        self.idx_active = !self.master_index.entries.is_empty();
        // reset selected index if needed.
        if self.idx >= self.master_index.entries.len() {
            self.idx = 0;
        }
        Ok(())
    }

    /// populate the command bar using the order of entries in the master index.
    /// here we build a vector of strings to display by iterating over master_index.entries
    /// and looking up the actual Entry in entries_map
    pub fn populate_string_buffer(&mut self) -> Vec<String> {
        let mut buffer = Vec::new();
        for meta in &self.master_index.entries {
            if let Some(entry) = self.entries_map.get(&meta.id) {
                buffer.push(entry.stringify(self.buffer.width));
            } else {
                buffer.push(String::from("<missing entry>"));
            }
        }
        buffer
    }

    /// add new entry in mem only
    pub fn add_entry(&mut self, label: &str) {
        use uuid::Uuid;
        let new_id = Uuid::new_v4().to_string();
        let current_date = chrono::Local::now().format("%Y/%m/%d").to_string();
        let new_entry = Entry {
            id: new_id.clone(),
            label: label.to_string(),
            date: current_date.clone(),
            content: String::new(),
            is_dirty: true,
        };
        // generate a filename for the new entry
        let file = format!("entry_{}.json", Uuid::new_v4().simple());
        let meta = EntryMeta {
            id: new_id,
            label: label.to_string(),
            date: current_date,
            file,
        };
        self.current_entry = Some(new_entry.clone());
        self.current_entry_meta = Some(meta.clone());
        self.buffer_editable = true;
        self.entries_map.insert(new_entry.id.clone(), new_entry);
        self.master_index.entries.push(meta);
    }

    /// save the currently edited entry to disk
    pub fn save_current_entry(&mut self) -> std::io::Result<()> {
        if let (Some(current), Some(meta)) = (&mut self.current_entry, &mut self.current_entry_meta)
        {
            let file_path = self.config.entries_path.join(&meta.file);
            let file = fs::File::create(&file_path)?;
            serde_json::to_writer_pretty(file, current)?;
            current.is_dirty = false;
            meta.label = current.label.clone();
            meta.date = current.date.clone();
            self.save_master_index()?;
        }
        Ok(())
    }

    /// delete an entry immediately from disk and memory
    /// `entry_id` is the identifier of the entry to delete
    pub fn delete_entry(&mut self, entry_id: &str) -> std::io::Result<()> {
        if let Some(pos) = self
            .master_index
            .entries
            .iter()
            .position(|e| e.id == entry_id)
        {
            let meta = &self.master_index.entries[pos];
            let file_path = self.config.entries_path.join(&meta.file);
            if file_path.exists() {
                fs::remove_file(&file_path)?;
            }
            self.master_index.entries.remove(pos);
            self.save_master_index()?;
            self.entries_map.remove(entry_id);
            if let Some(current) = &self.current_entry {
                if current.id == entry_id {
                    self.current_entry = None;
                    self.current_entry_meta = None;
                }
            }
        }
        Ok(())
    }

    /// write all entries (as given in the master index) to disk
    /// useful for bulk :w command or on exit
    pub fn write_loaded_entries_to_disk(&self) -> std::io::Result<()> {
        for meta in &self.master_index.entries {
            if let Some(entry) = self.entries_map.get(&meta.id) {
                let file_path = self.config.entries_path.join(&meta.file);
                let file = fs::File::create(&file_path)?;
                serde_json::to_writer_pretty(file, entry)?;
            }
        }
        self.save_master_index()?;
        Ok(())
    }
}
