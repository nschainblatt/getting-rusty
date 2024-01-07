fn main() {
    panic!("Crash and burn.");
    
    // Accessing an index out of bounds causing a panic
    let v = vec![1, 2, 3];

    v[99];
}
