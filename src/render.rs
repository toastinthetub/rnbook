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
            self.buffer.flush(stdout);
            return Ok(());
        }
        if self.mode == ModeT::OPEN(OpenMode::READ) {
            // TODO self.write_stuff()
            return Ok(());
        } else if self.mode == ModeT::BROWSE {
            self.write_loaded_entries();
            self.buffer.flush(stdout);
            // TODO
            return Ok(());
        }
        self.write_rectangle(0, self.buffer.width - 1, 0, self.buffer.height - 1); // draws the border rectangle
        self.write_str_at((self.buffer.width / 2) - 1, self.buffer.height / 2, "X");
        self.buffer.flush(stdout);
        Ok(())
    }
}
