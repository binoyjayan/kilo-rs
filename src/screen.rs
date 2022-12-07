use crossterm::cursor;
use crossterm::style::Print;
use crossterm::terminal;
use crossterm::QueueableCommand;
use std::io;
use std::io::Write;

pub struct Screen {
    stdout: io::Stdout,
    width: u16,
    height: u16,
}

impl Screen {
    pub fn new() -> crossterm::Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self {
            stdout: io::stdout(),
            width: columns,
            height: rows,
        })
    }

    pub fn open(&mut self) -> crossterm::Result<()> {
        terminal::enable_raw_mode()?;
        self.refresh()
    }

    pub fn clear(&mut self) -> crossterm::Result<()> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()
    }

    pub fn draw_rows(&mut self) -> crossterm::Result<()> {
        for row in 0..self.height {
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(Print("~".to_string()))?;
        }
        Ok(())
    }

    pub fn refresh(&mut self) -> crossterm::Result<()> {
        self.clear()?;
        self.draw_rows()?;
        self.stdout.queue(cursor::MoveTo(1, 0))?.flush()
    }

    pub fn release(&mut self) -> crossterm::Result<()> {
        let _ = self.clear();
        terminal::disable_raw_mode()
    }
}
