extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};
use std::collections::HashMap;

mod website;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.rust-lang.org")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
