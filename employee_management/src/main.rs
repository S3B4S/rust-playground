/**
 * Chapter 8, Common Collections
 * https://doc.rust-lang.org/book/second-edition/ch08-03-hash-maps.html#creating-a-new-hash-map
 * 
 * Using a hash map and vectors, create a text interface
 * to allow a user to add employee names to a department in a company. 
 * For example, “Add Sally to Engineering” or “Add Amir to Sales".
 * Then let the user retrieve a list of all people in a department
 * or all people in the company by department, sorted alphabetically.
 */

use std::collections::HashMap;
use std::io;

fn main() {
    let mut map_by_department: HashMap<String, Vec<String>> = HashMap::new();

    // Init test values
    add_employee(&mut map_by_department, "Gamedev".to_string(), "Kevin".to_string());
    add_employee(&mut map_by_department, "Design".to_string(), "Buck".to_string());
    add_employee(&mut map_by_department, "Gamedev".to_string(), "Oz".to_string());
    add_employee(&mut map_by_department, "Art".to_string(), "Jennifer".to_string());

    loop {
        println!("");
        println!("| Welcome! Which action do you want to perform?:");
        println!("| 1. View all employees of the company.");
        println!("| 2. View all employees of chosen department.");
        println!("| 3. Add an employee.");
        println!("| 4. Exit the program.");
        println!("");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("| Failed to read line");

        let input = match input.trim().parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                println!("| Please put in a number");
                continue;
            },
        };

        match input {
             1 => {
                 println!("");
                 println!("|| All employees currently working at your company");
                 println!("");
                 print_all_employees(&map_by_department)
             },
             2 => {
                 println!("");
                 println!("| Department to retrieve names of:");
                 println!("");
                 
                 let mut department = String::new();
                 io::stdin().read_line(&mut department)
                    .expect("| Failed to read line");
                 let department = department.trim().to_string();

                 println!("");
                 employees_by_department(&map_by_department, department)
             },
             3 => {
                 let mut department = String::new();
                 let mut name = String::new();

                 println!("| Name of employee:");
                 io::stdin().read_line(&mut name)
                    .expect("| Failed to read line");

                 println!("| Department of employee:");
                 io::stdin().read_line(&mut department)
                    .expect("| Failed to read line");

                 let department = department.trim().to_string();
                 let name = name.trim().to_string();
                 let name_department = department.to_string();
                 let name_copy = name.to_string();
                 add_employee(&mut map_by_department, department, name);
                 println!("| Employee {} added to {}", name_copy, name_department);
             },
             4 => break,
             _ => {
                println!("| Please choose one of the options");
                continue;
            }
        }
    }
}

/**
 * Show all employees, by department, sorted alphabetically by name.
 */
fn print_all_employees(map: &HashMap<String, Vec<String>>) {
    for (department, persons) in map.iter() {
        println!("|| {}", department.to_uppercase());
        for person in persons {
            println!("| {}", person);
        }
        println!("");
    }
}

/**
 * Show all employees of given department.
 */
fn employees_by_department(map: &HashMap<String, Vec<String>>, department_other: String) {
    println!("|| {}", department_other.to_uppercase());
    for (department, persons) in map.iter() {
        if *department == department_other {
            for person in persons.iter() {
                println!("| {}", person);
            }
        }
    }
}

/**
 * Add an employee to the hashmap.
 * If the department already exists, add the person to the vector.
 * If not, create a new department-persons pair.
 * All vectors remain sorted after an update.
 */
fn add_employee(map: &mut HashMap<String, Vec<String>>, department: String, name: String) {
    let vec = map.entry(department)
        .or_insert(Vec::new());
    vec.push(name.to_string());
    vec.sort();
}