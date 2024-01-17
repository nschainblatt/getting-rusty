// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("Helelo".to_string()));
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
