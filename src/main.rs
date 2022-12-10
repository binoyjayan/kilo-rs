use crate::editor::*;
use std::path::Path;

mod cursor;
mod data;
mod editor;
mod events;
mod input;
mod screen;
mod window;

fn main() -> crossterm::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut editor = if args.len() < 2 {
        Editor::new()
    } else {
        let file_path = Path::new(&args[1]);
        Editor::open(file_path)
    }?;

    editor.run()
}
