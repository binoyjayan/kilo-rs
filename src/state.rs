pub struct RenderState {
    pub prev_in_ml_comment: bool, // If the previous line has an open multiline comment
    pub ml_comment_changed: bool, // If row.update_syntax() changed the 'open_ml_comment' state
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            prev_in_ml_comment: false,
            ml_comment_changed: false,
        }
    }
}
