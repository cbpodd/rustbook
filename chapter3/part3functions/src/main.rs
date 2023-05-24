fn main() {
    println!("Hello, world!");

    another_function(3);
    another_function(5);

    print_labeled_measurement(3, 'h');
    print_labeled_measurement(5, 'w');

    statements();
    expressions();

    let x = five();
    println!("The value of x is: {x}");

    let plus_one = increment(x);
    println!("The value of x after incrementing is: {plus_one}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statements() {
    // Statement
    let _y = 6;

    // Will not compile - statements return no value.
    // let x = (let y = 6);

    // Also does not compile - cannot assign two values.
    // x = y = 6;
}

fn expressions() {
    // Anything that isn't a statement is an expression - most code is
    let y = {
        let x = 3;
        x + 1 // Expressions do not include ending semicolons. Only statements have that. Weird.
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn increment(x: i32) -> i32 {
    x + 1
}