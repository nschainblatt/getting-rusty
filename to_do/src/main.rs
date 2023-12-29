use std::io;
use std::process;

struct Todo {
    name: String,
    todos: [String; 5],
}

impl Todo {
    fn get_position() -> usize {
        println!("Enter the position [1-5]");
        println!();

        loop {
            let mut position = String::new();
            io::stdin()
                .read_line(&mut position)
                .expect("Failed to read line");
            println!();
            let position: usize = match position.trim().parse() {
                Ok(result) => match result {
                    1 => {
                        println!("You've selected the first position");
                        1
                    }
                    2 => {
                        println!("You've selected the second position");
                        2
                    }
                    3 => {
                        println!("You've selected the third position");
                        3
                    }
                    4 => {
                        println!("You've selected the fourth position");
                        4
                    }
                    5 => {
                        println!("You've selected the fifth position");
                        5
                    }
                    _ => {
                        println!("Must enter a position from [1-5]");
                        continue;
                    }
                },
                Err(_) => {
                    println!("Must enter a position from [1-5]\n");
                    continue;
                }
            };
            return position - 1;
        }
    }

    fn add_todo(self: &mut Self) {
        let position = Self::get_position();
        println!("Enter the todo you want to add\n");
        let mut content = String::new();
        io::stdin()
            .read_line(&mut content)
            .expect("Failed to read from line");
        let content = content.trim().to_string();
        self.todos[position] = content;
        println!();
    }

    fn delete_todo(self: &mut Self) {
        let position = Self::get_position();
        self.todos[position] = "".to_string();
        println!();
    }

    fn read_todos(self: &Self) {
        println!("Your todos:");
        let mut empty = true;
        for (i, todo) in self.todos.iter().enumerate() {
            if todo.to_string() != "".to_string() {
                empty = false;
                println!("{}. {}", i + 1, todo);
            }
        }
        if !empty {
            println!();
        }
    }
}

fn create_todo_list(container: &mut Vec<Todo>) {
    println!("Enter a name for the todo list");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim().to_string();

    container.push(Todo {
        name: name,
        todos: [
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ],
    });
    println!();
}

fn delete_todo_list(container: &mut Vec<Todo>) {}

fn list_todo_lists(container: &Vec<Todo>) {
    print!("Todo lists: \n");
    let mut empty = true;
    for (i, todo_list) in container.iter().enumerate() {
        if todo_list.name != "".to_string() {
            empty = false;
            println!("{}. {}", i + 1, todo_list.name);
        }
    }
    if !empty {
        println!();
    } else {
        println!("None found, create one!\n");
    }
}

fn select_todo_list(todo_container: &mut Vec<Todo>) {
    println!("Enter the name of the todo list\n");
    let mut selected_todo_list = String::new();

    io::stdin()
        .read_line(&mut selected_todo_list)
        .expect("Failed to read line");
    println!();
    selected_todo_list = selected_todo_list.trim().to_string();

    for todo_list in todo_container.iter_mut() {
        if selected_todo_list == todo_list.name {
            let active_todo_list: &mut Todo = todo_list;
            loop {
                let option = display_individual_todo_menu();
                match option.as_str() {
                    "1" => active_todo_list.add_todo(),
                    "2" => active_todo_list.delete_todo(),
                    "3" => active_todo_list.read_todos(),
                    "4" => break,
                    _ => println!("You entered something not on the menu!"),
                }
            }
            println!();
            return;
        }
    }

    println!("{} does not exist!\n", selected_todo_list);
}

fn display_individual_todo_menu() -> String {
    println!("1. Add a todo");
    println!("2. Delete a todo");
    println!("3. View all todos");
    println!("4. Back\n");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    println!();

    input.to_string()
}

fn display_todo_list_menu() -> String {
    println!("1. Create a todo list");
    println!("2. Delete a todo list");
    println!("3. View all todo lists");
    println!("4. Select a todo list");
    println!("5. Exit\n");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    println!();

    input.to_string()
}

fn main() {
    println!("\nHello, welcome to your todo list!\n");
    println!("You can create many lists, and each list can have a maximum of five todos.\n");

    let mut todo_container: Vec<Todo> = Vec::new();

    loop {
        let option = display_todo_list_menu();
        match option.as_str() {
            "1" => create_todo_list(&mut todo_container),
            "2" => delete_todo_list(&mut todo_container),
            "3" => list_todo_lists(&todo_container),
            "4" => select_todo_list(&mut todo_container),
            "5" => break,
            _ => println!("You entered something not on the menu!\n"),
        }
    }
}
