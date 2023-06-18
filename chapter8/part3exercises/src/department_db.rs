use std::{collections::HashMap, error::Error, fmt::Display};

use derive_getters::{Getters, Dissolve};

use crate::not_empty_string::NotEmptyString;

pub struct DepartmentDb {
    departments: HashMap<NotEmptyString, Vec<NotEmptyString>>,
}

impl DepartmentDb {
    // Gets a single department.
    pub fn get_department(&self, dept_name: &NotEmptyString) -> Option<&Vec<NotEmptyString>> {
        self.departments.get(dept_name)
    }

    pub fn add_person_to_department(
        &mut self,
        person: NotEmptyString,
        department: NotEmptyString) {
        self.departments
            .entry(department)
            .or_insert(Vec::new())
            .push(person);
    }

    pub fn add_employee(&mut self, employee: Employee) {
        let (employee_name, employee_department) = employee.dissolve();
        self.add_person_to_department(employee_name, employee_department);
    }

    pub fn get_all_current_employees(&self) -> Vec<Employee> {
        let mut employees = self.create_current_employees_vec();
        employees.sort_by(|a, b| a.name.cmp(&b.name));
        employees
    }

    fn create_current_employees_vec(&self) -> Vec<Employee> {
        let mut employees = Vec::new();
        for (department_name, employee_records) in &self.departments {
            for employee in employee_records {
                employees.push(Employee::new(employee.clone(), department_name.clone()));
            }
        }

        employees
    }
}

#[derive(Getters, Dissolve)]
pub struct Employee {
    name: NotEmptyString,
    department: NotEmptyString,
}

impl Employee {
    fn new(name: NotEmptyString, department: NotEmptyString) -> Self {
        Self {
            name,
            department,
        }
    }
}

#[derive(Debug)]
pub struct InvalidInputError;

impl Display for InvalidInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Input Error")
    }
}

impl Error for InvalidInputError {}

pub fn parse_employee_from_input(input: &str) -> Result<Employee, InvalidInputError> {
    let mut words_in_input = input.split_whitespace();

    if words_in_input.clone().count() != 4 {
        return Err(InvalidInputError);
    }

    if words_in_input.next() != Some("Add") {
        return Err(InvalidInputError);
    }

    let employee_name = match words_in_input.next() {
        Some(employee_name) => NotEmptyString::from(employee_name),
        None => return Err(InvalidInputError),
    };

    if words_in_input.next() != Some("to") {
        return Err(InvalidInputError);
    }

    let department= match words_in_input.next() {
        Some(employee_name) => NotEmptyString::from(employee_name),
        None => return Err(InvalidInputError),
    };

    Ok(Employee::new(employee_name, department))
}
