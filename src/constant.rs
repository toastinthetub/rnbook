use colored::{Color, ColoredString, Colorize};
use std::io::Write;
// constants

/*
▀ 	▁ 	▂ 	▃ 	▄ 	▅ 	▆ 	▇ 	█ 	▉ 	▊ 	▋ 	▌ 	▍ 	▎ 	▏

▐ 	░ 	▒ 	▓ 	▔ 	▕ 	▖ 	▗ 	▘ 	▙ 	▚ 	▛ █ 	▜ 	▝ 	▞ 	▟
*/
pub const VERTICAL_LINE: char = '│';
pub const HORIZONTAL_LINE: char = '─';
pub const LEFT_UPPER_SHOULDER: char = '┌';
pub const RIGHT_UPPER_SHOULDER: char = '┐';
pub const LEFT_LOWER_SHOULDER: char = '└';
pub const RIGHT_LOWER_SHOULDER: char = '┘';
pub const WHITESPACE: char = ' ';

// pub const CHRONICLE_RESOURCE_PATH: &str =
//    "/home/fizbin/lair/proj/rust/chronicle/asset/chronicle.txt";

// pub const MENU_OPTION_NEW_ENTRY: &str = "[] NEW ENTRY";
// pub const MENU_OPTION_BROWSE_ENTRIES: &str = "[] BROWSE ENTRIES";
// pub const MENU_OPTION_QUIT: &str = "[] QUIT";

pub fn selected(s: &str) -> ColoredString {
    s.to_owned().white().bold().on_bright_black()
}
