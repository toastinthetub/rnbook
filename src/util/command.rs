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

pub struct Commander {}

// no command is to have more than 2 arguments
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Command {
    Write,
    Quit,
    New(CommandArgument),
    Shred,
    Delete,
    NULL,
}

impl Command {
    pub fn from_str(s: String) -> Option<Self> {
        let tokens: Vec<char> = s.clone().chars().collect();
        let n = tokens.len();
        if !n > 0 {
            return None;
        }

        // for tok in tokens {}

        for (i, s) in tokens.iter().enumerate() {
            match i {
                0_usize => match s {
                    ' ' => {
                        todo!()
                    }
                    _ => {
                        todo!()
                    }
                },
                1_usize => {
                    // first arg
                }
                2_usize => {
                    // second arg
                }
                _ => {
                    // do nothing
                }
            }
        }
        /*
        match s.as_str() {
            "write" | "w" => {
                //
            }

            _ => {
                return None;
            }
        } */
        None
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum CommandArgument {
    Path(String),
    Name(String),
    Entry(crate::util::util::Entry),
}

pub struct CommandParser {}
