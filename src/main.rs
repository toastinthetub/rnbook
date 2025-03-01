mod color;
mod constant;
mod parser;
mod state;
mod term;
mod util;

use term::DoubleBuffer;

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    let buffer = DoubleBuffer::new();

    let mut state = crate::state::State::new(buffer);
    state.buffer.clear();
    state.buffer.flush(&mut stdout);
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;
    println!("thanks for using rnbook.");
    state.event_loop().unwrap();
    Ok(())
}
