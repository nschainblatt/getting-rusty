use ::std::io;

fn turn_pig_latin(word: &str) -> String {
    const CONSONANTS: [char; 21] = [
        'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W',
        'X', 'Y', 'Z',
    ];
    const VOWELS: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
    let first_letter = word.as_bytes()[0] as char;

    let mut pig_word = String::new();
    if CONSONANTS.contains(&first_letter.to_ascii_uppercase()) {
        // if first letter is consonant, then interate until the first vowel is found
        for (i, letter) in word.chars().enumerate() {
            if VOWELS.contains(&letter.to_ascii_uppercase()) {
                let end = format!("-{}ay", &word[0..i]);
                pig_word = format!("{}{}", &word[i..], &end);
                break;
            }
        }
    } else {
        // if first letter is vowel, then add hay to the end of the word
        pig_word = format!("{}-hay", word);
    }
    return pig_word;
}

fn main() {
    let mut word = String::new();

    println!("Enter the word to turn into pig latin");
    println!(
        "This program assumes you are using ASCII characters (normal characters on a keyboard)\n"
    );
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    word = word.trim().to_string();

    println!("{}\n", turn_pig_latin(&word));
}
