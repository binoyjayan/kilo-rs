use crate::highlight::*;

pub enum SearchDirection {
    Forwards,
    Backwards,
}

pub struct SearchInfo {
    pub last_match: Option<usize>,
    pub direction: SearchDirection,
    pub saved_highlight: Option<SavedHighlight>,
}

impl SearchInfo {
    pub fn new() -> Self {
        Self {
            last_match: None,
            direction: SearchDirection::Forwards,
            saved_highlight: None,
        }
    }
}

pub struct SavedHighlight {
    pub line: usize,
    pub highlight: Vec<Highlight>,
}

impl SavedHighlight {
    pub fn new(line: usize, highlight: Vec<Highlight>) -> Self {
        Self { line, highlight }
    }
}
