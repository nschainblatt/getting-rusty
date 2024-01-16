use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct TextMessage {
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub datetime: String,
}

impl Summary for TextMessage {
    fn summarize_author(&self) -> String {
        format!("{}", self.sender)
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", itm.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T>(item: &T)
where
    T: Summary,
{
    println!("Breaking news! {}", item.summarize());
}
