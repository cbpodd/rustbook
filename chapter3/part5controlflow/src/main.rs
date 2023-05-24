fn main() {
    if_statements();
    loop_practice();
    while_practice();
}

fn is_divisible_by(number: i32, divisor: i32) -> bool {
    number % divisor == 0
}

fn if_statements() {
    let number = 3;
    let cutoff = 5;

    if number < cutoff {
        println!("{number} is less than {cutoff}");
    } else {
        println!("{number} is greater than or equal to {cutoff}");
    }

    // if number {
    //     println!("Will not compile - condition expression must evaluate to a bool");
    // }

    let number = 6; // Reassignment here

    if is_divisible_by(number, 4) {
        println!("number is divisible by 4");
    } else if is_divisible_by(number, 3) {
        println!("number is divisible by 3");
    } else if is_divisible_by(number, 2) {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // let number = if condition { 5 } else { "six" }; // Will not compile b/c types must match

    println!("The value of numebr is: {number}");
}

fn loop_practice() {
    let mut counter = 0;

    let result = loop {
        println!("again!");

        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("Count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End Count: {count}");
}

fn while_practice() {
    let mut number = 3;

    while number > 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() { // Annoying code, and slow. Compiler has to check array boundaries at each time.
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..60).rev() { // This is a range like python
        println!("{number}!");
    }
    println!("LIFTOFF!");
}
