#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextStyle {
    Normal,
    Bold,
}

impl Color {
    fn fg_code(&self, style: TextStyle) -> String {
        let base_code = match self {
            Color::Reset => 0,
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::BrightBlack => 90,
            Color::BrightRed => 91,
            Color::BrightGreen => 92,
            Color::BrightYellow => 93,
            Color::BrightBlue => 94,
            Color::BrightMagenta => 95,
            Color::BrightCyan => 96,
            Color::BrightWhite => 97,
        };
        match style {
            TextStyle::Bold => format!("1;{}", base_code),
            TextStyle::Normal => format!("{}", base_code),
        }
    }

    fn bg_code(&self) -> u8 {
        match self {
            Color::Reset => 0,
            Color::Black => 40,
            Color::Red => 41,
            Color::Green => 42,
            Color::Yellow => 43,
            Color::Blue => 44,
            Color::Magenta => 45,
            Color::Cyan => 46,
            Color::White => 47,
            Color::BrightBlack => 100,
            Color::BrightRed => 101,
            Color::BrightGreen => 102,
            Color::BrightYellow => 103,
            Color::BrightBlue => 104,
            Color::BrightMagenta => 105,
            Color::BrightCyan => 106,
            Color::BrightWhite => 107,
        }
    }
}

#[macro_export]
macro_rules! colorize {
    ($text:expr, $fg:expr, $bg:expr, $style:expr) => {
        format!(
            "\x1B[{};{}m{}\x1B[0m",
            $fg.fg_code($style),
            $bg.bg_code(),
            $text
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fg_color_codes() {
        assert_eq!(Color::Red.fg_code(TextStyle::Normal), "31");
        assert_eq!(Color::Blue.fg_code(TextStyle::Bold), "1;34");
        assert_eq!(Color::BrightYellow.fg_code(TextStyle::Normal), "93");
    }

    #[test]
    fn test_bg_color_codes() {
        assert_eq!(Color::Green.bg_code(), 42);
        assert_eq!(Color::BrightMagenta.bg_code(), 105);
    }

    #[test]
    fn test_colorize_macro() {
        let colored_text = colorize!("Hello", Color::Cyan, Color::Black, TextStyle::Bold);
        assert_eq!(colored_text, "\x1B[1;36;40mHello\x1B[0m");
    }
}
