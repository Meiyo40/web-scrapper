use super::data::*;
use super::list::WebsiteList;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

///Retrieve HTML DOC from given URL
async fn get_document(url: &String) -> Result<Document, Box<dyn std::error::Error>> {
    let html = reqwest::get(url).await?.text().await?;

    let document = Document::from(html.as_str());

    Ok(document)
}

///SPECIFIC GETTER FROM OPEX WEBSITE TO RETRIEVE ARTICLE LIST
///3rd paramter is_debug_display display or not every title/url registred, in the console
pub async fn get_opex_website_article(
    url: &String,
    mut website_controller: WebsiteList,
    is_debug_display: bool,
) -> Result<WebsiteList, Box<dyn std::error::Error>> {
    //let url = websites.get_element(0); //DEV PURP, OPEX360

    //let article = &websites.articles.len();

    //let html = reqwest::get(url).await?.text().await?;

    //let document = Document::from(html.as_str());

    let document = get_document(&url).await?;
    //extract URL from homepage.
    document.find(Class("post-title")).for_each(|title| {
        title.find(Name("a")).for_each(|atag| {
            let article_title = atag.text();
            let article_url = String::from(atag.attr("href").unwrap());
            let article = Article::new(article_title, article_url, 0);

            if is_debug_display {
                println!("TITLE: {:?}", article.get_title());
                println!("URL: {:?}", article.get_url());
            }

            website_controller.add_article(article);
        });
    });
    //println!("{:?}", content);
    //let comments = get_opex_comments(&website_controller, true).await?;
    //println!("COMMENTS === {:?}", comments);
    Ok(website_controller)
}

pub async fn get_opex_comments(
    mut websites_controller: &WebsiteList,
    is_debug_display: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut html = String::new();

    for article in &websites_controller.articles {
        let url = article.get_url();
        let document = get_document(url).await?;

        document.find(Class("comment")).for_each(|ol_comment| {
            if is_debug_display {
                println!("{:?}", ol_comment);
            }
        })
    }

    /*

    websites_controller.articles.iter().for_each(|article| {
        let url = article.get_url();
        let document = get_document(url).await?;

        document
            .find(Class("commentList"))
            .for_each(|ol_comment| {})
    });
    */

    Ok(html)
}
