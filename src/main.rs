mod summary_utils;
use summary_utils::Tweet;

use crate::summary_utils::Summary;
fn main() {
    let tweet = Tweet {
        username: String::from("YemiTheDev"),
        content: String::from("C# is a nice programming language"),
        reply: true,
        retweet: true
    };

    println!("1 new tweet: {:?}", tweet.summarize());
}