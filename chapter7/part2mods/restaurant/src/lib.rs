// This lets the compiler know that there is a front_of_house module in this project
mod front_of_house;

// Like a symbolic link in filesystem - brings hosting to the top level of the crate
// Hosting is now a valid name, but inside hosting we still need to qualify with hosting::
// Sort of like hosting -> crate::front_of_house::hosting
// It is generally bad form to qualify all the way to a function - only local functions should be used that way.
use crate::front_of_house::hosting;

// This redefines a child module into the parent module of the crate. When this is exported, it will be seen as a public top-level module
// Useful when your users think of code organization differently than you do.
pub use crate::front_of_house::serving;

pub use back_of_house::{Breakfast, types::Appetizer}; // This allows you to bring in many types at the same time. Self specifies the module itself.

use std::collections::*; // Specifies that all public items defined in a path should be brought into scope.

pub mod customer {
    use crate::serving;
    use super::back_of_house::{self, Breakfast as Bs, types::Appetizer as Appses};
    use super::hosting; // This references the use in the parent scope (use crate::front_of_house::hosting)
    use super::back_of_house::Breakfast; // It is generally okay to bring enums, structs, and other items all the way in.
    use super::back_of_house::types::Appetizer as Apps; // This is a way to give a new name to something.

    pub fn eat_at_restaurant() {
        // Absolute path
        hosting::add_to_waitlist(); // this no longer works, as usings only work for the scope they are defined in (not child or parent scopes)

        // Order a breakfast in the summer with Rye toast.
        let mut meal = Breakfast::summer("Rye");
        // Change our mind about what bread we would like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // Next line will not compile if we uncomment it.
        // We cannot see of motify seasonal fruit that comes with the meal.
        // meal.seasonal_fuit = String::from("blueberries");

        // let order1 = Appetizer::Soup; // This won't work, it is not in scope
        let order2 = Apps::Salad; // This will, as Apps is what we have in scope.

        serving::serve_order();
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fuit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fuit: String::from("peaches"),
            }
        }
    }

    pub mod types {
        pub enum Appetizer { // All variants of a pub enum are pub
            Soup,
            Salad,
        }
    }
}