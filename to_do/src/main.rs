use std::io;

fn display_menu() -> String {
    println!("1. Add a todo");
    println!("2. Edit a todo");
    println!("3. Delete a todo");
    println!("4. View all todos");
    println!("5. Exit\n");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    println!();

    input.to_string()
}

fn add_todo(list: &mut [String]) {
    println!("Enter the position you want to add below! [1-5]");
    println!("...\n");

    // Check if user entered a number, and then subtract one to get the index
    list[0] = String::from("Test123");
    list[1] = String::from("Test123");
    list[2] = String::from("Test123");
    list[3] = String::from("Test123");
    list[4] = String::from("Test123");
}

fn edit_todo(list: &mut [String]) {
    println!("Enter the todo you want to edit below! [1-5]");
    println!("...\n");
}

fn delete_todo(list: &mut [String]) {
    println!("Enter the todo you want to delete below! [1-5]");
    println!("...\n");
}

fn read_todos(list: &[String]) {
    for (i, todo) in list.iter().enumerate() {
        if todo.to_string() != "".to_string() {
            println!("{}. {}", i + 1, todo);
        }
    }
    println!();
}

fn main() {
    println!("Hello, welcome to your todo list!");
    println!("You can have a max of five todos.\n");

    // Change to vector to allow flexible size
    let mut list: [String; 5] = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];

    loop {
        let option = display_menu();
        match option.as_str() {
            "1" => add_todo(&mut list),
            "2" => edit_todo(&mut list),
            "3" => delete_todo(&mut list),
            "4" => read_todos(&list),
            "5" => break,
            _ => println!("You entered something not on the menu!"),
        }
    }
}
