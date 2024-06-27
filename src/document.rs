#[derive(Clone)]
pub struct Document {
    title: String,
    content: String,
}

impl Document {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}
