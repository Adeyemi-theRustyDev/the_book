mod tweet;
mod news_article;
mod summary_utils;
use summary_utils::Summary;
use tweet::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("YemiTheDev"),
        content: String::from("C# is a nice programming language"),
        reply: true,
        retweet: true
    };

    println!("1 new tweet: {:?}", tweet.summarize());
}