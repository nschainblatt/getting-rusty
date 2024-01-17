// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

fn type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let num = 3;
    call_me(&num);
    type_of(&num);
}

fn call_me(num: &i32) {
    for i in 0..*num {
        println!("Ring! Call number {}", i + 1);
    }
}
