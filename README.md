# kilo-rs

🦀 🦀 🦀 A Rust port of the text editor 'kilo' from antirez 🦀 🦀 🦀

The editor is cross platform and has been tested on MacOs and Linux.

## Build and run

```
cargo run --release tests/test01.c
cargo run --release tests/test02.p2
```

## Feature set

### Navigation
  - Arrow Keys to move cursor up/down/left/right
  - Page Up / Page down to move pages
  - Home / End to move to beginning/end of line

### Control keys
 - Ctrl + Q: Quit
 - Ctrl + H: Backspace
 - Ctrl + S: Save
 - Ctrl + F: Find
 - Ctrl + L: Toggle line numbers

### Text Search [ Ctrl + F]
  - Incremental search
  - Arrow keys to navigate to next occurrence
  - Esc to go back to edit mode, restore cursor
  - Enter to go back to edit mode, move cursor to the occurrence

### Text manipulation
 - Ascii key codes characters to insert text
 - Backspace/Del key to remove text
 - Tab key to insert 8 characters

### Syntax highlighting
 - File type based syntax support
 - Supported file types - c,c++,sh,rust,python

## TODO
 - Config files to store custom config
 - Copy and paste text
 - Support for multiple files
 - Unicode support
 - Keymap
 - Language client support

