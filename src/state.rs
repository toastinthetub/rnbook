use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Color,
    terminal::{self},
};

use crate::{
    config::Config,
    parser::Parser,
    term::DoubleBuffer,
    util::{log_message, Entry, ModeT, OpenMode},
};

use std::{
    fs::{File, OpenOptions},
    io::{stdout, BufRead, BufReader, Write},
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub struct State {
    pub buffer: DoubleBuffer,
    pub mode: ModeT,
    pub config: crate::config::Config,
    pub loaded: Vec<crate::util::Entry>,
    pub string_buffer: Vec<String>,
    pub n_fits: u32,
    pub no_entry_flag: bool,
}

impl State {
    pub fn new(buffer: DoubleBuffer) -> Self {
        let config = Config::load().unwrap();
        let n_fits: u32 = (buffer.height - 4) as u32;
        Self {
            buffer,
            mode: ModeT::BROWSE,
            config,
            loaded: Vec::new(),
            string_buffer: Vec::new(),
            n_fits,
            no_entry_flag: true,
        }
    }

    pub fn load_entries(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file = self.config.entries_file.clone();

        let parser = Parser;

        let entries = parser.get_entries(&file)?;

        self.loaded = entries;
        self.no_entry_flag = self.loaded.is_empty();

        Ok(())
    }

    pub fn push_entry(&mut self, entry: Entry) -> Result<(), Box<dyn std::error::Error>> {
        let parser = Parser;
        parser.add_entry(&self.config.entries_file, &entry)?;
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

        if !self.loaded.is_empty() {
            self.no_entry_flag = false;
        }

        self.populate_string_buffer();

        self.buffer.clear();
        self.buffer.resize();
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
    pub fn populate_string_buffer(&mut self) {
        for entry in self.loaded.iter() {
            self.string_buffer.push(entry.stringify(self.buffer.width));
        }
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
    } */
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
        self.n_fits = (self.buffer.height - 4) as u32;
        self.string_buffer.clear();
        self.populate_string_buffer();
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
