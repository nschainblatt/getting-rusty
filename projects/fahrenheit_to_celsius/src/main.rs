use std::io;

fn main() {
    println!("Fahrenheit to celcius.");
    println!("Enter q to quit.\n");

    loop {
        println!("Enter a temperature in fahrenheit:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        if input.trim() == "q" {
            println!("\nDone.");
            break;
        }

        let input: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease enter a integer!\n");
                continue;
            }
        };

        println!(
            "{input} is {} degrees celsius\n",
            convert_to_fahrenheit(input)
        );
    }
}

fn convert_to_fahrenheit(temp: i64) -> i64 {
    (temp - 32) * 5 / 9
}
