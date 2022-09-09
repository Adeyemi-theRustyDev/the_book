use crate::summary_utils::Summary;
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        String::from(format!("Summary:{}", self.content))
    }

    fn summarize_author(&self) -> String {
        String::from("Don't give af about the author")
    }
}