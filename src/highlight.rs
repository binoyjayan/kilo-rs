use crossterm::style;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Highlight {
    Normal,
    Comment,
    MlComment,
    KeywordBase,
    KeywordType,
    Number,
    Str,
    Match,
}

impl Highlight {
    pub fn is_normal(&self) -> bool {
        self == &Highlight::Normal
    }
}

impl From<Highlight> for style::Color {
    fn from(highlight: Highlight) -> style::Color {
        match highlight {
            Highlight::Normal => style::Color::White,
            Highlight::Comment => style::Color::Cyan,
            Highlight::MlComment => style::Color::Cyan,
            Highlight::KeywordBase => style::Color::Yellow,
            Highlight::KeywordType => style::Color::Green,
            Highlight::Number => style::Color::Red,
            Highlight::Str => style::Color::Magenta,
            Highlight::Match => style::Color::Blue,
        }
    }
}
