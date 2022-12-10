#[derive(Default)]
pub struct EditRow {
    pub chars: Vec<String>,  // characters in the file
    pub render: Vec<String>, // characters rendered on the screen
    pub rowoff: usize,
    pub coloff: usize,
}

impl EditRow {
    pub fn new(chars: Vec<String>) -> Self {
        let render = chars.clone();
        // vec![String::new(); len],
        Self {
            chars,
            render,
            rowoff: 0,
            coloff: 0,
        }
    }
}
