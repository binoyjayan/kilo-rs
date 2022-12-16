use crate::editor::*;

mod data;
mod dimensions;
mod editor;
mod events;
mod input;
mod screen;
mod search;
mod syntax;

fn main() -> crossterm::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut editor = if args.len() < 2 {
        Editor::new()
    } else {
        Editor::open(&args[1])
    }?;

    editor.run()
}
