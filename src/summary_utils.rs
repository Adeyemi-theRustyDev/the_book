pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from(format!("(Read more from {}...", self.summarize_author()))
    }
}