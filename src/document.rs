pub struct Document {
    pub title: String,
    pub content: String,
}

impl Document {
    pub fn new(title: String, content: String) -> Document {
        Document { title, content }
    }
}

//implement display trait for Document
use std::fmt;

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Title: {}\nContent: {}", self.title, self.content)
    }
}
