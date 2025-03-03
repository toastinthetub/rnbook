use crate::{
    state::State,
    util::{log_message, ModeT, OpenMode},
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
        } else if self.mode == ModeT::BROWSE {
            self.defaults();
            self.write_loaded_entries();
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
        self.write_command_bar();
        if self.dbg {
            self.write_debug_info()
        }
        // draws the border rectangle
    }
}
