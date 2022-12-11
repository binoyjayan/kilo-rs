#[derive(Default)]
pub struct EditRow {
    pub chars: String,  // characters in the file
    pub render: String, // characters rendered on the screen
}

const TABSTOP: usize = 8;

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
    pub fn new(chars: String) -> Self {
        let render = Self::render_chars(&chars);
        Self { chars, render }
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

    pub fn insert_char(&mut self, idx: usize, ch: char) {
        self.chars.insert(idx, ch);
        self.render = Self::render_chars(&self.chars);
    }

    pub fn append_str(&mut self, s: &str) {
        self.chars.push_str(s);
        self.render = Self::render_chars(&self.chars);
    }

    pub fn del_char(&mut self, idx: usize) {
        if idx >= self.chars.len() {
            return;
        }
        self.chars.remove(idx).to_string();
        self.render = Self::render_chars(&self.chars);
    }
}
