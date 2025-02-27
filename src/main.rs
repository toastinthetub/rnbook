mod color;
mod constant;
mod state;
mod term;
mod util;

use term::DoubleBuffer;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

fn main() {
    let mut stdout = std::io::stdout();
    let buffer = DoubleBuffer::new();

    let mut state = crate::state::State::new(buffer);
    state.buffer.clear();
    state.buffer.flush(&mut stdout);

    let (w, h) = crossterm::terminal::size().unwrap();
    state.event_loop().unwrap();
}
