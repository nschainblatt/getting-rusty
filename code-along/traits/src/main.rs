use traits::{notify, NewsArticle, Summary, TextMessage, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("bob"),
        content: String::from("I love Rust"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("The Rust Programming Language"),
        location: String::from("New York"),
        author: String::from(&tweet.username),
        content: String::from("The Rust Programming Language is a high-level, imperative, object-oriented, functional programming language developed by <NAME> and <NAME>."),
    };

    let text = TextMessage {
        sender: String::from("Alice"),
        recipient: String::from("Bob"),
        content: String::from("Hello, Bob!"),
        datetime: String::from("2017-01-01 00:00:00"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());
    println!("1 new text message: {}", text.summarize());
    notify(&tweet);
}
