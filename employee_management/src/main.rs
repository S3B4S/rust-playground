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
    let mut employees: HashMap<&str, &str> = HashMap::new();
    
    loop {
        println!("");
        println!("Welcome! Which action do you want to perform?");
        println!("1. View all employees of the company.");
        println!("2. View all employees of chosen department.");
        println!("3. Add an employee.");
        println!("");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let mut input = match input.trim().parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                println!("Please put in a number");
                continue;
            },
        };

        match input {
             1 => employeesByDepartment(),
             2 => allEmployees(),
             3 => addEmployee(),
             _ => {
                println!("Please choose one of the options");
                continue;
            }
        }
    }
}

fn employeesByDepartment() {

}

fn allEmployees() {

}

fn addEmployee() {

}