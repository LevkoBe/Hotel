pub struct Document {
    pub title: String,
    pub content: String,
}

#[allow(dead_code)] // todo: do not allow dead code
impl Document {
    pub fn new(title: String, content: String) -> Document {
        Document { title, content }
    }
}

use std::fmt;

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Title: {}\nContent: {}", self.title, self.content)
    }
}
