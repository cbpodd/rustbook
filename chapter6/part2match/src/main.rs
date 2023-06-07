fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;

    match dice_roll {
        3 => println!("Get fancy hat"),
        7 => println!("Lose fanyc hat"),
        other => println!("Move {other} spaces forward"), // Other is a variable for all other matches.
    };

    let dice_roll = 9;

    match dice_roll {
        3 => println!("Get fancy hat"),
        7 => println!("Lose fancy hat"),
        _ => (), // _ catches all values and does not bind to them. () is the unit value (nothing happens)
    };
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}