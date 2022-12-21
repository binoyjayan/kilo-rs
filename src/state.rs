use crate::syntax::*;

pub struct RenderState {
    pub syntax: Option<&'static Syntax>,
    pub ml_comment: bool,
}

impl RenderState {
    pub fn new(syntax: Option<&'static Syntax>) -> Self {
        Self {
            syntax,
            ml_comment: false,
        }
    }
}