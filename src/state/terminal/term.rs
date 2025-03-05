/*
 * src/state/terminal/mod.rs
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

use crossterm::style::Color;

use crate::util::constant::{
    HORIZONTAL_LINE, LEFT_LOWER_SHOULDER, LEFT_UPPER_SHOULDER, NO_ENTRIES_WARNING,
    RIGHT_LOWER_SHOULDER, RIGHT_UPPER_SHOULDER, TOO_SMALL_WARNING, VERTICAL_LINE,
};

impl crate::state::state::State {
    /// this function needs to be flushed.
    pub fn write_line_horizontal(&mut self, x_0: usize, x: usize, y: usize) {
        for i in x_0..=x {
            self.buffer.write(i, y, HORIZONTAL_LINE)
        }
    }

    pub fn write_colored_line_horizontal(
        &mut self,
        x_0: usize,
        x: usize,
        y: usize,
        fg: Color,
        bg: Color,
    ) {
        for i in x_0..=x {
            self.buffer.write_colored(i, y, HORIZONTAL_LINE, fg, bg);
        }
    }

    pub fn write_line_vertical(&mut self, x: usize, y_0: usize, y: usize) {
        for i in y_0..=y {
            self.buffer.write(x, i, VERTICAL_LINE)
        }
    }

    pub fn write_colored_line_vertical(
        &mut self,
        x: usize,
        y_0: usize,
        y: usize,
        fg: Color,
        bg: Color,
    ) {
        for i in y_0..=y {
            self.buffer.write_colored(x, i, VERTICAL_LINE, fg, bg)
        }
    }

    pub fn write_rectangle(&mut self, x_0: usize, x: usize, y_0: usize, y: usize) {
        self.buffer.write(x_0, y_0, LEFT_UPPER_SHOULDER);
        self.buffer.write(x, y_0, RIGHT_UPPER_SHOULDER);
        self.buffer.write(x_0, y, LEFT_LOWER_SHOULDER);
        self.buffer.write(x, y, RIGHT_LOWER_SHOULDER);

        self.write_line_horizontal(x_0 + 1, x - 1, y_0);
        self.write_line_horizontal(x_0 + 1, x - 1, y);
        self.write_line_vertical(x_0, y_0 + 1, y - 1);
        self.write_line_vertical(x, y_0 + 1, y - 1);
    }

    pub fn write_colored_rectangle(
        &mut self,
        x_0: usize,
        x: usize,
        y_0: usize,
        y: usize,
        fg: Color,
        bg: Color,
    ) {
        self.buffer
            .write_colored(x_0, y_0, LEFT_UPPER_SHOULDER, fg, bg);
        self.buffer
            .write_colored(x, y_0, RIGHT_UPPER_SHOULDER, fg, bg);
        self.buffer
            .write_colored(x_0, y, LEFT_LOWER_SHOULDER, fg, bg);
        self.buffer
            .write_colored(x, y, RIGHT_LOWER_SHOULDER, fg, bg);

        self.write_colored_line_horizontal(x_0 + 1, x - 1, y_0, fg, bg);
        self.write_colored_line_horizontal(x_0 + 1, x - 1, y, fg, bg);
        self.write_colored_line_vertical(x_0, y_0 + 1, y - 1, fg, bg);
        self.write_colored_line_vertical(x, y_0 + 1, y - 1, fg, bg);
    }
    pub fn write_char_horizontal(&mut self, x_0: usize, x: usize, y: usize, ch: char) {
        for i in x_0..=x {
            self.buffer.write(i, y, ch)
        }
    }

    // i dont think either of the above or below methods r ever getting used ever.

    pub fn write_char_vertical(&mut self, x: usize, y_0: usize, y: usize, ch: char) {
        for i in y_0..=y {
            self.buffer.write(x, i, ch)
        }
    }
    pub fn write_str_at(&mut self, x: usize, y: usize, str: &str) {
        for (i, ch) in str.char_indices() {
            self.buffer.write(x + i, y, ch);
        }
    }
    pub fn write_colored_str_at(&mut self, x: usize, y: usize, str: &str, fg: Color, bg: Color) {
        for (i, ch) in str.char_indices() {
            self.buffer.write_colored(x + i, y, ch, fg, bg)
        }
    }
    pub fn write_too_small_warning(&mut self) {
        //        self.buffer.clear(); we do not need to do this
        self.write_rectangle(0, self.buffer.width - 1, 0, self.buffer.height - 1);
        self.write_str_at(
            (self.buffer.width / 2) - (TOO_SMALL_WARNING.len() / 2),
            self.buffer.height / 2,
            TOO_SMALL_WARNING,
        );
    }
    pub fn write_no_entries_warning(&mut self) {
        self.write_rectangle(0, self.buffer.width - 1, 0, self.buffer.height - 1);
        self.write_str_at(
            (self.buffer.width / 2) - (NO_ENTRIES_WARNING.len() / 2),
            self.buffer.height / 2,
            NO_ENTRIES_WARNING,
        );
    }

    pub fn write_command_bar(&mut self) {
        let str = self.command_bar.stringify(self.buffer.width as u32);
        self.write_colored_str_at(1, 1, &str, Color::Black, Color::White)
    }

    pub fn write_command_window(&mut self) {
        let str = self.command_bar.stringify(self.buffer.width as u32 / 2);
        // this should be buffered
        let n = str.len();
        let effective_width = n + 2; // one on each side
        self.write_colored_rectangle(
            (self.buffer.width / 2) - (effective_width / 2),
            (self.buffer.width / 2) + (effective_width / 2),
            (self.buffer.height / 2) - 1,
            (self.buffer.height / 2) + 1,
            Color::Cyan,
            Color::Black,
        );
        self.write_colored_str_at(
            (self.buffer.width / 2) - (effective_width / 2) + 1,
            self.buffer.height / 2,
            &str,
            Color::Green,
            Color::Black,
        );
    }

    pub fn write_loaded_entries(&mut self) {
        let num_entries = self.entries_map.len();
        let mut idx: u32 = 0;
        let max_idx: u32 = self.buffer.height as u32 - 4;

        if num_entries > 0 {
            for i in 0..=std::cmp::min(self.n_fits, num_entries as u32) - 1 {
                if i < max_idx {
                    let default_entry = crate::util::util::Entry {
                        label: String::from("it didnt work"),
                        date: String::from("datedate"),
                        content: String::from("some content"),
                        id: String::from("cum"),
                        is_dirty: false,
                    }; // TODO do something about this abomination

                    let entry_meta = self.master_index.entries.get(i as usize).unwrap();
                    let entry = self
                        .entries_map
                        .get(&entry_meta.id)
                        .unwrap_or(&default_entry);

                    let entry_string = if idx == self.idx as u32 {
                        format!("> {}", entry.selected_stringify(self.buffer.width))
                    } else {
                        entry.stringify(self.buffer.width)
                    };

                    self.write_str_at(2, idx as usize + 2, &entry_string);
                    self.buffer.write_colored(
                        self.buffer.width - 1,
                        idx as usize + 2,
                        VERTICAL_LINE,
                        Color::White,
                        Color::Black,
                    ); // dirty fucking hackc but should work
                       // lol it worked
                    idx += 1;
                }
            }
        } else {
            self.no_entry_flag = true;
            self.write_no_entries_warning();
        }
    }

    pub fn write_active_buffer(&mut self) {
        let effective_width = self.buffer.width - 2;
        let x_0 = 2;
        let x = self.buffer.width - 2;
        let y_0 = 2;
        let y = self.buffer.height - 2;

        let s = String::new();

        if !self.active_buffer.is_empty() {
            //for ()
            /*
            let mut lines: Vec<&str> = self.active_buffer.lines().collect();
            for (i, line) in lines.iter_mut().enumerate() {
                if line.len() >= effective_width {
                    let split: (&str, &str) = line.split_at(effective_width - 1);
                    let editable_line = lines.get_mut(i + 1).unwrap_or({
                        &mut s.as_str()
                    })
                }

            } */
        }
    }

    pub fn write_debug_info(&mut self) {
        let str_start_x = self.buffer.width - 19;
        let str_start_y = self.buffer.height - 11;

        self.write_colored_rectangle(
            self.buffer.width - 20,
            self.buffer.width - 2,
            self.buffer.height - 12,
            self.buffer.height - 2,
            Color::Yellow,
            Color::Black,
        );
        self.write_colored_str_at(str_start_x, str_start_y, "DBG!", Color::Red, Color::Black);
        self.write_colored_str_at(
            str_start_x,
            str_start_y + 1,
            &format!("(w, h): ({}, {})", self.buffer.width, self.buffer.height),
            Color::Green,
            Color::Black,
        );
        self.write_colored_str_at(
            str_start_x,
            str_start_y + 2,
            &format!("mode: {}", self.mode),
            Color::Green,
            Color::Black,
        );
        self.write_colored_str_at(
            str_start_x,
            str_start_y + 3,
            &format!("cmd: {}", self.command_mode),
            Color::Green,
            Color::Black,
        );
        self.write_colored_str_at(
            str_start_x,
            str_start_y + 4,
            &format!("nld: {}", self.entries_map.len()),
            Color::Green,
            Color::Black,
        );
        self.write_colored_str_at(
            str_start_x,
            str_start_y + 5,
            &format!("idx: {}", self.idx),
            Color::Green,
            Color::Black,
        );
        self.write_colored_str_at(
            str_start_x,
            str_start_y + 6,
            &format!("dxa: {}", self.idx_active),
            Color::Green,
            Color::Black,
        );

        self.write_colored_str_at(
            str_start_x,
            str_start_y + 7,
            &format!(
                "bn6: {}",
                self.active_buffer.chars().take(6).collect::<String>()
            ),
            Color::Green,
            Color::Black,
        );

        self.write_colored_str_at(
            str_start_x,
            str_start_y + 8,
            &format!("bed: {}", self.buffer_editable),
            Color::Green,
            Color::Black,
        );
    }
}
