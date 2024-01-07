use ::std::collections::HashMap;
use ::std::io;

fn list_employees(d: &str, e: &mut Vec<String>) {
    if e.len() == 0 {
        println!("No employees found in {d}\n");
        return;
    }
    println!("List of employees in {}", d);
    sort_alphabetically(e);
    for employee in e.iter() {
        println!("{employee}");
    }
}

fn add_employee(e: &mut Vec<String>) {
    println!("Enter the name of the employee you want to add");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!();

    name = name.trim().to_string();

    e.push(name);
}

fn remove_employee(d: &str, e: &mut Vec<String>) {
    list_employees(d, e);
    println!("Enter the name of the employee you want to remove");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!();

    name = name.trim().to_string();

    for i in 0..e.len() {
        if name == e[i] {
            e.remove(i);
            println!("{name} has been removed\n");
            return;
        }
    }
    println!("{name} note found\n");
}

fn list_departments(c: &mut HashMap<String, Vec<String>>) {
    if c.is_empty() {
        println!("No departments found\n");
        return;
    }
    println!("Departments");
    for (department, _employees) in c.iter() {
        println!("  - {department}");
    }
    println!();
}

fn create_department(c: &mut HashMap<String, Vec<String>>) {
    println!("Enter a name for the new department");
    let mut name = String::new();
    let mut employees: Vec<String> = Vec::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!();

    name = name.trim().to_string();

    c.insert(name, employees);
}

fn select_department(c: &mut HashMap<String, Vec<String>>) {
    list_departments(c);
    println!("Enter the name of the department to select");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!();

    name = name.trim().to_string();

    for (department, mut employees) in c.iter_mut() {
        if name == *department {
            loop {
                println!("You currently have the {department} department selected\n");
                println!("Department Menu");
                println!("1. Add employee");
                println!("2. List employees");
                println!("3. Remove employee");
                println!("4. Back");

                let mut selection = String::new();
                io::stdin()
                    .read_line(&mut selection)
                    .expect("Failed to read line");
                println!();

                let selection: &str = selection.trim();
                match selection {
                    "1" => add_employee(&mut employees),
                    "2" => list_employees(department, &mut employees),
                    "3" => remove_employee(department, &mut employees),
                    "4" => return,
                    _ => println!("You have entered something not on the menu"),
                }
                println!();
            }
        }
    }
    println!("{name} not found\n");
}

fn delete_department(c: &mut HashMap<String, Vec<String>>) {
    list_departments(c);
    println!("Enter the department name to remove");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!();

    name = name.trim().to_string();

    match c.remove(&name) {
        Some(_value) => println!("The {name} department has been deleted\n"),
        None => println!("The {name} department does not exist\n"),
    };
}

fn sort_alphabetically(l: &mut Vec<String>) {
    if l.len() == 0 {
        return;
    }
    for i in 0..l.len() - 1 {
        for j in 0..l.len() - i - 1 {
            if l[j] > l[j + 1] {
                let temp = l[j].clone();
                l[j] = l[j + 1].to_string();
                l[j + 1] = temp.to_string();
            }
        }
    }
}

fn list_all_employees(c: &mut HashMap<String, Vec<String>>) {
    let mut all_employees: Vec<String> = Vec::new();
    for (_department, employees) in c.iter() {
        for employee in employees.iter() {
            all_employees.push(employee.to_string());
        }
    }
    if all_employees.len() == 0 {
        println!("No employees found in the company\n");
        return;
    }
    sort_alphabetically(&mut all_employees);

    println!("List of all employees in each department sorted alphabetically");
    for employee in all_employees.iter() {
        println!("  - {employee}");
    }
    println!();
}

fn main() {
    println!("Welcome to the employee directory\n");
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Menu");
        println!("1. Create department");
        println!("2. Select department");
        println!("3. List departments");
        println!("4. Delete department");
        println!("5. List all employyees with their departments sorted alphabetically");
        println!("6. Quit");

        let mut selection: String = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");
        println!();

        let selection: &str = selection.trim();

        match selection {
            "1" => create_department(&mut company),
            "2" => select_department(&mut company),
            "3" => list_departments(&mut company),
            "4" => delete_department(&mut company),
            "5" => list_all_employees(&mut company),
            "6" => std::process::exit(0),
            _ => {
                println!("You entered something not on the menu\n");
            }
        };
    }
}
