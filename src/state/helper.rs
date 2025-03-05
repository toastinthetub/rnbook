/*
 * src/state/helper.rs
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

use crate::state::state::State;
use crate::util::command::{Command, Commander};
use std::str::FromStr;

impl State {
    pub fn submit_command(&mut self) {
        let input = self.command_bar.get_buffer_contents();
        self.command_bar.clear();
        self.command_bar.swap();
        match input.parse::<Command>() {
            Ok(cmd) => {
                Commander::dispatch(cmd, self);
            }
            Err(_) => {
                eprintln!("Failed to parse command: {}", input);
            }
        }
        self.command_mode = false;
    }
}
