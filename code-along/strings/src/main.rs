fn main() {
    let mut s = String::new();
    
    let data = "initial content";
    let s = data.to_string();

    let s = "initial content".to_string();

    let s = String::from("initial content");

    let s = format!("initial {}", "content");
    
    let s = "initial".to_string() + " " + "content";
    
    let mut s = "initial".to_string();
    s.push_str(" content");
    s.push('s');

    for char in s.chars() {
        println!("{char}");
    }

    for byte in s.bytes() {
        println!("{byte}");
    }

    let slice = &s[0..4];
    println!("{slice}");

    println!("{}", s);
}
