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
    /*
        state.write_line_horizontal(1, (w - 1) as usize, 1);
        state.write_line_horizontal(1, (w - 1) as usize, (h - 1) as usize);
        /*
            for i in 0..=h - 1 {
                state
                    .write_line_horizontal(1, (w - 1) as usize, i as usize)
                    .unwrap();
            }
        */
        state.write_line_horizontal(1, (w - 1) as usize, (h - 1) as usize);

        state.write_line_vertical(1, 0, h as usize);
        state.write_line_vertical(w as usize, 0, h as usize);

        state.buffer.flush(&mut stdout);
        // state.buffer.write()
    */
    state.event_loop().unwrap();
}
