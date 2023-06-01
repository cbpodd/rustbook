fn main() {
    let s = "hello"; // String literal, value hardcoded into the text of our program
    // String literals are immutable

    let s = String::from(s);
    println!("{}", s);

    let s2 = s; // s is no longer valid
    // println!("{}", s) // Will not compile - s is no longer valid;
    // Rust doesn't really have shallow and deep copies
    // Because the first reference is invalidated, this is a "move", not
    // a shallow copy.
    // Deep copies never happen implicitly - they must happen explicitly.
    // This ensures copies are always fast.

    let s3 = s2.clone(); // This is a deep copy - done using the clone method (clonable trait?)
    // Clone is an indicator that arbitrary code is being called
    // and the operation may be expensive.
    println!("{}, {}", s2, s3);

    let x = 5;
    let y = x;

    println!("{x}, {y}");
    // This works because x and y are trivial values stored on the stack.
    // They implement the Copy trait - meaning that there is no difference
    // Between deep and shallow copying.
    // This allows copying to work.
    // We cannot implement copy if a type, or any of its parts, have implemented
    // the drop trait (deallocation on the heap).
    // Scalar values or tuples can implement copy.

    // ------------- Ownership and functions ----------------

    let s = String::from("Hello"); // s comes into scope.

    takes_ownership(s); // S moves to the function
    // println!("{}", s); // this will not compile - s has moved to the function, and is no longer valid here.

    let x = 5; // x comes into scope.

    makes_copy(x); // Because x is a trivial value implementing copy, x is still valid.
    println!("x is still valid and is {x}");

    let s1 = gives_ownership(); // Ownership of s1 given here.
    let s2 = takes_and_gives_ownership(s1); // Ownership of s1 is given to the function
                                                             // Ownership is then given back to the main function
                                                             // and placed in the possesion of the new variable s1.
                                                             // Interestingly, the old s2 is also dropped here.
    println!("{}", s2);
} // x is popped off the stack here. s1 is also dropped here.

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here some_string goes out of scope and drop is called. The backing memory is freed.

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}

fn gives_ownership() -> String { // Will move ownership of its return value to the calling function
    let some_string = String::from("yours"); // some_string comes into scope.
    some_string // some_string is returned and will give scope back to the calling function.
}

fn takes_and_gives_ownership(a_string: String) -> String { // ownership is transferred to the function for a_string
    a_string // ownership is given back to the caller by returning.
}