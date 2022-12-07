use crate::editor::*;

mod editor;
mod input;
mod screen;

fn main() -> crossterm::Result<()> {
    let mut editor = Editor::new()?;
    editor.run()
}
