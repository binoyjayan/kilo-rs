#[derive(Default, Clone)]
pub struct Window {
    pub width: u16,
    pub height: u16,
}

impl Window {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub rx: u16, // render row
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y, rx: 0 }
    }
}

pub enum SearchDirection {
    Forwards,
    Backwards,
}
