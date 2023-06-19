// lib.rs

pub mod department_db;
pub mod utils;
pub mod error;

mod prelude;

pub use department_db::DepartmentDb;
pub use department_db::Employee;
pub use department_db::parse_employee_from_input;

pub use error::Error;

pub use utils::not_empty_string::NotEmptyString;

#[cfg(test)]
mod tests_common;