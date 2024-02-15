use blog::Post;

fn main() {
    let mut post = Post::new();

    // Draft state
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    println!("Draft state {}", post.content());

    // Request review state
    post.request_review();
    post.add_text("blah");
    assert_eq!("", post.content());
    println!("Request review state {}", post.content());

    // Scheduled state
    post.approve();
    post.add_text("blah");
    assert_eq!("", post.content());
    println!("Scheduled state {}", post.content());

    // Approved state
    post.approve();
    post.add_text("blah");
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("Approved state {}", post.content());
}
