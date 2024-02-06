use add_one;
use add_two;

fn main() {
    println!("1 + 1 = {}", add_one::add_one(1));
    println!("1 + 2 = {}", add_two::add_two(1));
}
