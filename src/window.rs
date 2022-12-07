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
