use std::collections::HashMap;
use std::io;

use department::Department;

fn main() {
    let mut people_in_departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!(
            "Hi, I created this program to improve my rust knowledge. It has following options:"
        );
        println!("Option 1: add a person to a department.");
        println!("Option 2: list all people of a department.");
        println!("Option 3: list all people in all deptartments.");
        println!("Option 4: exit.");
        println!("Type a number corresponding to the option:");
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Error while reading input.");

        let option: u16 = String::from(option.trim())
            .parse()
            .expect("Your input should be a positive number");

        assert!(option <= 4 && option != 0,);

        if option == 4 {
            break;
        }

        match option {
            1 => {
                Department::print_valid_departments();
                println!(
                    "To add a person to a department use this syntax: Add <Name> to <Department>."
                );
                let mut instruction = String::new();
                io::stdin()
                    .read_line(&mut instruction)
                    .expect("Error while reading input.");

                let instruction = String::from(instruction.trim());
                let words = instruction.split_whitespace().collect::<Vec<_>>();

                assert!(words.len() == 4, "You should provide at least 4 arguments.");

                let name = words.get(1).expect("An error during name retrieval.");
                let department = words.get(3).expect("An error during department retrieval.");
                let department = Department::new(String::from(*department));
                let people = people_in_departments.get_mut(department.value());
                match people {
                    Some(people) => {
                        people.push(String::from(*name));
                    }
                    None => {
                        let mut people: Vec<String> = Vec::new();
                        people.push(String::from(*name));
                        people_in_departments.insert(department.value().clone(), people);
                    }
                }

                println!("Added!");
            }
            2 => {
                Department::print_valid_departments();
                println!("Specify department:");
                let mut department = String::new();
                io::stdin()
                    .read_line(&mut department)
                    .expect("Error while reading input.");

                let department = String::from(department.trim());
                let department = Department::new(department);

                let people = people_in_departments.get(department.value());
                match people {
                    Some(people) => println!("People of department: {:?}", people),
                    None => println!("Department has no one!"),
                }
            }
            3 => println!("People in all departmemnts: {:?}", people_in_departments),
            4 => {}
            _ => println!("Option does not exist!"),
        }
    }
}
