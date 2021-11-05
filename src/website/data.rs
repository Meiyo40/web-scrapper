#[derive(Debug, Clone)]
///store article data[title, url, nb_comments]
pub struct Article {
    pub title: String,
    pub url: String,
    pub nb_comments: u32,
}

impl Article {
    pub fn new(title: String, url: String, nb_comments: u32) -> Article {
        Article {
            title,
            url,
            nb_comments,
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub id: u32,
    pub author: String,
    pub content: String,
    pub date: String,
    pub children: Vec<Comment>,
}
