fn main() {
    let s1 = String::from("Hello");

    let length = calculate_length(&s1);

    println!("The length of {} is {length}", s1);

    let mut s2 = String::from("Hello");
    let r1 = &mut s2;
    // let r2 = &mut s2; // This will not work, as you can only have one mutable ref at a time.
    // This is supposed to prevent data races. Even the owner cannot modify
    // the string at this time. For example, the following line will not compile.
    // s2.push_str("string");
    // Right now, only r1 holds the ability to mutate s2.

    change(r1); // You have to make both the variable and the reference mutable

    let mut s3 = String::from("Another string");

    let r2 = &s3;
    let r3 = &s3; // Multiple immutable references are fine.
    // let r4 = &mut s3; // Problem - r2 and r3 don't expect their string
                         // to change from under them.
                         // This will not compile.

    println!("{} {} {}", s3, r2, r3);
    // immutable reference scopes r2 and r3 go out of scope here.

    let r4 = &mut s3; // This is okay because it is after the last use of r2 and r3
    r4.push_str(", More string");
    println!("{}", r4);

    //let r7 = dangle();

    let s9 = no_dangle();
    println!("{}", s9);
}

fn calculate_length(s: &String) -> usize { // the & is a reference
                                           // This allows us to get access to s without
                                           // taking ownership of it.
                                           // This is similar to a pointer in
                                           // that it is a ref/pointer to an object,
                                           // but somehow guarantees safety. I wonder how?
    s.len()
} // Here s goes out of scope, but since is it s reference and not an object
  // proper, it is not dropped.

fn change_immutable(s: &String) {
    // s.push_str(", world"); // This will not work, as you cannot mutate borrowed references
}

fn change(s: &mut String) {
    s.push_str(", world"); // This will work as it's a mutable reference.
}

/* fn dangle() -> &String { // Returns a reference to a string.
    let s = String::from("Hello"); // New string s created

    &s; // Return a reference to the string, s
} // Here s goes out of scope and is dropped. It's memory goes away, and the
  */ // memory becomes a dangling pointer. Danger!

fn no_dangle() -> String {
    let s = String::from("Hello");
    s // Ownership is moved out, and nothing is deallocated
}