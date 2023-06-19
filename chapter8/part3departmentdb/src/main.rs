// main.rs

use std::io;

use part3departmentdb::{DepartmentDb, NotEmptyString};

fn main() {
    let db = user_seed_db();

    println!("Printing all employees from Database.");

    for employee in db.get_all_current_employees() {
        println!("{employee}")
    }

    println!();

    let employees = get_employees_from_department(&db);

    for employee in employees {
        println!("Employee Name: {employee}");
    }
}

fn user_seed_db() -> DepartmentDb {
    let mut db = DepartmentDb::new();

    loop {
        const ADD_EMPLOYEE_PROMPT: &str = "Add a user to the database.\nAdd in format: Add <employee> to <department>\nOr, enter 'q' to quit.";
        let user_input = get_user_input(ADD_EMPLOYEE_PROMPT)
            .expect("Failed to get user input.");

        if user_input.trim() == "q" {
            break;
        }

        let result = add_employee_to_db(&mut db, user_input);

        if let Err(err) = result {
            println!("Failed to parse input: {err}");
            println!("Please try again.");
        }
    }

    println!();

    db
}

fn add_employee_to_db(db: &mut DepartmentDb, user_input: String) -> Result<(), part3departmentdb::Error> {
    let employee = part3departmentdb::parse_employee_from_input(&user_input)?;

    println!("Adding {employee} to the database");
    db.add_employee(employee);

    Ok(())
}

fn get_employees_from_department(db: &DepartmentDb) -> &Vec<NotEmptyString> {
    const GET_EMPLOYEES_PROMPT: &str = "What department would you like to view employees from?";
    let department: NotEmptyString = get_user_input(GET_EMPLOYEES_PROMPT)
        .expect("Failed to get user input")
        .trim()
        .into();

    if department.value().split_whitespace().count() != 1 {
        panic!("Wrong number of words");
    }

    db.get_department(&department)
        .expect("Department does not exist.")
}

fn get_user_input(prompt: &str) -> Result<String, io::Error> {
        let mut user_input = String::new();

        println!("{prompt}");

        io::stdin()
            .read_line(&mut user_input)?;

        Ok(user_input)
}
