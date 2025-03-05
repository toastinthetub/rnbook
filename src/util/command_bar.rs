/*
 * src/util/command_bar.rs
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

#[derive(Debug, Clone)]
pub struct CommandBar {
    pub user_buffer: String,
    pub buffer: String,
}

impl CommandBar {
    pub fn stringify(&mut self, x: u32) -> String {
        let mut str = String::new();
        let max_chars = x - 2;

        for (index, character) in self.buffer.char_indices() {
            if index < max_chars as usize {
                str.push(character)
            }
        }

        while str.len() < max_chars as usize {
            str.push(' ')
        }

        str
    }
    pub fn push_char(&mut self, c: char) {
        self.buffer.push(c)
    }
    pub fn push_str(&mut self, s: &str) {
        self.buffer.push_str(s)
    }
    pub fn pop_char(&mut self) {
        self.buffer.pop();
    }
    pub fn clear(&mut self) {
        self.buffer.clear()
    }
    pub fn get_buffer_contents(&self) -> String {
        self.buffer.clone()
    }
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.buffer, &mut self.user_buffer);
    }
}
