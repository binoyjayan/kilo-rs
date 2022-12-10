#[derive(Default, Clone)]
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