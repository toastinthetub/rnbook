pub struct Commander {}

// no command is to have more than 2 arguments
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Command {
    Write,
    Quit,
    New(CommandArgument),
    Shred,
    Delete,
    NULL, // this will die soon
}

/*
we should only ever be changing 1 entry at a time. we store all loaded entries in 1 spot in memory and we can just flush the
*/

pub struct CallQueue {}

// TODO FINISH THIS
impl Command {
    pub fn from_str(s: String) -> Option<Self> {
        let tokens: Vec<char> = s.clone().chars().collect();
        let n = tokens.len();
        if !n > 0 {
            return None;
        }

        // for tok in tokens {}

        for (i, s) in tokens.iter().enumerate() {
            match i {
                0_usize => match s {
                    ' ' => {
                        todo!()
                    }
                    _ => {
                        todo!()
                    }
                },
                1_usize => {
                    // first arg
                }
                2_usize => {
                    // second arg
                }
                _ => {
                    // do nothing
                }
            }
        }
        /*
        match s.as_str() {
            "write" | "w" => {
                //
            }

            _ => {
                return None;
            }
        } */
        None
    }
}

/* notes

i can store string buf from command in state, i can store current index (which SHOULD be the selected entry if my math is right)
and then we can take the string buf and run it through Command::from_str() at which point we can generate a commmand with an exectuable queue

maybe it doesnt even need to be that complicated. we need

*/

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum CommandArgument {
    Path(String),
    Name(String),
    Entry(crate::util::Entry),
}

pub struct CommandParser {}
