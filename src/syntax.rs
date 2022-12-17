use lazy_static::lazy_static;
use std::fmt;

pub type SyntaxFlags = u32;
pub const NUMBERS: SyntaxFlags = 1 << 0;
pub const STRINGS: SyntaxFlags = 1 << 1;

pub enum FileType {
    C,
    Rust,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FileType::C => write!(f, "C"),
            FileType::Rust => write!(f, "RUST"),
        }
    }
}

pub struct Comment {
    _single: Option<String>,
    _multiline: Option<(String, String)>,
}

impl Comment {
    pub fn new(single: Option<&str>, multiline: Option<(&str, &str)>) -> Self {
        Self {
            _single: single.map(|s| s.to_string()),
            _multiline: multiline.map(|(s, e)| (s.to_string(), e.to_string())),
        }
    }
}

pub struct Syntax {
    pub filetype: FileType,
    pub filematch: Vec<String>,
    pub flags: SyntaxFlags,
    pub _comment: Comment,
}

impl Syntax {
    pub fn new(
        filetype: FileType,
        filematch: Vec<&str>,
        flags: SyntaxFlags,
        _comment: Comment,
    ) -> Self {
        Self {
            filetype,
            filematch: filematch.iter().map(|s| s.to_string()).collect(),
            flags,
            _comment,
        }
    }
}

// Syntax highlight database
lazy_static! {
    pub static ref HLDB: Vec<Syntax> = vec![
        Syntax::new(
            FileType::C,
            vec!["c", "h"],
            NUMBERS | STRINGS,
            Comment::new(Some("//"), Some(("/*", "*/")))
        ),
        Syntax::new(
            FileType::Rust,
            vec!["rs"],
            NUMBERS | STRINGS,
            Comment::new(Some("//"), Some(("/*", "*/")))
        ),
    ];
}
