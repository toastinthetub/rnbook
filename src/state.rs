use crate::term::DoubleBuffer;
use crate::util::{log_message, Entry};
use crate::{
    constant::*,
    util::{ModeT, OpenMode},
};

use crate::parser::Parser;

use chrono::ParseError;
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use dirs::home_dir;
use std::fs::{File, OpenOptions};
use std::io::{stdout, Write};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub struct State {
    pub buffer: DoubleBuffer,
    pub mode: ModeT,
    pub entry_path: String,
    pub loaded: Vec<crate::util::Entry>,
}

impl State {
    pub fn new(buffer: DoubleBuffer) -> Self {
        let mut config_path = home_dir().expect("Failed to get home directory");
        config_path.push(".config/rnbook.config");

        let default_entry_path = format!("{}/rnbook_entries", home_dir().unwrap().display());
        let mut entry_path = None;

        if let Ok(file) = File::open(&config_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Some(entry) = line.strip_prefix("ENTRY_PATH=") {
                        entry_path = Some(entry.trim().to_string());
                        break;
                    }
                } else {
                    //
                }
            }
        }

        let entry_path = entry_path.unwrap_or_else(|| {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&config_path)
                .expect("failed to create config file");

            writeln!(file, "ENTRY_PATH={}", default_entry_path)
                .expect("failed to write to config file");

            default_entry_path
        });

        Self {
            buffer,
            mode: ModeT::BROWSE,
            entry_path,
            loaded: Vec::new(),
        }
    }

    pub fn load_entries(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.entry_path)
            .expect("failed to create config file");
        let mut parser = Parser::new(file);

        let entries = parser.get_entries()?;

        self.loaded = entries;

        Ok(())
    }

    pub fn push_entry(&mut self, entry: Entry) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.entry_path)
            .expect("failed to create config file");

        let parser = Parser::new(file);
        parser.add_entry(&self.entry_path, &entry)?;

        Ok(())
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = std::io::stdout();
        let _ = execute!(stdout, terminal::EnterAlternateScreen);
        let _ = execute!(stdout, crossterm::cursor::Hide);
        self.buffer.clear();
        self.buffer.flush(&mut stdout);
        let _ = terminal::enable_raw_mode();

        self.mode = ModeT::BROWSE;
        self.load_entries()?;

        Ok(())
    }

    pub fn deconstruct(&mut self) {
        let mut stdout: std::io::Stdout = std::io::stdout();
        let _ = execute!(stdout, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }

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
    /*
        pub fn event_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            self.init();
            let mut stdout = stdout();
            //    terminal::enable_raw_mode().unwrap();
            //    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide).unwrap();

            let mut last_tick = Instant::now();
            let tick_rate = Duration::from_millis(33); // 30fps = 33ms frametime

            loop {
                if event::poll(Duration::from_millis(10)).unwrap() {
                    if let Event::Key(KeyEvent {
                        code, modifiers, ..
                    }) = event::read().unwrap()
                    {
                        match code {
                            KeyCode::Esc => break,
                            KeyCode::Char(c) => {
                                if modifiers.contains(KeyModifiers::CONTROL) {
                                    if c == 'c' {
                                        break; // CONTROL-C BREAK
                                    } else {
                                        // DO NOTHING; OTHER CTRL+CHAR
                                        // handle_modified(c);
                                    }
                                } else {
                                    // handle_char(c);
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    last_tick = Instant::now();
                }
                self.render(&mut stdout).unwrap();
            }
            self.deconstruct();
            Ok(())
        }
    */
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
            // TODO self.write_entries()
            return Ok(());
        }
        self.write_rectangle(0, self.buffer.width - 1, 0, self.buffer.height - 1); // draws the border rectangle
        self.write_str_at((self.buffer.width / 2) - 1, self.buffer.height / 2, "X");
        self.buffer.flush(stdout);
        Ok(())
    }
}

impl State {
    fn handle_event(&mut self) -> bool {
        if event::poll(Duration::from_millis(10)).unwrap() {
            // crate::util::log_message("event!");
            match event::read().unwrap() {
                Event::Key(key_event) => return self.handle_key_event(key_event),
                Event::Resize(_, _) => self.handle_resize_event(),
                _ => {} // Ignore mouse events and other stuff
            }
        }
        false
    }

    /// handles **keyboard input**
    fn handle_key_event(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Esc => return true,
            KeyCode::Char(c) => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    if c == 'c' {
                        return true; // Exit on CTRL+C
                    }
                } else {
                    self.handle_char(c);
                }
            }
            _ => {}
        }
        false
    }

    /// handles **resize events**
    fn handle_resize_event(&mut self) {
        self.buffer.resize();
        // crate::util::log_message("resize event, resize() called");
    }

    /// is passed any raw character presses
    fn handle_char(&self, c: char) {
        match &self.mode {
            ModeT::BROWSE => {}
            ModeT::OPEN(open_mode) => match open_mode {
                OpenMode::EDIT => {
                    // find the buffer (should be somewhere in state) and push this char to it
                }
                OpenMode::READ => {
                    // we should only really be worried about commands
                }
            },
            ModeT::COMMAND => {
                // push to command bar
            }
        }
    }
}
