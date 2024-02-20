use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    let post = post.reject();
    let post = post.request_review();

    let post = post.schedule();
    let post = post.reject();
    let post = post.schedule();

    let post = post.approve();

    println!("Scheduled state {}", post.content());
}
