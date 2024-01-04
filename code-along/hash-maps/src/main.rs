use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 50);

    println!("{}", scores["Blue"]);
    println!("{}", scores.get("Red").copied().unwrap_or(0));

    for (key, value) in &scores {
        println!("{key} {value}");
    }

    let key = "Key".to_string();
    let value = "Value".to_string();

    let mut hashmap = HashMap::new();
    hashmap.insert(&key, &value);

    println!("{key} {value}");

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 50);
    scores.insert("Red", 60);
    for (key, value) in &scores {
        println!("{key} {value}");
    }
    println!("{:?}", scores);

    scores.entry("Yellow").or_insert(100);
    scores.entry("Red").or_insert(100);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
