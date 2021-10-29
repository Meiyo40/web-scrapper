#[derive(Debug)]
pub struct WebsiteList {
    website_to_call: Vec<String>,  // will contain website to scrap
    url_from_website: Vec<String>, //will contains url's captured from website (ex: the 11 article links from homepage of opex360)
    comments: Vec<String>,         // will contain comments
}

impl WebsiteList {
    ///return the element at index X in the website_to_call attr
    pub fn get_element(&self, idx: usize) -> &String {
        &self.website_to_call[idx]
    }

    ///Return the List struct initialized with basic website list.
    pub fn init() -> WebsiteList {
        WebsiteList {
            website_to_call: vec![String::from("http://www.opex360.com/")],
            url_from_website: vec![],
            comments: vec![],
        }
    }

    pub fn add_website(&mut self, website: String) {
        self.website_to_call.push(website);
    }
}
