use crossterm::{
    cursor, execute,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self},
};

use crate::constant::*;
use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

const TOO_SMALL_WARNING: &str = "> 60x4 TERM SIZE REQUIRED";
const NO_ENTRIES_WARNING: &str = "< not an entry to be found :) >";

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
    pub fn write_colored(&mut self, x: usize, y: usize, ch: char, fg: Color, bg: Color) {
        if x < self.width && y < self.height {
            self.back_buffer.insert((x, y), ch);
            self.back_fg_buffer.insert((x, y), fg);
            self.back_bg_buffer.insert((x, y), bg);
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.back_buffer.insert((x, y), ' ');
            }
        }
    }
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
/*
    pub fn flush(&mut self, stdout: &mut impl Write) {
        use crossterm::{
            cursor, execute,
            style::{ResetColor, SetBackgroundColor, SetForegroundColor},
        };

        for (&pos, &new_char) in &self.back_buffer {
            let old_char = self.front_buffer.get(&pos);

            let new_fg = self.back_fg_buffer.get(&pos).unwrap_or(&Color::White);
            let old_fg = self.front_fg_buffer.get(&pos);

            let new_bg = self.back_bg_buffer.get(&pos).unwrap_or(&Color::Black);
            let old_bg = self.front_bg_buffer.get(&pos);

            let char_changed = old_char != Some(&new_char);
            let fg_changed = old_fg != Some(new_fg);
            let bg_changed = old_bg != Some(new_bg);

            if char_changed || fg_changed || bg_changed {
                execute!(stdout, cursor::MoveTo(pos.0 as u16, pos.1 as u16)).unwrap();
                execute!(stdout, SetForegroundColor(*new_fg)).unwrap();
                execute!(stdout, SetBackgroundColor(*new_bg)).unwrap();
                print!("{}", new_char);
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
*/
/* impl DoubleBuffer {
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

    // TODO FINISH WRITING AND IMPLEMENT SELECTION

    pub fn write_loaded_entries(&mut self) {
        let num_entries = self.loaded.len();
        let mut idx: u32 = 0;
        let max_idx: u32 = self.buffer.height as u32 - 4;

        if num_entries > 0 {
            for i in 0..=std::cmp::min(self.n_fits, num_entries as u32) - 1 {
                if i < max_idx {
                    let default_entry = crate::util::Entry {
                        label: String::from("it didnt work"),
                        date: String::from("datedate"),
                        content: String::from("some content"),
                    }; // TODO do something about this abomination

                    let entry = self.loaded.get(i as usize).unwrap_or(&default_entry);

                    let entry_string = if idx == self.idx {
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
    pub fn write_command_bar(&mut self) {
        let str = self.command_bar.stringify(self.buffer.width as u32);
        self.write_colored_str_at(1, 1, &str, Color::Black, Color::White)
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
            &format!("nld: {}", self.loaded.len()),
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

/*fn stringify_entry(entry: &Entry, x: usize) -> String {
    let a = String::new();
    let b = String::new();
    let c = String::new();
    let x = x - 2


    while s.len() <= x {

    }

    todo!()
}*/
