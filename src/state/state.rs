/*
 * src/state/state.rs
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

/// this file includes a lot of methods on state, though methods on state are littered kind of all over this codebase
use crossterm::{
    execute,
    terminal::{self, Clear, ClearType},
};

use crate::util::{
    command_bar::CommandBar,
    config::ENTRIES_DIR,
    mode::ModeT,
    term::DoubleBuffer,
    util::{Entry, EntryMeta, MasterIndex},
};

use std::{
    collections::HashMap,
    fs::{self},
};

#[derive(Debug, Clone)]
pub struct State {
    pub buffer: DoubleBuffer,
    pub mode: ModeT,
    pub last_mode: ModeT, // i will likely have a use for this
    pub config: crate::util::config::Config,
    pub loaded: Vec<crate::util::util::Entry>, // still used for listing entries
    pub string_buffer: Vec<String>,
    pub n_fits: u32,
    pub no_entry_flag: bool,
    pub command_bar: CommandBar,
    pub command_mode: bool,
    pub idx: u32,
    pub idx_active: bool,
    pub idx_selected: bool, // this may or may not go
    pub active_buffer: String,
    pub buffer_editable: bool,
    pub dbg: bool,

    pub master_index: MasterIndex,             // in-memory master index
    pub entries_map: HashMap<String, Entry>, // mapping from entry ID to full Entry (for quick lookup)
    pub current_entry: Option<Entry>,        // entry being edited (if any)
    pub current_entry_meta: Option<EntryMeta>, // corresponding metadata for the entry in edit mode
}

impl State {
    pub fn new(buffer: DoubleBuffer) -> Self {
        let config = crate::util::config::Config::load().unwrap();
        let n_fits: u32 = (buffer.height - 4) as u32;
        // ensure the directory exists
        fs::create_dir_all(ENTRIES_DIR).expect("Failed to create entries directory");

        // but we need a cross platform abstract function in util/config.rs for this

        Self {
            buffer,
            mode: ModeT::BROWSE,
            last_mode: ModeT::BROWSE,
            config,
            loaded: Vec::new(),
            string_buffer: Vec::new(),
            n_fits,
            no_entry_flag: true,
            command_bar: CommandBar {
                buffer: String::from("test buffer on line 82 of state.rs"),
                user_buffer: String::new(),
            },
            command_mode: false,
            idx: 0,
            idx_active: false, // we initialize this as false until self.init() at which point we set it to true if loaded.len() > 0
            idx_selected: false,
            active_buffer: String::new(),
            buffer_editable: false,
            dbg: true,

            master_index: MasterIndex::default(),
            entries_map: HashMap::new(),
            current_entry: None,
            current_entry_meta: None,
        }
    }

    /// initialize things in memory and on screen when State is instantiated. this includes
    /// setting some flags and initializing buffers and loading entries and other such bullshit.
    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = std::io::stdout();
        let _ = execute!(stdout, terminal::EnterAlternateScreen);
        let _ = execute!(stdout, crossterm::cursor::Hide);
        self.buffer.clear();
        self.buffer.flush(&mut stdout);
        let _ = terminal::enable_raw_mode();

        self.mode = ModeT::BROWSE;
        self.load_all_entries()?;

        if !self.loaded.is_empty() {
            self.no_entry_flag = false;
            self.idx_active = true;
        }

        self.populate_string_buffer();

        self.buffer.clear();
        self.buffer.resize();
        Ok(())
    }

    /// this function is a sorry excuse for an attempt at cleaning up instead of just std::process::exit(0)
    pub fn deconstruct(&mut self) {
        // TODO do something about this
        let mut stdout: std::io::Stdout = std::io::stdout();
        let _ = execute!(stdout, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
        let _ = execute!(stdout, Clear(ClearType::All));
    }

    /// this is really embarrassing
    pub fn quit(&mut self) {
        // TODO do something about this
        self.deconstruct();
        std::process::exit(0);
    }

    /// this computes the formatted label for every entry loaded in memory and
    /// loads it into `string_buffer: Vec<String>`
    ///
    /// gets called on every resize, but at least we aren't reformatting every frame
    pub fn populate_string_buffer(&mut self) {
        for entry in self.loaded.iter() {
            self.string_buffer.push(entry.stringify(self.buffer.width));
        }
    }
}
