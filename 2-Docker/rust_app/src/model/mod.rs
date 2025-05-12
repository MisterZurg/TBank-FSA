#[derive(Debug)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
}

impl Note {
    pub fn new(id: i64, title: String, content: String) -> Self {
        Note { id, title, content }
    }
}
