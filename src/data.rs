use crate::syntax::*;

#[derive(Clone)]
pub struct EditRow {
    pub chars: String,             // characters in the file
    pub render: String,            // characters rendered on the screen
    pub highlight: Vec<Highlight>, // highlight for each character in 'render'
}

const TABSTOP: u16 = 8;

impl EditRow {
    fn render_chars(chars: &str) -> String {
        let mut idx = 0;
        let mut render = String::new();
        for c in chars.chars() {
            if c == '\t' {
                render.push(' ');
                idx += 1;
                while idx % TABSTOP != 0 {
                    render.push(' ');
                    idx += 1;
                }
            } else {
                render.push(c);
                idx += 1;
            }
        }
        render
    }

    pub fn update_row(&mut self) {
        self.render = Self::render_chars(&self.chars);
        self.update_syntax();
    }

    pub fn new(chars: String) -> Self {
        let mut newrow = Self {
            chars,
            render: String::new(),
            highlight: Vec::new(),
        };
        newrow.update_row();
        newrow
    }

    /* Loop through all the characters to the left of cx to figure out how
     * many spaces each tab takes. For each character, if it’s a tab, use
     * rx % TAB_STOP to find out how many columns we are is to the right
     * of the last tab stop, and then subtract that from TAB_STOP - 1 to
     * find out how many columns we are to the left of the next tab stop.
     * Add that amount to rx to get just to the left of the next tab stop,
     * and then the unconditional rx +=1 statement gets us right on the
     * next tab stop. Notice how this works even if we are currently on
     * a tab stop. Call this function at the top of scroll() to finally
     * set rx to its proper value.
     */
    pub fn cx_to_rx(&self, cx: u16) -> u16 {
        let mut rx = 0;
        for c in self.chars.chars().take(cx as usize) {
            if c == '\t' {
                rx += (TABSTOP - 1) - (rx % TABSTOP);
            }
            rx += 1;
        }
        rx as u16
    }

    /*
     * Loop through the chars string, calculating the current rx value (cur_rx)
     * as we go. But instead of stopping when we hit a particular cx value and
     * returning cur_rx, we want to stop when cur_rx hits the given rx value
     * and return cx. The return statement at the very end is just in case the
     * caller provided an rx that’s out of range, which shouldn’t happen. The
     * return statement inside the for loop should handle all rx values that
     * are valid indexes into render.
     */
    pub fn rx_to_cx(&self, rx: u16) -> u16 {
        let mut cur_rx = 0;
        let mut cx = 0;
        for c in self.chars.chars() {
            if c == '\t' {
                cur_rx += (TABSTOP - 1) - (cur_rx % TABSTOP);
            }
            cur_rx += 1;
            if cur_rx > rx {
                return cx;
            }
            cx += 1;
        }
        cx
    }

    pub fn insert_char(&mut self, idx: usize, ch: char) {
        self.chars.insert(idx, ch);
        self.update_row();
    }

    pub fn append_str(&mut self, s: &str) {
        self.chars.push_str(s);
        self.update_row();
    }

    pub fn delete_char(&mut self, idx: usize) {
        if idx >= self.chars.len() {
            return;
        }
        self.chars.remove(idx).to_string();
        self.update_row();
    }

    // Splits the current EditRow object based on index to 'chars' and returns a new one
    pub fn split(&mut self, at: usize) -> Self {
        let right = self.chars.split_off(at);
        self.update_row();
        Self::new(right)
    }

    fn update_syntax(&mut self) {
        self.highlight = vec![Highlight::Normal; self.render.len()];
        let render_chars: Vec<char> = self.render.chars().collect();
        let mut i = 0;
        let mut prev_sep = true;

        while i < render_chars.len() {
            let c = render_chars[i];
            let prev_hl = if i > 0 {
                self.highlight[i - 1]
            } else {
                Highlight::Normal
            };

            if (c.is_ascii_digit() && (prev_sep || prev_hl == Highlight::Number))
                || (c == '.' && prev_hl == Highlight::Number)
            {
                self.highlight[i] = Highlight::Number;
                i += 1;
                prev_sep = false;
                continue;
            }
            prev_sep = Self::is_separator(c);
            i += 1;
        }
    }

    pub fn is_separator(ch: char) -> bool {
        ch.is_ascii_whitespace()
            || [
                ',', '.', '(', ')', '+', '-', '*', '/', '=', '~', '%', '<', '>', '[', ']', ';',
            ]
            .contains(&ch)
    }

    pub fn highlight_match(&mut self, start: usize, len: usize) {
        for c in self.highlight[start..start + len].iter_mut() {
            *c = Highlight::Match
        }
    }
}
