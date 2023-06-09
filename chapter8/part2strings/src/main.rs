fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string(); // Available on any type that implements Display trait

    let s = "initial contents".to_string(); // Works on the lieteral directly as well.

                                                    // This is the same as String::from, so either can be used.
    let mut s = String::from("foo");
    s.push_str("bar"); // takes a slice because we don't want to own the parameter.
    s.push('l'); // Pushes a single char

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 has been moved and can no longer be used. This is done because it is WAY more efficient.

    // Concatonation can get unwieldy if you concat multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // Kinda weird - difficult to read

    let s1 = String::from("tic"); // Have to remake s1 as its value has been dropped into s.

    let s = format!("{s1}-{s2}-{s3}"); // Way easier to read. Similar to println!(), but returns the value in a string. I'd actually guess println!() uses this.

    let s1 = String::from("hello");
    // let h = s1[0]; // This will not compile. This is because some unicode takes more than one u8 byte. "Hola" has length 4, but
                      // "Здравствуйте" has length 24 (as each character takes up 2 bytes). returning the first byte of that won't
                      // give you a correct answer. Rust prevents this by not allowing indexing via a single integer at all.

    let h = &s1[..1]; // Return the first byte of the string.

    // The way to iterate over strings is to be explicit in whether you want characters or bytes.
    let hello = String::from("Здравствуйте");

    // let he = &hello[..1]; // This will panic at runtime because we try to slice across a character boundary.

    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}"); // This returns the bytes of the string, but unicode scalar values may be made up of more than one byte.
    }
}
