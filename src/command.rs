#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Command {
    Write,
    Quit,
}

impl Command {}

pub struct CommandParser {}
