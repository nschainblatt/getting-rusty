fn get_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    let a = "Word.";
    let b = "This is a sentence.";
    
    // a and b are string literals which are equivalent to a string slice
    let index_a = get_word(&a[..]); // string slice
    let index_b = get_word(b); // string literal

    println!("{}", index_a);
    println!("{}", index_b);
}
