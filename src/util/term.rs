/*
 * src/util/term.rs
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
use crossterm::{
    cursor, execute,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self},
};

use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

#[derive(Debug, Clone)]
pub struct DoubleBuffer {
    front_buffer: HashMap<(usize, usize), char>,
    back_buffer: HashMap<(usize, usize), char>,

    front_fg_buffer: HashMap<(usize, usize), Color>,
    front_bg_buffer: HashMap<(usize, usize), Color>,

    back_fg_buffer: HashMap<(usize, usize), Color>,
    back_bg_buffer: HashMap<(usize, usize), Color>,

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

            front_fg_buffer: HashMap::new(),
            front_bg_buffer: HashMap::new(),
            back_fg_buffer: HashMap::new(),
            back_bg_buffer: HashMap::new(),

            width: width as usize,
            height: height as usize,
            too_small_flag,
        }
    }

    /// recalculate terminal size in case of resize
    pub fn resize(&mut self) {
        if let Ok((tw, th)) = terminal::size() {
            self.width = tw as usize;
            self.height = th as usize;
            self.too_small_flag = tw < 60 || th < 4;

            self.front_buffer.clear();
            self.front_fg_buffer.clear();
            self.front_bg_buffer.clear();

            self.back_buffer.clear();
            self.back_fg_buffer.clear();
            self.back_bg_buffer.clear();

            for y in 0..self.height {
                for x in 0..self.width {
                    self.back_buffer.insert((x, y), ' ');
                    self.back_fg_buffer.insert((x, y), Color::White);
                    self.back_bg_buffer.insert((x, y), Color::Black);
                }
            }
            self.flush(&mut std::io::stdout());
        }
    }

    /// write to the **back buffer**
    pub fn write(&mut self, x: usize, y: usize, ch: char) {
        self.write_colored(x, y, ch, Color::White, Color::Black);
    }

    /// writes a colored char to the back buffer
    pub fn write_colored(&mut self, x: usize, y: usize, ch: char, fg: Color, bg: Color) {
        if x < self.width && y < self.height {
            self.back_buffer.insert((x, y), ch);
            self.back_fg_buffer.insert((x, y), fg);
            self.back_bg_buffer.insert((x, y), bg);
        }
    }

    /// clear buffer with whitespace
    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.back_buffer.insert((x, y), ' ');
            }
        }
    }

    /// diff & flush to stdout
    pub fn flush(&mut self, stdout: &mut impl Write) {
        if self.front_buffer.is_empty() {
            for y in 0..self.height {
                for x in 0..self.width {
                    let pos = (x, y);
                    let new_char = *self.back_buffer.get(&pos).unwrap_or(&' ');
                    let new_fg = *self
                        .back_fg_buffer
                        .get(&pos)
                        .unwrap_or(&crossterm::style::Color::White);
                    let new_bg = *self
                        .back_bg_buffer
                        .get(&pos)
                        .unwrap_or(&crossterm::style::Color::Black);
                    execute!(
                        stdout,
                        cursor::MoveTo(x as u16, y as u16),
                        SetForegroundColor(new_fg),
                        SetBackgroundColor(new_bg)
                    )
                    .unwrap();
                    print!("{}", new_char);
                }
            }
        } else {
            let all_positions: HashSet<(usize, usize)> = self
                .front_buffer
                .keys()
                .chain(self.back_buffer.keys())
                .cloned()
                .collect();

            for pos in all_positions {
                if pos.0 >= self.width || pos.1 >= self.height {
                    continue;
                }
                let old_char = *self.front_buffer.get(&pos).unwrap_or(&' ');
                let new_char = *self.back_buffer.get(&pos).unwrap_or(&' ');
                let old_fg = *self
                    .front_fg_buffer
                    .get(&pos)
                    .unwrap_or(&crossterm::style::Color::White);
                let new_fg = *self
                    .back_fg_buffer
                    .get(&pos)
                    .unwrap_or(&crossterm::style::Color::White);
                let old_bg = *self
                    .front_bg_buffer
                    .get(&pos)
                    .unwrap_or(&crossterm::style::Color::Black);
                let new_bg = *self
                    .back_bg_buffer
                    .get(&pos)
                    .unwrap_or(&crossterm::style::Color::Black);

                if old_char != new_char || old_fg != new_fg || old_bg != new_bg {
                    execute!(
                        stdout,
                        cursor::MoveTo(pos.0 as u16, pos.1 as u16),
                        SetForegroundColor(new_fg),
                        SetBackgroundColor(new_bg)
                    )
                    .unwrap();
                    print!("{}", new_char);
                }
            }
        }

        execute!(stdout, ResetColor).unwrap();
        stdout.flush().unwrap();

        std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
        std::mem::swap(&mut self.front_fg_buffer, &mut self.back_fg_buffer);
        std::mem::swap(&mut self.front_bg_buffer, &mut self.back_bg_buffer);

        self.back_buffer.clear();
        self.back_fg_buffer.clear();
        self.back_bg_buffer.clear();
    }
}
