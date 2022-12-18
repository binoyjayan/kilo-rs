use std::ffi;
use std::fmt::Display;
use std::fs;
use std::path;

use crate::events::*;
use crate::screen::*;
use crate::syntax::*;

pub struct Editor {
    screen: Screen,
    file: Option<String>,
}

impl Editor {
    pub fn new() -> crossterm::Result<Self> {
        Self::create(&[], None, None)
    }

    pub fn open(file: &str) -> crossterm::Result<Self> {
        let data = Self::read_file(file);
        let lines: Vec<String> = data.split('\n').map(|s: &str| s.to_string()).collect();
        let syntax = Self::file_syntax(file)?;
        Self::create(&lines, Some(file.to_string()), syntax)
    }

    /*
     * Look up the syntax highlight database for the file extension and file
     * a reference to the syntax object for the file type.
     */
    fn file_syntax(filename: &str) -> crossterm::Result<Option<&'static Syntax>> {
        let path = path::Path::new(&filename).canonicalize()?;
        if let Some(extension) = path.extension().and_then(ffi::OsStr::to_str) {
            for syntax in HLDB.iter() {
                for ext in syntax.filematch.iter() {
                    if ext == extension {
                        return Ok(Some(syntax));
                    }
                }
            }
        }
        Ok(None)
    }

    fn read_file(file: &str) -> String {
        match fs::read_to_string(file) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("{}: {}", file, e);
                std::process::exit(1);
            }
        }
    }

    pub fn create(
        lines: &[String],
        file: Option<String>,
        syntax: Option<&'static Syntax>,
    ) -> crossterm::Result<Self> {
        Ok(Self {
            screen: Screen::new(lines, file.clone(), syntax)?,
            file,
        })
    }

    pub fn die<T: Display>(&mut self, message: &str, err: T) {
        let _ = self.screen.release();
        eprintln!("{}: {}", message, err);
        std::process::exit(1);
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        self.screen.open()?;

        loop {
            self.screen.refresh()?;
            let pos = self.screen.position();
            self.screen.move_to(pos)?;
            self.screen.flush()?;

            if self.event()? {
                break;
            }
        }
        self.screen.release()
    }

    pub fn event(&mut self) -> crossterm::Result<bool> {
        let result = self.screen.read();

        match result {
            Ok(event) => match event {
                EditorEvent::Key(ch) => {
                    self.screen.insert_char(ch);
                }
                EditorEvent::Cursor(direction) => self.screen.move_cursor(direction),
                EditorEvent::Control(ctrl) => match ctrl {
                    ControlEvent::Quit => {
                        let quit_times = self.screen.dec_quit_times();
                        if self.screen.is_dirty() && quit_times > 0 {
                            let msg = format!("WARNING: File has unsaved changes. Press Ctrl-Q {} more time(s) to quit", quit_times);
                            self.screen.set_status(&msg);
                            return Ok(false);
                        } else {
                            return Ok(true);
                        }
                    }
                    ControlEvent::Save => self.save()?,
                    ControlEvent::Escape => {
                        self.screen.set_status("");
                    }
                    ControlEvent::Find => {
                        self.screen.find()?;
                    }
                },
            },
            Err(e) => {
                self.die("Failed to read event", e);
            }
        }
        self.screen.reset_quit_times();
        Ok(false)
    }

    pub fn save(&mut self) -> crossterm::Result<()> {
        let filename = if let Some(filename) = self.file.clone() {
            Some(filename)
        } else {
            self.screen.show_prompt("Save as", None)?
        };
        if let Some(filename) = filename {
            if self.save_as(&filename) {
                self.screen.set_syntax(Self::file_syntax(&filename)?);
                self.file = Some(filename);
            }
        } else {
            self.screen.set_status("Cancelled save");
        }
        Ok(())
    }

    pub fn save_as(&mut self, filename: &str) -> bool {
        let buf = self.screen.rows_to_string();
        match fs::write(filename, &buf) {
            Ok(_) => {
                let file_len = buf.as_bytes().len();
                self.screen.set_dirty(false);
                self.screen
                    .set_status(&format!("{} bytes written to {}", file_len, filename));
                true
            }
            Err(e) => {
                self.screen
                    .set_status(&format!("Failed to write to '{}' - {}", filename, e));
                false
            }
        }
    }
}
