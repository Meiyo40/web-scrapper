#[derive(Debug)]
pub struct Comment {
    id: u32,
    author: String,
    content: String,
    date: String,
    children: Vec<Comment>,
}
