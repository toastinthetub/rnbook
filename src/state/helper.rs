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

use super::state::State;

impl State {
    pub fn submit_command(&mut self) {
        let mut buf = self.command_bar.get_buffer_contents();
        buf.truncate(2);
        self.command_bar.clear();
        self.command_bar.swap();
        match buf.as_str() {
            "wq" => {
                self.command_bar.clear();
                self.command_bar.push_str("doesnt work yet <sadge :(>")
            }
            "w" => {
                self.command_bar.clear();
                self.command_bar
                    .push_str("lol. no writing to disk happening here")
            }
            "q" => {
                self.quit();
            }
            _ => {}
        }
        self.command_mode = false;
    }
}
