use std::io;
use std::collections::HashMap;

struct Departments {
    engineering: Vec<String>,
    sales: Vec<String>,
    accounting: Vec<String>,
}

impl Departments {
    fn add_to_department(&mut self, department: &str, name: String) {
        match department {
            "Sales" => self.sales.push(name),
            "Engineering" => self.engineering.push(name),
            "Accounting" => self.accounting.push(name),
            _ => ()
        }
    }
}

fn main() {

    let mut first_message = String::new();

    first_message += "\n\nWelcome to enslavery! \nAdd your employee to a department....
    \n 1. Add someone to Sales \n 2. Add someone to Engineering \n 3. Add someone to Accounting\n 4. List a department";

    println!("{}", first_message);

    let mut choice = String::new();
    
    loop {
        io::stdin() 
            .read_line(&mut choice).expect("Please put something valid.");

        let choice: u32 = choice.trim().parse().expect("Enter a valid number");

        if (1..=3).contains(&choice) {
            break add_and_save(choice);
        }
        else if choice == 4{
            let departments = add_and_save(choice);
            retrieve(&departments);
        }
        else {
            continue;
        }
    };
}


fn retrieve(department: &Departments) {
    let employee_hashmap = HashMap::<&str, Departments>::new();
    
    //The department as a key should be the name of the
    //employee

    //How should I make the value the department of the 
    //employee?
    let new_department = employee_hashmap.insert;
    //Use insert here for 
    for (key, value) in &employee_hashmap {
    println!("{} is in {}", key, value);
    }
}

fn add_and_save(num: u32) -> Departments {

    let mut departments = Departments {
        engineering: Vec::new(),
        sales: Vec::new(),
        accounting: Vec::new()

    };
    print!("Name: ");
    
    let mut employee = String::new();

    io::stdin()
        .read_line(&mut employee).expect("Message here");
    
    match num {
        1 => departments.add_to_department("Sales", employee), 
        2 => departments.add_to_department("Engineering", employee),
        3 => departments.add_to_department("Accounting", employee),       
        _ => ()
    };
    // Add the instances of the department and the employee

    departments



    /*
    To do list:
    1. Continue on with making the hashmap of the employee and department
        Some things to consider:
        How do we pass in the struct and the name at once?
        How do we push it in like the PSET?
    2. Try to match the departments and the employee name
    3. Continue with making the retrieve function

     */
}
