use std::io;

fn main() {
    println!("Get the nth Fibonacci number.");
    println!("Enter q to quit.\n");

    loop {
        println!("Enter a number:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from line.");

        if input.trim() == "q" {
            println!("\nDone.");
            break;
        }

        let input: u128 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease enter a positive integer!\n");
                continue;
            }
        };

        println!(
            "The {input} fibonacci number is: {}\n",
            nth_fibonacci_number(input)
        );
    }
}

fn nth_fibonacci_number(num: u128) -> u128 {
    // Fibonacci is F(n) = F(n - 1) + F(n - 2)
    // Given F(0) = 0 and F(1) == 1

    if num == 0 {
        return 0;
    }

    if num == 1 {
        return 1;
    }

    let mut current = 0;
    let mut previous: u128 = 0;
    let mut next: u128 = 1;

    for _index in 1..num {
        current = previous + next;
        previous = next;
        next = current;
    }

    current
}
