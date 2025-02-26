use crate::constant::*;
use crate::term::DoubleBuffer;

use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

pub struct State {
    pub buffer: DoubleBuffer,
}

impl State {
    pub fn new(buffer: DoubleBuffer) -> Self {
        Self { buffer }
    }

    pub fn init(&mut self) {
        let mut stdout = std::io::stdout();
        let _ = execute!(stdout, terminal::EnterAlternateScreen);
        let _ = execute!(stdout, crossterm::cursor::DisableBlinking);
        self.buffer.clear();
        self.buffer.flush(&mut stdout);
        let _ = terminal::enable_raw_mode();
    }

    pub fn deconstruct(&mut self) {
        let mut stdout: std::io::Stdout = std::io::stdout();
        let _ = execute!(stdout, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }

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
        self.write_rectangle(0, self.buffer.width - 1, 0, self.buffer.height - 1);
        self.buffer.flush(stdout);
        Ok(())
    }
}
