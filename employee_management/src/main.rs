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
    
    loop {
        println!("");
        println!("Welcome! Which action do you want to perform?");
        println!("1. View all employees of the company.");
        println!("2. View all employees of chosen department.");
        println!("3. Add an employee.");
        println!("4. Exit the program.");
        println!("");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input = match input.trim().parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                println!("Please put in a number");
                continue;
            },
        };

        match input {
             1 => employees_by_department(&map),
             2 => all_employees(&map),
             3 => add_employee(&mut map),
             4 => break,
             _ => {
                println!("Please choose one of the options");
                continue;
            }
        }
    }
}

fn employees_by_department(map: &HashMap<String, String>) {

}

fn all_employees(map: &HashMap<String, String>) {

}

fn add_employee(map: &mut HashMap<String, String>) {
    let mut name = String::new();
    let mut department = String::new();

    println!("Name of employee:");
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");

    println!("Department of employee:");
    io::stdin().read_line(&mut department)
        .expect("Failed to read line");

    let name = name.trim();
    let department = department.trim();

    map.insert(name.to_string(), department.to_string());
    println!("Employee {} added to {}", name, department);
}