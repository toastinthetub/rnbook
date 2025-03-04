use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Color,
    terminal::{self, Clear, ClearType},
};

use crate::{
    config::Config,
    parser::Parser,
    term::DoubleBuffer,
    util::{log_message, CommandBar, Entry, ModeT, OpenMode},
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
    pub last_mode: ModeT,
    pub config: crate::config::Config,
    pub loaded: Vec<crate::util::Entry>,
    pub string_buffer: Vec<String>,
    pub n_fits: u32,
    pub no_entry_flag: bool,
    pub command_bar: CommandBar,
    pub command_mode: bool,
    pub idx: u32,
    pub idx_active: bool,
    pub idx_selected: bool,
    pub active_buffer: String,
    pub buffer_editable: bool,
    pub dbg: bool,
}

impl State {
    pub fn new(buffer: DoubleBuffer) -> Self {
        let config = Config::load().unwrap();
        let n_fits: u32 = (buffer.height - 4) as u32;
        Self {
            buffer,
            mode: ModeT::BROWSE,
            last_mode: ModeT::BROWSE,
            config,
            loaded: Vec::new(),
            string_buffer: Vec::new(),
            n_fits,
            no_entry_flag: true,
            command_bar: CommandBar {
                buffer: String::from("test buffer on line 51 of state.rs"),
                user_buffer: String::new(),
            },
            command_mode: false,
            idx: 0,
            idx_active: true, // we init this as true coz we start in browse mode
            idx_selected: false,
            active_buffer: String::new(),
            buffer_editable: false,
            dbg: true,
        }
    }

    pub fn load_entries(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file = self.config.entries_file.clone();

        let parser = Parser;

        let entries = parser.get_entries(&file).unwrap_or_default();

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
        let _ = execute!(stdout, Clear(ClearType::All));
    }
    pub fn quit(&mut self) {
        self.deconstruct();
        std::process::exit(0);
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
}

impl State {
    fn handle_event(&mut self) -> bool {
        if event::poll(Duration::from_millis(10)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key_event) => {
                    if key_event.kind != crossterm::event::KeyEventKind::Press {
                        return false;
                    }
                    return self.handle_key_event(key_event);
                }
                Event::Resize(_, _) => self.handle_resize_event(),
                _ => {} // Ignore mouse events and other stuff
            }
        }
        false
    }
    /// handles **keyboard input**
    fn handle_key_event(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Down => {
                if self.idx_active {
                    self.idx += 1;
                    if self.idx >= self.loaded.len() as u32 {
                        self.idx = 0
                    } else {
                    };
                }
            }
            KeyCode::Up => {
                if self.idx_active {
                    if self.idx <= 0 {
                        self.idx = self.loaded.len() as u32 - 1;
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
                        return true; // Exit on CTRL+C
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
    fn handle_resize_event(&mut self) {
        self.buffer.resize();
        self.n_fits = (self.buffer.height - 4) as u32;
        self.string_buffer.clear();
        self.populate_string_buffer();
        // crate::util::log_message("resize event, resize() called");
    }

    /// is passed any raw character presses
    fn handle_char(&mut self, c: char) {
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
                }
                _ => {}
            },
        }
    }
    fn submit_command(&mut self) {
        let mut buf = self.command_bar.get_buffer_contents();
        buf.truncate(2);
        self.command_bar.clear();
        self.command_bar.swap();
        match buf.as_str() {
            "wq" => {
                self.command_bar.clear();
                self.command_bar.push_str("doesnt work yet <sadge :(>")
            }
            "w" => {
                self.command_bar.clear();
                self.command_bar
                    .push_str("lol. no writing to disk happening here")
            }
            "q" => {
                self.quit();
            }
            _ => {}
        }
        self.command_mode = false;
    }
}
