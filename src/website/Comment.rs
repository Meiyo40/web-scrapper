#[derive(Debug)]
pub struct Comment {
    id: u32,
    pub author: String,
    pub content: String,
    pub date: String,
    pub children: Vec<Comment>,
}
