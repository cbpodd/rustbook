fn main() {
    // Vectors are good for resizable lists/arrays.
    let v: Vec<i32> = Vec::new(); // Empty vector

    let v = vec![1, 2, 3]; // Macro to create a vector from an array.

    // Interesting - the type can be inferred from the pushed values.
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    };

    let does_not_exist = v.get(100);

    match does_not_exist {
        Some(dne) => println!("The 100th element is {dne}"),
        None => println!("There is no 100th element"),
    };

    // let does_not_exist = &v[100]; // panics
    // println!("The 100th element is {does_not_exist}");

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    // println!("The first element is: {first}"); // Will not compile, as we got an immutable reference to the vector and then tried to modify it.
    // This doesn't work because allocating another element may require allocating and moving the elements to a new location.
    // If that were the case, the old reference would point to unallocated memory.

    let v = vec![100, 32, 57];

    for i in &v { // i is a reference to reach element.
        println!("{i}");
    }

    for i in v { // In this case, i is just each element. Weird.
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // Need to dereference value to change it.
    }

    // If you need vectors of multiple types, use a vector of enums.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    // Dropping a vector drops its elements. All vectors are dropped here or earlier.
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
