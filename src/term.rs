use crossterm::{
    cursor, execute,
    terminal::{self},
};

use crate::constant::*;
use std::{collections::HashMap, io::Write};

const TOO_SMALL_WARNING: &str = "> 60x4 TERM SIZE REQUIRED";
const NO_ENTRIES_WARNING: &str = "< not an entry to be found :) >";

#[derive(Debug, Clone)]
pub struct DoubleBuffer {
    front_buffer: HashMap<(usize, usize), char>,
    back_buffer: HashMap<(usize, usize), char>,
    color_buffer: HashMap<(usize, usize), char>,
    pub width: usize,
    pub height: usize,
    pub too_small_flag: bool,
}

impl DoubleBuffer {
    pub fn new() -> Self {
        let mut too_small_flag: bool = false;
        let (width, height) = terminal::size().unwrap();
        if width < 60 || height < 4 {
            too_small_flag = true;
        }
        Self {
            front_buffer: HashMap::new(),
            back_buffer: HashMap::new(),
            color_buffer: HashMap::new(),
            width: width as usize,
            height: height as usize,
            too_small_flag,
        }
    }

    /// recalculate terminal size in case of resize
    pub fn resize(&mut self) {
        let (new_width, new_height) = terminal::size().unwrap();
        self.too_small_flag = new_width < 60 || new_height < 4;

        // If the size changed, clear & reset the buffers
        if self.width != new_width as usize || self.height != new_height as usize {
            self.width = new_width as usize;
            self.height = new_height as usize;

            // Reset buffers
            self.front_buffer.clear();
            self.back_buffer.clear();

            // Fill back buffer with spaces to prevent garbage display
            for y in 0..self.height {
                for x in 0..self.width {
                    self.back_buffer.insert((x, y), ' ');
                }
            }
        }
        self.flush(&mut std::io::stdout());
    }
    /// write to the **back buffer**
    pub fn write(&mut self, x: usize, y: usize, ch: char) {
        if x < self.width && y < self.height {
            self.back_buffer.insert((x, y), ch);
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.back_buffer.insert((x, y), ' ');
            }
        }
    }

    /// flushes only the **changed** characters to the screen
    pub fn flush(&mut self, stdout: &mut impl Write) {
        for (&pos, &ch) in &self.back_buffer {
            if self.front_buffer.get(&pos) != Some(&ch) {
                execute!(stdout, cursor::MoveTo(pos.0 as u16, pos.1 as u16)).unwrap();
                print!("{}", ch);
            }
        }

        stdout.flush().unwrap();
        std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
        self.back_buffer.clear();
    }
}

/*
impl DoubleBuffer {
    /// this function needs to be flushed.
    pub fn write_line_horizontal(
        &mut self,
        x_0: usize,
        x: usize,
        y: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for i in x_0..=x {
            self.write(i, y, HORIZONTAL_LINE_HIGH)
        }
        Ok(())
    }

    pub fn write_line_vertical(
        &mut self,
        x: usize,
        y_0: usize,
        y: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for i in y_0..=y {
            self.write(x, i, VERTICAL_LINE)
        }
        Ok(())
    }
}
*/

// state methods down here coz am lazy

impl crate::state::State {
    /// this function needs to be flushed.
    pub fn write_line_horizontal(&mut self, x_0: usize, x: usize, y: usize) {
        for i in x_0..=x {
            self.buffer.write(i, y, HORIZONTAL_LINE)
        }
    }

    pub fn write_line_vertical(&mut self, x: usize, y_0: usize, y: usize) {
        for i in y_0..=y {
            self.buffer.write(x, i, VERTICAL_LINE)
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

        /* for i in (x_0 + 1)..=(x - 1) {
            // i actually made a function that does this ig
            self.buffer.write(i, y_0, HORIZONTAL_LINE);
            self.buffer.write(i, y, HORIZONTAL_LINE);
        }
        for i in (y_0 + 1)..=(y - 1) {
            self.buffer.write(x_0, i, VERTICAL_LINE);
            self.buffer.write(x, i, VERTICAL_LINE);
        }
        */
    }
    pub fn write_char_horizontal(&mut self, x_0: usize, x: usize, y: usize, ch: char) {
        for i in x_0..=x {
            self.buffer.write(i, y, ch)
        }
    }

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
    pub fn write_loaded_entries(&mut self) {
        let num_entries = self.loaded.len();
        let idx: u32 = 2;
        let max_idx: u32 = self.buffer.height as u32 - 4;
        if num_entries > 0 {
            for i in 0..=self.n_fits {
                if i <= num_entries as u32 && !i > max_idx {
                    self.write_str_at(
                        2,
                        idx as usize,
                        &self
                            .loaded
                            .get(i as usize)
                            .unwrap()
                            .stringify(self.buffer.width),
                    );
                }
            }
        } else {
            self.no_entry_flag = true;
            self.write_no_entries_warning();
        }
    }
}

/*fn stringify_entry(entry: &Entry, x: usize) -> String {
    let a = String::new();
    let b = String::new();
    let c = String::new();
    let x = x - 2


    while s.len() <= x {

    }

    todo!()
}*/
