use crossterm::cursor;
use crossterm::style;
use crossterm::terminal;
use crossterm::QueueableCommand;

use std::io;
use std::io::Write;
use std::time;
use std::time::Duration;

use crate::data::*;
use crate::dimensions::*;
use crate::events::*;
use crate::input::*;
use crate::search::*;
use crate::state::*;
use crate::syntax::*;

pub struct Screen {
    input: Input,
    stdout: io::Stdout,
    window: Window,
    cursor: Position,
    editrows: Vec<EditRow>,
    rowoff: usize,
    coloff: usize,
    file: Option<String>,
    dirty: bool,
    quit_times: u8,
    status_msg: String,
    status_time: time::Instant,
    search_info: SearchInfo,
    state: RenderState,
}

type PromptCallback = fn(&mut Screen, &str, EditorEvent) -> bool;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

impl Screen {
    pub fn new(
        lines: &[String],
        file: Option<String>,
        syntax: Option<&'static Syntax>,
    ) -> crossterm::Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        let state = RenderState::new(syntax);
        Ok(Self {
            input: Input::new(),
            stdout: io::stdout(),
            // One row on the bottom for status bar
            window: Window::new(width, height - 2),
            cursor: Position::new(0, 0),
            editrows: Self::make_editrows(lines, &state),
            rowoff: 0,
            coloff: 0,
            file,
            dirty: false,
            quit_times: QUIT_TIMES,
            status_msg: String::from("Ctrl-Q: quit, Ctrl-S: save, Ctrl-F: find"),
            status_time: time::Instant::now(),
            search_info: SearchInfo::new(),
            state,
        })
    }

    pub fn make_editrows(lines: &[String], state: &RenderState) -> Vec<EditRow> {
        let editrows = lines
            .iter()
            .map(|line| EditRow::new(line.to_string(), state))
            .collect::<Vec<EditRow>>();

        editrows
    }

    pub fn set_syntax(&mut self, syntax: Option<&'static Syntax>) {
        self.state.syntax = syntax;
        for row in self.editrows.iter_mut() {
            row.update_row(&self.state);
        }
    }

    pub fn open(&mut self) -> crossterm::Result<()> {
        terminal::enable_raw_mode()
    }

    pub fn read(&self) -> crossterm::Result<EditorEvent> {
        self.input.read()
    }

    pub fn clear(&mut self) -> crossterm::Result<()> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn refresh(&mut self) -> crossterm::Result<()> {
        self.scroll();
        self.clear()?;
        self.draw_rows()?;
        self.draw_status()?;
        self.draw_message()?;
        Ok(())
    }

    /*
     * To display each row at the column offset, coloff as an index into the
     * chars of each editrow displayed, and subtract the number of characters
     * that are to the left of the offset from the length of the row. When
     * subtracting coloff from the length, len can be a negative number,
     * meaning the user scrolled horizontally past the end of the line.
     * In that case, len would be 0 and nothing is displayed on that line.
     *
     * Handle ascii control characters by converting non printable characters
     * into printable ones. Render the alphabetic control characters
     * (Ctrl-A = 1, Ctrl-B = 2, ... Ctrl-Z = 26) as the uppercase letters
     * A through Z. Also render the 0 byte like a control character.
     *
     * Ctrl-@ = 0, so render it as an @ sign. Finally, any other nonprintable
     * characters is rendered as a question mark (?). And to differentiate
     * these characters from their printable counterparts, render them using
     * inverted colors (black on white). use is_control() to check if the
     * current character is a control character. If so,  translate it into
     * a printable one by adding its value to '@' (in ASCII, the uppercase
     * letters of the alphabet come after the @ character), or using the '?'
     * character if it's not in the alphabetic range.
     */

    pub fn draw_rows(&mut self) -> crossterm::Result<()> {
        for y in 0..self.window.height {
            let filerow = y as usize + self.rowoff;
            if filerow >= self.editrows.len() {
                if self.editrows.is_empty() && y == self.window.height / 3 {
                    self.show_welcome(y)?;
                } else {
                    self.stdout
                        .queue(cursor::MoveTo(0, y))?
                        .queue(style::Print("~".to_string()))?;
                }
            } else {
                let colstart = self.coloff;
                // Handling horizontal scrolling
                let len = self.editrows[filerow]
                    .render
                    .len()
                    .saturating_sub(colstart)
                    .min(self.window.width as usize);

                // Nothing to display on this line
                if len == 0 {
                    continue;
                }
                let colend = colstart + len;
                let curr_row = self.editrows[filerow].render[colstart..colend].to_string();
                let curr_highlight = self.editrows[filerow].highlight[colstart..colend].to_vec();
                let mut curr_color = style::Color::Reset;

                self.stdout.queue(cursor::MoveTo(0_u16, y))?;
                for (c, hl) in curr_row.chars().zip(curr_highlight) {
                    // Handle ascii control characters. See notes above.
                    if c.is_control() {
                        let ctrl = if (c as u8) < 26 {
                            (b'@' + c as u8) as char
                        } else {
                            '?'
                        };
                        // Print the control character in the reverse style (fg/bg colors swapped)
                        self.stdout
                            .queue(style::SetAttribute(style::Attribute::Reverse))?
                            .queue(style::Print(ctrl))?
                            .queue(style::SetAttribute(style::Attribute::Reset))?;
                        if curr_color != style::Color::Reset {
                            // An attribute reset resets all formatting, so restore the current color
                            self.stdout.queue(style::SetForegroundColor(curr_color))?;
                        }
                    } else if hl.is_normal() {
                        if curr_color != style::Color::Reset {
                            self.stdout
                                .queue(style::SetForegroundColor(style::Color::Reset))?;
                            curr_color = style::Color::Reset;
                        }
                    } else {
                        let color = style::Color::from(hl);
                        if color != curr_color {
                            self.stdout.queue(style::SetForegroundColor(color))?;
                            curr_color = color;
                        }
                    }
                    self.stdout.queue(style::Print(c))?;
                }
                self.stdout
                    .queue(style::SetForegroundColor(style::Color::Reset))?;
            }
        }
        Ok(())
    }

    pub fn draw_status(&mut self) -> crossterm::Result<()> {
        let width = self.window.width as usize;

        let dirty_str = if self.dirty { ", modified" } else { "" };
        let mut status_left = if let Some(filename) = &self.file {
            format!("'{}' {}L{}", filename, self.editrows.len(), dirty_str)
        } else {
            format!("'No Name' {}L{}", self.editrows.len(), dirty_str)
        };
        status_left.truncate(width);

        let file_type = if let Some(ft) = self.state.syntax {
            ft.filetype.to_string()
        } else {
            "[no ft]".to_string()
        };
        let msg_right = format!(
            "{} {}/{}",
            file_type,
            self.cursor.y + 1,
            self.editrows.len()
        );

        let mut status_right = String::new();
        if status_left.len() < self.window.width as usize - msg_right.len() {
            let mut len = status_left.len();
            while len < width {
                if width - len == msg_right.len() {
                    status_right.push_str(&msg_right);
                    break;
                } else {
                    status_right.push(' ');
                    len += 1;
                }
            }
        }
        let status_msg = format!("{}{}", status_left, status_right);

        let color_status = style::Colors::new(style::Color::Black, style::Color::White);
        self.stdout
            .queue(cursor::MoveTo(0, self.window.height))?
            .queue(style::SetColors(color_status))?
            .queue(style::Print(status_msg))?;

        self.stdout.queue(style::ResetColor)?;

        Ok(())
    }

    pub fn draw_message(&mut self) -> crossterm::Result<()> {
        if self.status_time.elapsed() > Duration::from_secs(5) {
            self.status_msg.clear();
            return Ok(());
        }

        let color_status = style::Colors::new(style::Color::Black, style::Color::White);
        let status_help: String = self
            .status_msg
            .chars()
            .take(self.window.width as usize)
            .collect();

        // Pad the rest of the screen with with spaces
        let rem_len = status_help.len().max(self.window.width as usize) - (status_help.len());
        let status_help = status_help + &" ".repeat(rem_len);

        self.stdout
            .queue(style::SetColors(color_status))?
            .queue(cursor::MoveTo(0, self.window.height + 1))?
            .queue(style::Print(status_help))?;
        self.stdout.queue(style::ResetColor)?;
        Ok(())
    }

    pub fn set_status(&mut self, message: &str) {
        self.status_time = time::Instant::now();
        self.status_msg = message.to_string();
    }

    pub fn show_welcome(&mut self, row: u16) -> crossterm::Result<()> {
        let mut welcome = format!("Kilo-rs version {VERSION}");
        welcome.truncate(self.window.width as usize);
        if welcome.len() < self.window.width as usize {
            let left = ((self.window.width as usize - welcome.len()) / 2) as u16;
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(style::Print("~".to_string()))?
                .queue(cursor::MoveTo(left, row))?
                .queue(style::Print(welcome))?;
        } else {
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(style::Print(welcome))?;
        }
        Ok(())
    }

    /*
     * prompts user for an input string and returns an Ok(Some(String)) or an
     * Ok(None) if the prompt is cancelled. It can also return an error.
     */
    pub fn show_prompt(
        &mut self,
        prompt: &str,
        callback: Option<PromptCallback>,
    ) -> crossterm::Result<Option<String>> {
        let mut buf = String::new();

        loop {
            self.set_status(&format!("{}: {}", prompt, buf));
            self.refresh()?;
            self.flush()?;

            match self.read() {
                Ok(event) => {
                    match event {
                        EditorEvent::Cursor(CursorKey::Enter) => {
                            self.set_status("");
                            Self::do_callback(self, callback, &buf, event);
                            return Ok(Some(buf));
                        }
                        EditorEvent::Key(ch) => {
                            if Input::is_valid_file_char(ch) {
                                buf.push(ch);
                            }
                        }
                        EditorEvent::Cursor(CursorKey::Backspace)
                        | EditorEvent::Cursor(CursorKey::Delete) => {
                            buf.pop();
                        }
                        EditorEvent::Control(ControlEvent::Escape) => {
                            self.set_status("");
                            Self::do_callback(self, callback, &buf, event);
                            return Ok(None);
                        }
                        EditorEvent::Cursor(CursorKey::Right)
                        | EditorEvent::Cursor(CursorKey::Down) => {
                            Self::do_callback(
                                self,
                                callback,
                                &buf,
                                EditorEvent::Cursor(CursorKey::Right),
                            );
                            continue;
                        }
                        EditorEvent::Cursor(CursorKey::Left)
                        | EditorEvent::Cursor(CursorKey::Up) => {
                            Self::do_callback(
                                self,
                                callback,
                                &buf,
                                EditorEvent::Cursor(CursorKey::Left),
                            );
                            continue;
                        }
                        _ => {}
                    }
                    Self::do_callback(self, callback, &buf, event);
                }
                Err(_e) => {}
            }
        }
    }

    fn do_callback(&mut self, callback: Option<PromptCallback>, buf: &str, event: EditorEvent) {
        if let Some(callback) = callback {
            callback(self, buf, event);
        }
    }

    pub fn flush(&mut self) -> crossterm::Result<()> {
        self.stdout.flush()
    }

    pub fn position(&self) -> Position {
        self.cursor
    }

    pub fn _read_pos() -> crossterm::Result<Position> {
        let (x, y) = crossterm::cursor::position()?;
        Ok(Position::new(x, y))
    }

    /*
     * pos.y does not refer to the position of the cursor on the screen.
     * It refers to the position of the cursor within the text file. To
     * position the cursor on the screen, subtract rowoff from the value
     * of pos.cy.
     */
    pub fn move_to(&mut self, pos: Position) -> crossterm::Result<()> {
        self.stdout.queue(cursor::MoveTo(
            pos.rx - self.coloff as u16,
            pos.y - self.rowoff as u16,
        ))?;
        Ok(())
    }

    pub fn move_cursor(&mut self, key: CursorKey) {
        match key {
            CursorKey::Left => {
                if self.cursor.x != 0 {
                    self.cursor.x -= 1;
                } else if self.cursor.y > 0 {
                    // Goto the end of last line if cursor isn't already at the top
                    self.cursor.y -= 1;
                    self.cursor.x = self.editrows[self.cursor.y as usize].chars.len() as u16;
                }
            }
            CursorKey::Right => {
                /* Find editrow index based on if data is available at the row.
                 * Check if data is available at the editrow
                 */
                #[allow(clippy::comparison_chain)]
                if (self.cursor.y as usize) < self.editrows.len() {
                    let idx = self.cursor.y as usize;
                    // limit scrollng to the right
                    if (self.cursor.x as usize) < self.editrows[idx].chars.len() {
                        self.cursor.x += 1;
                    } else if (self.cursor.x as usize) == self.editrows[idx].chars.len() {
                        self.cursor.y += 1;
                        self.cursor.x = 0;
                    }
                }
            }
            CursorKey::Up => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            CursorKey::Down => {
                // allow the cursor to advance past the bottom of the screen, but
                // not past the bottom of the file.
                if (self.cursor.y as usize) < self.editrows.len() {
                    self.cursor.y += 1
                }
            }
            CursorKey::PageUp | CursorKey::PageDown => {
                let direction = if key == CursorKey::PageUp {
                    self.cursor.y = self.rowoff as u16;
                    CursorKey::Up
                } else {
                    let screenrows = self.window.height as usize;
                    self.cursor.y = (self.rowoff + screenrows - 1).min(self.editrows.len()) as u16;
                    CursorKey::Down
                };
                let times = self.window.height as usize;
                for _ in 0..times {
                    self.move_cursor(direction);
                }
            }
            CursorKey::Home => {
                self.cursor.x = 0;
            }
            CursorKey::End => {
                let cy = self.cursor.y as usize;
                if cy < self.editrows.len() {
                    self.cursor.x = self.editrows[cy].chars.len() as u16;
                }
            }
            CursorKey::Delete => {
                self.move_cursor(CursorKey::Right);
                self.delete_char();
            }
            CursorKey::Backspace => self.delete_char(),
            CursorKey::Enter => self.insert_newline(),
        }
        // Find the number of characters on the editrow
        let rowlen = if self.cursor.y as usize >= self.editrows.len() {
            0
        } else {
            self.editrows[self.cursor.y as usize].chars.len() as u16
        };
        self.cursor.x = self.cursor.x.min(rowlen);
    }

    /*
     * Check if the cursor has moved outside of the visible window, if so,
     * adjust E.rowoff so that the cursor is just inside the visible window.
     * Call this function right before the screen is refreshed.
     */
    pub fn scroll(&mut self) {
        self.cursor.rx = if (self.cursor.y as usize) < self.editrows.len() {
            self.editrows[self.cursor.y as usize].cx_to_rx(self.cursor.x)
        } else {
            0
        };

        // Check if cursor is above the visible window
        if (self.cursor.y as usize) < self.rowoff {
            self.rowoff = self.cursor.y as usize;
        }

        /* Check if cursor is past the bottom of the visible window. 'rowoff' refers
         * to the what is at the 'top' of the screen. And 'window.height' needs to be
         * used to figure out the bottom of the screen.
         */
        if (self.cursor.y as usize) >= self.rowoff + (self.window.height as usize) {
            self.rowoff = self.cursor.y as usize - self.window.height as usize + 1;
        }

        if (self.cursor.rx as usize) < self.coloff {
            self.coloff = self.cursor.rx as usize;
        }
        if (self.cursor.rx as usize) >= (self.coloff + self.window.width as usize) {
            self.coloff = self.cursor.rx as usize - self.window.width as usize
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        if (self.cursor.y as usize) == self.editrows.len() {
            self.insert_row(self.editrows.len(), "");
        }
        let cy = self.cursor.y as usize;
        let cx = self.cursor.x as usize;
        self.editrows[cy].insert_char(cx, ch, &self.state);
        self.cursor.x += 1;
        self.set_dirty(true);
    }

    // Delete character left of the cursor
    pub fn delete_char(&mut self) {
        let cy = self.cursor.y as usize;
        let cx = self.cursor.x as usize;

        if cx == 0 && cy == 0 || cy >= self.editrows.len() {
            return;
        }
        let s = self.editrows[cy].chars.clone();
        if cx > 0 {
            self.editrows[cy].delete_char(cx - 1, &self.state);
            self.cursor.x = (cx - 1) as u16;
        } else {
            self.cursor.x = self.editrows[cy - 1].chars.len() as u16;
            self.editrows[cy - 1].append_str(&s, &self.state);
            self.delete_row(cy);
            self.cursor.y -= 1;
        }
        self.set_dirty(true);
    }

    pub fn insert_row(&mut self, at: usize, s: &str) {
        if at > self.editrows.len() {
            return;
        }
        self.editrows
            .insert(at, EditRow::new(s.to_string(), &self.state));
    }

    pub fn insert_newline(&mut self) {
        let cy = self.cursor.y as usize;
        let cx = self.cursor.x as usize;
        // if cursor is at the beginning, just insert a new row at the current row index,
        // else split the current row. Either way increment 'y' and set 'x' to 0.
        if cx == 0 {
            self.insert_row(cy, "")
        } else {
            let new_row = self.editrows[cy].split(cx, &self.state);
            self.editrows.insert(cy + 1, new_row);
        }
        self.cursor.y += 1;
        self.cursor.x = 0;
        self.set_dirty(true);
    }

    pub fn delete_row(&mut self, at: usize) {
        if at >= self.editrows.len() {
            return;
        }
        self.editrows.remove(at);
        self.set_dirty(true);
    }

    pub fn find(&mut self) -> crossterm::Result<()> {
        let saved_cursor = self.cursor;
        let saved_coloff = self.coloff;
        let saved_rowoff = self.rowoff;

        if let Some(query) =
            self.show_prompt("Search (ESC/Arrows/Enter)", Some(Self::find_callback))?
        {
            if !self.find_callback(&query, EditorEvent::Cursor(CursorKey::Enter)) {
                self.set_status(&format!("Could not find '{}'", query));
            }
        } else {
            self.cursor = saved_cursor;
            self.coloff = saved_coloff;
            self.rowoff = saved_rowoff;
            self.set_status("Cancelled search");
        }
        self.restore_highlight();
        Ok(())
    }

    pub fn find_callback(&mut self, query: &str, event: EditorEvent) -> bool {
        self.restore_highlight();

        match event {
            EditorEvent::Control(ControlEvent::Escape) => {
                self.search_info.last_match = None;
                self.search_info.direction = SearchDirection::Forwards;
                return false;
            }
            EditorEvent::Cursor(CursorKey::Right) => {
                self.search_info.direction = SearchDirection::Forwards;
            }
            EditorEvent::Cursor(CursorKey::Left) => {
                self.search_info.direction = SearchDirection::Backwards;
            }
            _ => {}
        }

        let mut current = if let Some(last_match) = self.search_info.last_match {
            last_match
        } else {
            self.search_info.direction = SearchDirection::Forwards;
            0
        };
        /* Perform 'editrows.len()' iterations so that each row is searched for
         * atleast once but not necessarily in the order of its indices, but
         * starting with index 'current', moving forward, wrapping back to 0
         * when 'current' reaches the end. Set the cursor to the matched string
         * and return when upon finding a match.
         */
        for _ in self.editrows.iter() {
            if matches!(
                event,
                EditorEvent::Cursor(CursorKey::Left) | EditorEvent::Cursor(CursorKey::Right)
            ) {
                // Change cursor only when arrow keys are used for next/prev search
                match self.search_info.direction {
                    SearchDirection::Forwards => {
                        current = if current >= (self.editrows.len() - 1) {
                            0
                        } else {
                            current + 1
                        };
                    }
                    SearchDirection::Backwards => {
                        current = if current == 0 {
                            self.editrows.len() - 1
                        } else {
                            current - 1
                        };
                    }
                }
            }

            if let Some(rx) = self.editrows[current].render.find(query) {
                self.search_info.last_match = Some(current);
                self.cursor.y = current as u16;
                self.cursor.x = self.editrows[current].rx_to_cx(rx as u16) as u16;
                self.rowoff = self.editrows.len();
                let saved_hl = self.editrows[current].highlight.clone();
                self.search_info.saved_highlight = Some(SavedHighlight::new(current, saved_hl));
                self.editrows[current].highlight_match(rx, query.len());
                return true;
            }
        }
        false
    }

    fn restore_highlight(&mut self) {
        if let Some(saved_hl) = &self.search_info.saved_highlight {
            self.editrows[saved_hl.line].highlight = saved_hl.highlight.clone();
            self.search_info.saved_highlight = None;
        }
    }

    pub fn is_dirty(&mut self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    pub fn dec_quit_times(&mut self) -> u8 {
        self.quit_times -= 1;
        self.quit_times
    }

    pub fn reset_quit_times(&mut self) {
        self.quit_times = QUIT_TIMES;
    }

    pub fn rows_to_string(&self) -> String {
        self.editrows
            .iter()
            .map(|x| x.chars.clone())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn release(&mut self) -> crossterm::Result<()> {
        let _ = self.clear();
        terminal::disable_raw_mode()
    }
}
