fn main() {
    let mut count = 0;

    'counting_up: loop {
        let mut remaining: u8 = 10;

        loop {
            println!("Remaining: {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
        println!("Count: {count}");
    }

    let mut count = 0;
    
    // This will run five times
    while count < 5 {
        println!("{count}");
        count += 1;
    }

    let a: [u8; 5] = [1, 2, 3, 4, 5];

    for number in (1..4).rev() {
        println!("{number}");
    }

}
