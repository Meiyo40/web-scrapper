use super::article;
use super::comment;

#[derive(Debug)]
///Store Website data, links and comments struct
pub struct WebsiteList {
    website_to_call: Vec<String>,        // will contain website to scrap
    url_from_website: Vec<String>, //will contains url's captured from website (ex: the 11 article links from homepage of opex360)
    pub comments: Vec<comment::Comment>, // will contain comments
    pub articles: Vec<article::Article>,
}

impl WebsiteList {
    ///return the element at index X in the website_to_call attr
    pub fn get_element(&self, idx: usize) -> &String {
        &self.website_to_call[idx]
    }

    ///Return the List struct initialized with website list.
    pub fn init() -> WebsiteList {
        WebsiteList {
            website_to_call: vec![String::from("http://www.opex360.com/")],
            url_from_website: vec![],
            comments: vec![],
            articles: vec![],
        }
    }

    #[allow(dead_code)]
    ///Add the given webstring as a String in the website_to_call list
    pub fn add_website(&mut self, website: String) {
        self.website_to_call.push(website);
    }
}
