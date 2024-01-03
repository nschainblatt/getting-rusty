use std::any::type_name;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn print_type_of<t>(_: &t) {
    println!("{}", type_name::<t>());
}

fn take_ownership(var: i32) {
    println!("took ownership of {}", var);
}

fn main() {
    // let v: vec<i32> = vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);

    let four = v[3];

    take_ownership(four);

    let two = &v[3];

    println!("{} {}", four, two);

    let one = v.get(0);

    match one {
        Some(num) => println!("{}", num),
        None => println!("nothing found"),
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("{}", i);
        *i += 50;
        println!("{}", i);
    }

    print_type_of(&four);
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
