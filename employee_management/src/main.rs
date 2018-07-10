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
    let mut map: HashMap<String, String> = HashMap::new();
    
    // Init test values
    map.insert("Kevin".to_string(), "Gamedev".to_string());
    map.insert("Buck".to_string(), "Design".to_string());
    map.insert("Oz".to_string(), "Gamedev".to_string());
    map.insert("Jennifer".to_string(), "Art".to_string());

    loop {
        println!("");
        println!("| Welcome! Which action do you want to perform?");
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
             1 => all_employees(&map),
             2 => employees_by_department(&map),
             3 => add_employee(&mut map),
             4 => break,
             _ => {
                println!("| Please choose one of the options");
                continue;
            }
        }
    }
}

/**
 * Show all employees, by department, sorted alphabetically by name
 */
fn all_employees(map: &HashMap<String, String>) {
    let mut map_by_department: HashMap<String, Vec<String> > = HashMap::new();
    
    println!("");
    println!("|| All employees currently working at your company");
    println!("");

    // Add to another hashmap, with key department
    // and value a vector of employees working that department
    for (key, value) in map.iter() {
        let vec = map_by_department.entry(value.to_string())
            .or_insert(Vec::new());
        vec.push(key.to_string());
        vec.sort();
    }

    for (department, persons) in map_by_department.iter() {
        println!("|| {}", department.to_uppercase());
        for person in persons {
            println!("| {}", person);
        }
        println!("");
    }
}

fn employees_by_department(map: &HashMap<String, String>) {
    let mut department_input = String::new();

    println!("");
    println!("| Department to retrieve names of:");
    println!("");
    io::stdin().read_line(&mut department_input)
        .expect("| Failed to read line");

    let department_input = department_input.trim();
    
    for (name, department) in map.iter() {
        if *department == department_input {
            println!("| {}", name);
        }
    }
}

fn add_employee(map: &mut HashMap<String, String>) {
    let mut name = String::new();
    let mut department = String::new();

    println!("| Name of employee:");
    io::stdin().read_line(&mut name)
        .expect("| Failed to read line");

    println!("| Department of employee:");
    io::stdin().read_line(&mut department)
        .expect("| Failed to read line");

    let name = name.trim();
    let department = department.trim();

    map.insert(name.to_string(), department.to_string());
    println!("| Employee {} added to {}", name, department);
}