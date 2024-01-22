mod news_article;
mod summary_utils;
use news_article::NewsArticle;
use summary_utils::Summary;

fn main() {
    let article = NewsArticle {
        headline: String::from("Bomb blast at Ibadan"),
        location: String::from("Ibadan, Oyo state"),
        author: String::from("Adeyemi"),
        content: String::from("Don't really know"),
    };
    
    println!("{}", article.summarize());
}
