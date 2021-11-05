use super::configuration;
use super::data::*;

#[derive(Debug, Clone)]
///Store Website data, links and comments struct
pub struct WebsiteList {
    pub website_to_call: Vec<String>,  // will contain website to scrap
    pub url_from_website: Vec<String>, //will contains url's captured from website (ex: the 11 article links from homepage of opex360)
    pub comments: Vec<Comment>,        // will contain comments
    pub articles: Vec<Article>,
}

impl WebsiteList {
    ///return the element at index X in the website_to_call attr
    pub fn get_element(&self, idx: usize) -> &String {
        &self.website_to_call[idx]
    }

    ///Return the List struct initialized with website list.
    pub fn init() -> WebsiteList {
        WebsiteList {
            website_to_call: vec![],
            url_from_website: vec![],
            comments: vec![],
            articles: vec![],
        }
    }

    ///Prepare the WebsiteList struct with data from the app_config file
    pub fn set_configuration(&mut self, configuration: &configuration::Config) {
        for site in &configuration.website {
            self.add_website(site.url.clone());
            println!("WEBSITE ADDED: {}", site.url);
        }
    }

    ///Add the given webstring as a String in the website_to_call list
    pub fn add_website(&mut self, website: String) {
        self.website_to_call.push(website);
    }

    pub fn add_article(&mut self, article: Article) {
        self.articles.push(article);
    }
}
