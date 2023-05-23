fn main() {
    arrays();
    tuples();
    floatingpoint();
    numeric_ops();
    booleans();
    characters();
    int_overflow();
}

fn arrays() {
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("months: {:?}", months);

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("b: {:?}", b);

    let c = [0; 23];
    println!("c: {:?}", c);
}

fn tuples() {
    let strongly_named_tuple: (f32, f64, u8) = (500.0, 6.4, 1);
    let first_element = strongly_named_tuple.0;
    let second_element = strongly_named_tuple.1;
    let third_element = strongly_named_tuple.2;

    println!("({first_element}, {second_element}, {third_element})");

    let inferred_tuple = (500, 6.4, 1);
    let (x, y, z) = inferred_tuple;

    println!("({x}, {y}, {z})");
}

fn floatingpoint() {
    let x = 2.0;
    let y: f32 = 3.0;
    println!("X: {x}, Y: {y}");
}

fn numeric_ops() {
    // addition
    let sum = 5 + 10;
    println!("Sum: {sum}");

    // Subtraction
    let difference = 95.5 - 4.3;
    println!("Difference: {difference}");

    // Multiplication
    let product = 4 *30;
    println!("Product: {product}");

    // Division
    let quotient = 56.7 / 32.2;
    println!("Quotient: {quotient}");
    let truncated = -5 / 3;
    println!("Truncated: {truncated}");

    // Remainder
    let remainder = 43 % 5;
    println!("Remainder: {remainder}");
}

fn booleans() {
    let t = true;
    println!("True: {t}");

    let f: bool = false;
    println!("False: {f}");
}

fn characters() {
    let c = 'z';
    println!("C: {c}");

    let x: char = 'Z';
    println!("x: {x}");

    let heart_eyed_cat = '😻';
    println!("Heart-Eyed Cat: {heart_eyed_cat}");
}

fn int_overflow() {
    let original: i8 = 100;

    let wrapped = original.wrapping_mul(2);
    println!("Wrapped: {wrapped}");

    let checked = original.checked_mul(2);
    match checked {
        Some(checked) => println!("Checked: {checked}"),
        None => println!("Check is None"),
    }

    let (over_flowed_result, has_over_flow) = original.overflowing_mul(2);
    println!("Overflowed: {over_flowed_result}, Did Overflow: {has_over_flow}");

    let saturated = original.saturating_mul(2);
    println!("Saturated: {saturated}");

    // let panicked = original * 2;
    // println!("Panicked: {panicked}");
}
