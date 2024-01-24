use ::traits2::{notify, Email, NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: "nateschain".to_string(),
        content: "Hello there".to_string(),
        reply: true,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: "Breaking News!".to_string(),
        location: "Amercia".to_string(),
        author: "Bimbimpop".to_string(),
        content: "Florida man saves Christmas".to_string(),
    };

    println!("Lastest from the network: {}", article.summarize());

    let email = Email {
        sender: "nateschain@gmail.com".to_string(),
        recipient: "nschainblatt@gmail.com".to_string(),
        subject: "Greetings".to_string(),
        body: "Hey I wanted to ask you about...".to_string(),
        read: false,
    };

    println!("Unread email: {}", email.summarize());

    notify(&email);
    println!("{email}");
}
