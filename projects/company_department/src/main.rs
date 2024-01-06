use ::std::io;

struct Department {
    name: String,
    employees: Vec<String>,
}

impl Department {
    fn list_employees(self: &mut Self) {
        println!("Employees in {}", self.name);
        for employee in self.employees.iter() {
            println!("  - {employee}");
        }
        println!();
    }

    fn add_employee(self: &mut Self) {
        println!("Enter the employee's name to add");
        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read from line");
        println!();

        name = name.trim().to_string();

        self.employees.push(name);
    }

    fn remove_employee(self: &mut Self) {
        self.list_employees();
        println!("Enter the employee's name to remove");
        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read from line");
        println!();

        name = name.trim().to_string();

        for i in 0..self.employees.len() {
            if name == self.employees[i] {
                self.employees.remove(i);
                println!("{} has been removed from {}\n", name, self.name);
                return;
            }
        }
        println!("{} not found in {}\n", name, self.name);
    }

    fn department_menu(self: &mut Self) {
        loop {
            println!("Department Menu");
            println!("1. Add employee");
            println!("2. List employees");
            println!("3. Remove employee");
            println!("4. Back");

            let mut selection: String = String::new();

            io::stdin()
                .read_line(&mut selection)
                .expect("Failed to read line");
            println!();

            let selection: &str = selection.trim();

            match selection {
                "1" => self.add_employee(),
                "2" => self.list_employees(),
                "3" => self.remove_employee(),
                "4" => return,
                _ => {
                    println!("You entered something not on the menu\n");
                }
            };
        }
    }
}

fn list_departments(c: &mut Vec<Department>) {
    println!("Departments");
    for department in c.iter() {
        println!("  - {}", department.name);
    }
    println!();
}

fn create_department(c: &mut Vec<Department>) {
    println!("Enter a name for the department");
    let mut name = String::new();
    let employees: Vec<String> = Vec::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!();

    name = name.trim().to_string();

    c.push(Department { name, employees });
}

fn select_department(c: &mut Vec<Department>) {
    list_departments(c);
    println!("Enter the name of the department to select");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!();

    name = name.trim().to_string();

    for department in c.iter_mut() {
        if name == department.name {
            department.department_menu();
            return;
        }
    }

    println!("Department not found");
}

fn delete_department(c: &mut Vec<Department>) {
    list_departments(c);
    println!("Enter the department name to remove");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!();

    name = name.trim().to_string();

    for i in 0..c.len() {
        if name == c[i].name {
            c.remove(i);
            println!("{} has been deleted\n", name);
            return;
        }
    }
    println!("{} does not exist\n", name);
}

fn sort_alphabetically(l: &mut Vec<String>) {
    // list of names: String
    // bubble sort
    for i in 0..l.len() - 1 {
        for j in 0..l.len() - i - 1 {
            if &l[j][..].chars().nth(0) > &l[j + 1][..].chars().nth(0) {
                let temp = l[j].clone();
                l[j] = l[j + 1].to_string();
                l[j + 1] = temp.to_string();
            }
        }
    }
}

fn list_all_employees(c: &mut Vec<Department>) {
    let mut all_employees: Vec<String> = Vec::new();
    for department in c.iter() {
        for employee in department.employees.iter() {
            all_employees.push(employee.to_string());
        }
    }
    sort_alphabetically(&mut all_employees);

    println!("List of all employees in each department sorted alphabetically");
    for employee in all_employees.iter() {
        println!("  - {employee}");
    }
    println!();
}

fn main() {
    // Create a CLI program that allows users to add employees to a department.
    // Then allow then to view all employees in a department.
    // Also allow then to view all employees in the company (every department combined).
    println!("Welcome to the employee directory\n");
    let mut company: Vec<Department> = Vec::new();

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
