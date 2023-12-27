struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("nschainblatt"),
        email: String::from("nschainblatt@gmail.com"),
        sign_in_count: 1,
    };

    user1.active = false;

    let username = String::from("nateschain");
    let email = String::from("nateschain@gmail.com");

    let user1 = build_user(username, email);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("madeofamericansteel@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{}", user1.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
