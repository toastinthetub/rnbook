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
    util::config::{self, ENTRIES_DIR, MASTER_INDEX_FILE},
    util::util::{Entry, EntryMeta, MasterIndex},
};

use serde_json;
use std::{
    fs::{self, File},
    path::Path,
};
use uuid::Uuid;

impl state::state::State {
    /// load the master index from disk (or create an empty one if not present)
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

    /// save the master index to disk
    pub fn save_master_index(&self) -> std::io::Result<()> {
        let file = File::create(MASTER_INDEX_FILE)?;
        serde_json::to_writer_pretty(file, &self.master_index)?;
        Ok(())
    }

    /// load all entries from disk into the in-memory mapping.
    /// this uses the master index to locate and deserialize each entry.
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

    /// add a new entry in memory (modification stays unsaved until :w is executed)
    pub fn add_entry(&mut self, label: &str) {
        // Generate a UUID for the new entry
        let new_id = Uuid::new_v4().to_string();
        let current_date = chrono::Local::now().format("%Y/%m/%d").to_string();

        // create the new entry; start with empty content.
        let new_entry = Entry {
            id: new_id.clone(),
            label: label.to_string(),
            date: current_date.clone(),
            content: String::new(),
            is_dirty: true, // mark as unsaved
        };

        let meta = EntryMeta {
            id: new_id,
            label: label.to_string(),
            date: current_date,
            file: format!("entry_{}.json", Uuid::new_v4().simple()),
        };

        self.loaded.push(new_entry.clone());
        self.entries_map.insert(new_entry.id.clone(), new_entry);
        self.master_index.entries.push(meta);
    }

    pub fn save_current_entry(&mut self) -> std::io::Result<()> {
        if let (Some(current), Some(meta)) = (&mut self.current_entry, &mut self.current_entry_meta)
        {
            let file_path = Path::new(config::ENTRIES_DIR).join(&meta.file);
            let file = File::create(&file_path)?;
            serde_json::to_writer_pretty(file, current)?;
            current.is_dirty = false;

            meta.label = current.label.clone();
            meta.date = current.date.clone();

            self.save_master_index()?;
        }
        Ok(())
    }

    /// delete an entry immediately from disk and in-memory.
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

            if file_path.exists() {
                fs::remove_file(&file_path)?;
            }

            self.master_index.entries.remove(pos);
            self.save_master_index()?;

            self.entries_map.remove(entry_id);
            self.loaded.retain(|entry| entry.id != entry_id);
            if let Some(current) = &self.current_entry {
                if current.id == entry_id {
                    self.current_entry = None;
                    self.current_entry_meta = None;
                }
            }
        }
        Ok(())
    }

    /// write all loaded entries to disk.
    /// can be called on exit or via a bulk :w command.
    pub fn write_loaded_entries_to_disk(&self) -> std::io::Result<()> {
        for entry in self.entries_map.values() {
            if let Some(meta) = self.master_index.entries.iter().find(|m| m.id == entry.id) {
                let file_path = Path::new(config::ENTRIES_DIR).join(&meta.file);
                let file = File::create(&file_path)?;
                serde_json::to_writer_pretty(file, entry)?;
            }
        }

        self.save_master_index()?;
        Ok(())
    }
}
