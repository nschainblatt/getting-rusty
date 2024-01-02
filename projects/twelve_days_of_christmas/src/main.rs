use std::io;

fn main() {
    println!("Enter p to print the lyrics to 'The Twelve Days of Christmas'!");
    println!("Enter q to quit.");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from line.");

        if input.trim().to_ascii_lowercase() == "q" {
            println!("\nDone.");
            break;
        } else if input.trim().to_ascii_lowercase() == "p" {
            println!();
            assemble_lyrics();
        } else {
            println!("\nInvalid input.\n");
        }

        println!("Enter p to print the lyrics to 'The Twelve Days of Christmas'!");
        println!("Enter q to quit.");
    }
}

fn assemble_lyrics() {
    let gifts: [&str; 12] = [
        "And a partridge in a pear tree!",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let day_nums: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    print_lyrics(gifts, day_nums);
}

fn print_lyrics(gifts: [&str; 12], day_nums: [&str; 12]) {
    for i in 0..12 {
        println!("On the {} day of Christmas,", day_nums[i]);
        println!("my true love gave to me");

        for j in (0..=i).rev() {
            if i == 0 {
                println!("A {}", &gifts[j][6..]);
            } else {
                println!("{}", gifts[j]);
            }
        }

        println!();
    }
}
