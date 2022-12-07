use crate::editor::*;

mod cursor;
mod editor;
mod events;
mod input;
mod screen;
mod window;

fn main() -> crossterm::Result<()> {
    let mut editor = Editor::new()?;
    editor.run()
}
