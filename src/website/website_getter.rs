use super::data::*;
use super::list::WebsiteList;
use select::document::Document;
use select::node::Node;

#[allow(unused_imports)]
use select::predicate::{Attr, Class, Name, Predicate};

///Retrieve HTML DOC from given URL
async fn get_document(url: &String) -> Result<Document, Box<dyn std::error::Error>> {
    let html = reqwest::get(url).await?.text().await?;

    let document = Document::from(html.as_str());

    Ok(document)
}

///SPECIFIC GETTER FROM OPEX WEBSITE TO RETRIEVE ARTICLE LIST
pub async fn get_opex_website_article(
    url: &String,
    mut website_controller: WebsiteList,
    is_debug_display: &bool,
) -> Result<WebsiteList, Box<dyn std::error::Error>> {
    let document = get_document(&url).await?;
    //extract URL from homepage.
    document.find(Class("post-title")).for_each(|title| {
        title.find(Name("a")).for_each(|atag| {
            let article_title = atag.text();
            let article_url = String::from(atag.attr("href").unwrap());
            let article = Article::new(article_title, article_url, 0);

            if *is_debug_display {
                println!("TITLE: {:?}", article.get_title());
                println!("URL: {:?}", article.get_url());
            }

            website_controller.add_article(article);
        });
    });

    let _comments = get_opex_comments(&website_controller, is_debug_display).await?;

    Ok(website_controller)
}

///RETRIEVE ALL COMMENTS OF OPEX360 (FROM THE 11 ARTICLE OF HOMEPAGE)
pub async fn get_opex_comments(
    websites_controller: &WebsiteList,
    is_debug_display: &bool,
) -> Result<Vec<Comment>, Box<dyn std::error::Error>> {
    let mut comment_list: Vec<Comment> = vec![];

    for article in &websites_controller.articles {
        let url = article.get_url();
        let document = get_document(url).await?;

        document.find(Name("ol")).for_each(|li_1| {
            let comment = get_comment(&li_1).unwrap();
            comment_list.push(comment);
        });
    }

    if *is_debug_display {
        comment_list.iter().for_each(|comment| {
            println!("###COMMENT DATA###");
            println!(
                "ID: {}, AUTHOR: {}, DATE: {}, CONTENT:\n {:?}",
                comment.id, comment.author, comment.date, comment.content
            );
            println!("###CHILDREN###");
            comment.children.iter().for_each(|child| {
                println!(
                    "ID: {}, AUTHOR: {}, DATE: {}, CONTENT:\n {:?}",
                    child.id, child.author, child.date, child.content
                );
            });
        });
    }

    Ok(comment_list)
}

///GET GET COMMENTS AND THEIR CHILDRENS FROM GIVEN DIV, SET FOR opex360
fn get_comment(node: &Node) -> Result<Comment, Box<dyn std::error::Error>> {
    let mut comment = Comment {
        id: 0,
        author: String::from("NA"),
        content: String::from("NA"),
        date: String::from("NA"),
        children: vec![],
    };

    //GET CHILDREN ELEMENTS FROM COMMENT IF EXIST
    node.find(Class("children")).for_each(|children_list| {
        let l = get_comment(&children_list).unwrap();
        comment.children.push(l);
    });

    node.find(Class("comment-body")).for_each(|body| {
        let id: Vec<&str> = body.attr("id").unwrap().split("div-comment-").collect();
        let id = id.get(1).unwrap().parse::<u32>().unwrap();

        comment.id = id;

        //RETRIEVE COMMENT AUTHOR FROM HTML
        body.find(Class("comment-author")).for_each(|author| {
            author.find(Name("cite")).for_each(|author_name| {
                comment.author = author_name.text();
            })
        });

        //RETRIEVE COMMENT POST DATE FROM HTML
        body.find(Class("commentmetadata"))
            .for_each(|comment_metadata| {
                comment_metadata.find(Name("a")).for_each(|date| {
                    comment.date = date.text();
                })
            });

        //RETRIEVE COMMENT CONTENT FROM HTML
        body.find(Name("p")).for_each(|content| {
            comment.content = content.text();
        });
    });

    Ok(comment)
}
