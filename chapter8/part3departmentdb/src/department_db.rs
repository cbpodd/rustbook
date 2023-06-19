use crate::{prelude::*, error::{WrongWordError, WrongNumberOfWordsError}};

use std::{collections::HashMap, str::SplitWhitespace};

use derive_getters::{Getters, Dissolve};

use crate::utils::not_empty_string::NotEmptyString;

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

pub fn parse_employee_from_input(input: &str) -> Result<Employee> {
    let mut words_in_input = input.split_whitespace();
    const EXPECTED_NUM_WORDS: usize = 4;

    let first_word = words_in_input.next().ok_or(
        Error::WrongNumberOfWords {
            expected_num_words: EXPECTED_NUM_WORDS,
            actual_num_words: 0 })?;

    expect_next_word_is(&mut words_in_input, "Add", EXPECTED_NUM_WORDS, 0)?;
    let employee_name = NotEmptyString::from(next_word_is(&mut words_in_input)?);
    expect_next_word_is(&mut words_in_input, "to")?;
    let department = NotEmptyString::from(next_word_is(&mut words_in_input)?);

    Ok(Employee::new(employee_name, department))
}

fn expect_next_word_is(
    words: &mut SplitWhitespace,
    expected: &str,
    expected_num_words: usize,
    actual_num_words: usize) -> Result<()> {
    let next_word = words.next().ok_or(
        Error::WrongNumberOfWords {
            expected_num_words,
            actual_num_words })?;

    if next_word != expected {
        return Err(Error::WrongWord {
            expected_word: expected.to_owned(),
            actual_word: next_word.to_owned(),
        });
    }

    Ok(())
}
