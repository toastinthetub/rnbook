/*
 * src/util/command.rs
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

/// NOTE! This file is largely untouched and incomplete, eventually this will control how we call methods on State
/// and by parsing commands we can do things like state.write_loaded_entry_to_disk()
// src/util/command.rs
use std::str::FromStr;

use crate::{state::state::State, util::util::log_message};

/// Unique identifier for an entry (using the id from Entry).
pub type EntryId = String;

/// Command abstraction – each variant corresponds to a user action.
#[derive(Debug)]
pub enum Command {
    AddEntry(String),     // add a new entry with the given title/label
    DeleteEntry(EntryId), // delete the entry with the specified id
    Save,                 // save current entry (equivalent to :w)
    Quit,                 // quit
    QuitForce,            // quit without saving
    Invalid(String),      // unrecognized command.
}

impl FromStr for Command {
    type Err = ();

    /// simple parser splits the input on whitespace and matches the first token.
    ///   "add My new entry" -> Command::AddEntry("My new entry".into())
    ///   "delete <id>"     -> Command::DeleteEntry(id)
    ///   "w" or "save"     -> Command::Save
    ///   "q" or "quit"     -> Command::Quit
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.trim().splitn(2, ' ').collect();
        if tokens.is_empty() {
            return Err(());
        }
        match tokens[0].to_lowercase().as_str() {
            "add" => {
                if tokens.len() > 1 {
                    Ok(Command::AddEntry(tokens[1].to_string()))
                } else {
                    Ok(Command::AddEntry(String::new()))
                }
            }
            "delete" => {
                if tokens.len() > 1 {
                    Ok(Command::DeleteEntry(tokens[1].to_string()))
                } else {
                    Err(())
                }
            }
            "w" | "save" => Ok(Command::Save),
            "q" | "quit" => Ok(Command::Quit),
            "q!" => Ok(Command::QuitForce),
            _ => Ok(Command::Invalid(s.to_string())),
        }
    }
}

/// simple command dispatcher that acts on the State
pub struct Commander;

impl Commander {
    pub fn dispatch(cmd: Command, state: &mut State) {
        match cmd {
            Command::AddEntry(label) => {
                state.add_entry(&label);
            }
            Command::DeleteEntry(id) => {
                if let Err(e) = state.delete_entry(&id) {
                    log_message(&format!("failed to delete entry: {}", e));
                }
            }
            Command::Save => {
                if let Err(e) = state.save_current_entry() {
                    log_message(&format!("failed to save entry: {}", e));
                }
            }
            Command::Quit => {
                // TODO warn of unsaved changes
                log_message("quit command run");
                state.quit();
            }
            Command::QuitForce => {
                // TODO fix state.quit()
                log_message("quitforce command run");
                state.quit();
            }
            Command::Invalid(s) => log_message(&format!("unrecognized command: {}", s)),
        }
    }
}
