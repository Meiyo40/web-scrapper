//WHAT AM I DOING
//FEATURES TO IMPLEMENT:
//RETRIEVE SELECTED DATA FROM WEBSITE USING SCRAPER and REQWEST in async env.
//Save desired data in a database and use it with a webapp (Symfony ?)
//The Rust scraper must run and retrieve data every X minutes / hours

//DOC of libs
//https://docs.rs/crate/scraper/0.12.0
//https://docs.rs/crate/reqwest/0.11.5
//https://github.com/utkarshkukreti/select.rs

//use std::collections::HashMap;

mod website;

use website::configuration as WebConfig;

#[tokio::main]
async fn main() {
    let data: WebConfig::Config = WebConfig::set_configuration("app_config.toml");
    let mut websites = website::list::WebsiteList::init();
    websites.set_configuration(data);
    let html = get_website_html(websites.get_element(0)).await;
    println!("{:?}", html);
}

async fn get_website_html(url: &String) -> Result<std::string::String, Box<dyn std::error::Error>> {
    //let url = websites.get_element(0); //DEV PURP, OPEX360

    //let article = &websites.articles.len();

    //println!("{}", article);

    let html = reqwest::get(url).await?.text().await?;
    Ok(html)
}
