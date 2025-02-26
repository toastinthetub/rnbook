use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crate::constant::*;

pub struct DoubleBuffer {
    front_buffer: HashMap<(usize, usize), char>,
    back_buffer: HashMap<(usize, usize), char>,
    pub width: usize,
    pub height: usize,
}

impl DoubleBuffer {
    pub fn new() -> Self {
        let (width, height) = terminal::size().unwrap();
        Self {
            front_buffer: HashMap::new(),
            back_buffer: HashMap::new(),
            width: width as usize,
            height: height as usize,
        }
    }

    /// recalculate terminal size in case of resize
    pub fn resize(&mut self) {
        let (width, height) = terminal::size().unwrap();
        self.width = width as usize;
        self.height = height as usize;
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
}
