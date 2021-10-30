//WHAT AM I DOING
//FEATURES TO IMPLEMENT:
//RETRIEVE SELECTED DATA FROM WEBSITE USING SCRAPER and REQWEST in async env.
//Save desired data in a database and use it with a webapp (Symfony ?)
//The Rust scraper must run and retrieve data every X minutes / hours

//DOC of libs
//https://docs.rs/crate/scraper/0.12.0
//https://docs.rs/crate/reqwest/0.11.5

//LIB HTML/CSS SELECTOR
//https://github.com/utkarshkukreti/select.rs
//https://crates.io/crates/nos

//use std::collections::HashMap;

mod website;

//use nos::Document;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use website::article::Article;
use website::configuration as WebConfig;
use website::list::WebsiteList;

#[tokio::main]
async fn main() {
    let data: WebConfig::Config = WebConfig::set_configuration("app_config.toml");
    let mut websites = WebsiteList::init();
    websites.set_configuration(data);
    let url = websites.get_element(0);
    let website_list = websites.clone();
    let websites = get_website_html(url, website_list).await;

    websites.ok().iter().for_each(|list| {
        list.articles.iter().for_each(|article| {
            println!("{:?}", article.get_title());
            println!("{:?}", article.get_url());
        })
        //println!("{:?}", article);
    })

    //println!("{:?}", html);
}

async fn get_website_html(
    url: &String,
    mut website_controller: WebsiteList,
) -> Result<WebsiteList, Box<dyn std::error::Error>> {
    //let url = websites.get_element(0); //DEV PURP, OPEX360

    //let article = &websites.articles.len();

    let html = reqwest::get(url).await?.text().await?;

    let document = Document::from(html.as_str());
    //extract URL from homepage.
    document.find(Class("post-title")).for_each(|title| {
        title.find(Name("a")).for_each(|atag| {
            let article_title = atag.text();
            let article_url = String::from(atag.attr("href").unwrap());
            let article = Article::new(article_title, article_url, 0);
            website_controller.add_article(article);
            println!("{:?}", atag.text());
            println!("{:?}", atag.attr("href").unwrap());
        });
    });
    //println!("{:?}", content);

    Ok(website_controller)
}
