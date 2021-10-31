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
use website::configuration as WebConfig;
use website::list::WebsiteList;
use website::website_getter;

#[tokio::main]
async fn main() {
    let data: WebConfig::Config = WebConfig::set_configuration("app_config.toml");
    let mut websites = WebsiteList::init();
    websites.set_configuration(data);
    let url = websites.get_element(0);
    let website_list = websites.clone();
    let websites = website_getter::get_opex_website_article(url, website_list).await;

    /*
    websites.ok().iter().for_each(|list| {
        list.articles.iter().for_each(|article| {
            println!("{:?}", article.get_title());
            println!("{:?}", article.get_url());
        })
        //println!("{:?}", article);
    })
    */

    //println!("{:?}", html);
}
