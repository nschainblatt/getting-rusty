enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(self: &Self) {}
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    let quarter = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(quarter));

    let five = Some(5);
    let none: Option<i32> = None;
    let six = plus_one(five);
    let none = plus_one(none);

    let config_max: Option<u8> = Some(3);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    };
}
