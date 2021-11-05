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

    let comments = get_opex_comments(&website_controller, false).await?;
    //println!("COMMENTS === {:?}", comments);
    Ok(website_controller)
}

pub async fn get_opex_comments(
    mut websites_controller: &WebsiteList,
    is_debug_display: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut html = String::new();

    let mut comment_id: u32 = 0;
    let mut Comments: Vec<Comment> = vec![];

    for article in &websites_controller.articles {
        let url = article.get_url();
        let document = get_document(url).await?;

        document.find(Class("comment")).for_each(|comment| {
            comment.find(Class("comment-body")).for_each(|body| {
                let mut comment_data: Comment = Comment {
                    id: 0,
                    author: String::from("NA"),
                    content: String::from("NA"),
                    date: String::from("NA"),
                    children: vec![],
                };
                comment_data.id = comment_id;
                /*
                let mut comment_author = String::new();
                let mut comment_content = String::new();
                let mut comment_date = String::new();
                */

                body.find(Class("comment-author")).for_each(|author| {
                    author.find(Name("cite")).for_each(|author_name| {
                        comment_data.author = author_name.text();
                    })
                });

                body.find(Class("commentmetadata"))
                    .for_each(|comment_metadata| {
                        comment_metadata.find(Name("a")).for_each(|date| {
                            comment_data.date = date.text();
                        })
                    });

                body.find(Name("p")).for_each(|content| {
                    //COMMENT CONTENT
                    comment_data.content = content.text();
                    //println!("COMMENT CONTENT :: {:?}", content.text());
                });

                Comments.push(comment_data);
                comment_id = comment_id + 1; //DEV PURP NO REAL ID
            });
            if is_debug_display {
                println!("{:?}", comment);
            }
        });
    }

    Comments.iter().for_each(|comment| {
        println!("COMMENT DATA :: {:?}", comment);
    });

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
