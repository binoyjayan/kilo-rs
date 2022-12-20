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
    pub single: Option<String>,
    pub _multiline: Option<(String, String)>,
}

impl Comment {
    pub fn new(single: Option<&str>, multiline: Option<(&str, &str)>) -> Self {
        Self {
            single: single.map(|s| s.to_string()),
            _multiline: multiline.map(|(s, e)| (s.to_string(), e.to_string())),
        }
    }
}

pub enum Keyword {
    Base(String),
    Type(String),
}

pub struct Syntax {
    pub filetype: FileType,
    pub filematch: Vec<String>,
    pub flags: SyntaxFlags,
    pub comment: Comment,
    pub keywords: Vec<Keyword>,
}

impl Syntax {
    pub fn new(
        filetype: FileType,
        filematch: Vec<&str>,
        flags: SyntaxFlags,
        comment: Comment,
        keywords: Vec<Keyword>,
    ) -> Self {
        Self {
            filetype,
            filematch: filematch.iter().map(|s| s.to_string()).collect(),
            flags,
            comment,
            keywords,
        }
    }
}

// Syntax highlight database
lazy_static! {
    pub static ref HLDB: Vec<Syntax> = vec![
        Syntax::new(
            FileType::C,
            vec!["c", "h", "cc", "cpp", "hpp"],
            NUMBERS | STRINGS,
            Comment::new(Some("//"), Some(("/*", "*/"))),
            vec![
                Keyword::Base("switch".into()),
                Keyword::Base("if".into()),
                Keyword::Base("else".into()),
                Keyword::Base("for".into()),
                Keyword::Base("while".into()),
                Keyword::Base("break".into()),
                Keyword::Base("continue".into()),
                Keyword::Base("case".into()),
                Keyword::Base("default".into()),
                Keyword::Base("do".into()),
                Keyword::Base("goto".into()),
                Keyword::Base("return".into()),
                Keyword::Base("const".into()),
                Keyword::Base("enum".into()),
                Keyword::Base("struct".into()),
                Keyword::Base("union".into()),
                Keyword::Base("typedef".into()),
                Keyword::Base("sizeof".into()),
                Keyword::Base("volatile".into()),
                Keyword::Base("register".into()),
                Keyword::Base("static".into()),
                Keyword::Base("extern".into()),
                Keyword::Base("inline".into()),
                Keyword::Base("asm".into()),
                Keyword::Base("class".into()),
                Keyword::Base("public".into()),
                Keyword::Base("private".into()),
                Keyword::Base("protected".into()),
                Keyword::Base("new".into()),
                Keyword::Base("delete".into()),
                Keyword::Base("operator".into()),
                Keyword::Base("template".into()),
                Keyword::Base("this".into()),
                Keyword::Base("friend".into()),
                Keyword::Base("virtual".into()),
                Keyword::Base("try".into()),
                Keyword::Base("throw".into()),
                Keyword::Base("catch".into()),
                Keyword::Type("void".into()),
                Keyword::Type("auto".into()),
                Keyword::Type("char".into()),
                Keyword::Type("int".into()),
                Keyword::Type("short".into()),
                Keyword::Type("long".into()),
                Keyword::Type("signed".into()),
                Keyword::Type("unsigned".into()),
                Keyword::Type("float".into()),
                Keyword::Type("double".into()),
            ],
        ),
        Syntax::new(
            FileType::Rust,
            vec!["rs"],
            NUMBERS | STRINGS,
            Comment::new(Some("//"), Some(("/*", "*/"))),
            Vec::new(),
        ),
    ];
}
