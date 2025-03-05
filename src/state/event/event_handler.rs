/*
 * src/state/event/handle_event.rs
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

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    state::state::State,
    util::mode::{ModeT, OpenMode},
};

use std::{
    io::stdout,
    time::{Duration, Instant},
};

impl State {
    /// main event loop function!
    pub fn event_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.init()?;
        let mut stdout = stdout();
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(33); // ~30 FPS

        loop {
            if self.handle_event() {
                break;
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
                self.render(&mut stdout)?;
            }
        }
        self.deconstruct();
        Ok(())
    }

    /// polls, checks for an event of an sort. returns true if yes
    pub fn handle_event(&mut self) -> bool {
        if event::poll(Duration::from_millis(10)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key_event) => {
                    if key_event.kind != crossterm::event::KeyEventKind::Press {
                        // stupid windows bug
                        return false;
                    }
                    return self.handle_key_event(key_event);
                }
                Event::Resize(_, _) => self.handle_resize_event(),
                _ => {} // ignore any other events
            }
        }
        false
    }
}

impl State {
    /// handles **keyboard input**
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Down => {
                if self.idx_active {
                    self.idx += 1;
                    if self.idx >= self.entries_map.len() {
                        self.idx = 0
                    } /* else {
                      }; */
                }
            }
            KeyCode::Up => {
                if self.idx_active {
                    if self.idx == 0 {
                        self.idx = (self.entries_map.len())
                            .checked_sub(1)
                            .unwrap_or(self.entries_map.len());
                    } else {
                        self.idx -= 1;
                    }
                }
            }
            KeyCode::Esc => {
                if self.command_mode {
                    self.command_bar.swap();
                    self.command_mode = false;
                } else {
                    match self.mode {
                        ModeT::BROWSE => {
                            return true;
                        }
                        ModeT::OPEN(OpenMode::READ) => {
                            // read
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Char(c) => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    if c == 'c' {
                        return true; // exit on CTRL+C
                    }
                } else if c == ':' && self.mode != ModeT::OPEN(OpenMode::EDIT) {
                    self.command_bar.swap();
                    self.command_mode = true;
                } else {
                    self.handle_char(c);
                }
            }
            KeyCode::Backspace => {
                if self.command_mode {
                    self.command_bar.pop_char();
                }
            }
            KeyCode::Enter => {
                if self.command_mode {
                    self.submit_command();
                }
            }
            _ => {}
        }
        false
    }

    /// handles **resize events**
    pub fn handle_resize_event(&mut self) {
        self.buffer.resize();
        self.n_fits = (self.buffer.height - 4) as u32;
        self.string_buffer.clear();
        self.populate_string_buffer();
        // crate::util::log_message("resize event, resize() called");
    }

    /// is passed any raw character presses
    pub fn handle_char(&mut self, c: char) {
        if self.command_mode {
            self.command_bar.push_char(c);
        }
        match &self.mode {
            ModeT::BROWSE => {}
            ModeT::OPEN(open_mode) => match open_mode {
                OpenMode::EDIT => {
                    // find the buffer (should be somewhere in state) and push this char to it
                }
                OpenMode::READ => {
                    // we should only really be worried about commands
                } //_ => {}
            },
        }
    }
}
