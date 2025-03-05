mod command;
mod config;
mod constant;
mod db;
mod render;
mod state;
mod term;
mod util;

use term::DoubleBuffer;

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

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
