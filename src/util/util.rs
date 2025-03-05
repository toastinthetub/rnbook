/*
 * src/util/util.rs
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
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{fmt, fs::OpenOptions, io::Write};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub label: String,
    pub date: String,
    pub content: String,
    #[serde(skip)]
    pub is_dirty: bool, // runtime flag, not serialized
}

impl Entry {
    pub fn stringify(&self, total_width: usize) -> String {
        let effective_width = total_width.saturating_sub(2);

        let label_width = (effective_width * 80) / 100;
        let date_width = effective_width - label_width - 2;

        let truncated_label: String = self.label.chars().take(label_width).collect();
        let padded_label = format!("{:<width$}", truncated_label, width = label_width);

        let padded_date = format!("{:<width$}", self.date, width = date_width);

        format!(
            "{}{}{}{}",
            padded_label,
            crate::util::constant::VERTICAL_LINE,
            padded_date,
            crate::util::constant::VERTICAL_LINE,
        )
    }
    pub fn selected_stringify(&self, total_width: usize) -> String {
        let effective_width = total_width.saturating_sub(2);

        let label_width = ((effective_width * 80) / 100) - 2;
        let date_width = effective_width - label_width - 2;

        let truncated_label: String = self.label.chars().take(label_width).collect();
        let padded_label = format!("{:<width$}", truncated_label, width = label_width);

        let padded_date = format!("{:<width$}", self.date, width = date_width);

        format!(
            "{}{}{}{}",
            padded_label,
            crate::util::constant::VERTICAL_LINE,
            padded_date,
            crate::util::constant::VERTICAL_LINE,
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryMeta {
    pub id: String,
    pub label: String,
    pub date: String,
    pub file: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MasterIndex {
    pub entries: Vec<EntryMeta>,
}

/// i do not think this function will be used again for some time
pub fn log_message(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")
        .expect("Failed to open log file");

    writeln!(file, "[b] {}", message).expect("Failed to write to log file");
}
