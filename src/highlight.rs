use crossterm::style;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Highlight {
    Normal,
    Number,
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
            Highlight::Number => style::Color::Red,
            Highlight::Match => style::Color::Blue,
        }
    }
}
