use crate::summary_utils::Summary;

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        String::from(format!("Summary: Written by {} Content: {}",
            self.username, self.content
        ))
    }

    fn summarize_author(&self) -> String {
        String::from(format!("{}", self.username))
    }
}
