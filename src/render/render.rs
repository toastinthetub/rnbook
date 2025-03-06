/*
 * src/render/render.rs
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
    state::state::State,
    util::mode::{ModeT, OpenMode},
};

use std::io::Write;

impl State {
    pub fn render(&mut self, stdout: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
        if self.buffer.too_small_flag {
            // log_message("too_small_warning!");
            self.write_too_small_warning();
            self.defaults();
            self.buffer.flush(stdout);
            return Ok(());
        }
        if self.mode == ModeT::OPEN(OpenMode::READ) {
            // TODO self.write_stuff()
            self.defaults();
            self.buffer.flush(stdout);
            return Ok(());
        } else if self.mode == ModeT::OPEN(OpenMode::EDIT) {
            // write the window and the buffer
            // i need to put a dummy function in for the moment
        } else if self.mode == ModeT::BROWSE {
            self.write_loaded_entries();
            self.defaults();
            self.buffer.flush(stdout);
            // TODO
            return Ok(());
        } else {
            self.defaults()
        }

        self.buffer.flush(stdout);
        Ok(())
    }
    pub fn defaults(&mut self) {
        self.write_rectangle(0, self.buffer.width - 1, 0, self.buffer.height - 1);

        if !self.buffer.too_small_flag {
            self.write_command_bar(); // the command bar will soon be slaughtered
        }
        if self.command_mode && !self.buffer.too_small_flag {
            self.write_command_window();
        }
        if self.dbg && !self.buffer.too_small_flag {
            self.write_debug_info()
        }
        // draws the border rectangle
    }
}
