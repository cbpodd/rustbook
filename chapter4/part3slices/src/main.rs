fn main() {
    let mut s1 = String::from("The first word is");
    let i1 = first_word_index(&s1);
    println!("The index of the end of the first word in {} is {i1}", s1);
    s1.clear(); // Problem here is that i1 still points to 3, which is useless in this case.

    let s2 = String::from("OneWord");
    let i2 = first_word_index(&s2);
    println!("The index of the end of the first word in {} is {i2}", s2);

    let mut s3 = String::from("hello world");
    // The below are references to part of the string only.
    let hello = &s3[..5]; // Can drop out the 0 if you're starting at the beginning
    let world = &s3[6..]; // Can drop out the end if you are going to the end

    println!("{} {} {}", s3, hello, world);

    let first_word = first_word(&s3); // Returns a reference to the slice of the first word.

    // s3.clear(); // This will not compile because first_word still has a reference to this string.

    println!("{} {}", s3, first_word);

    s3.clear(); // This works here because it is after the last use of first_word

    let s4 = "Hello world"; // String literals are slices - they point to a slice of the binary/memory.
                                  // They are immutable because the binary is immutable.

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// Return the index of the end of the word (indicated by a space)
fn first_word_index(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str { // We can pass a string literal directly
                                 // Or we can pass a reference or slice of a string
                                 // This uses deref coercions, discussed in chapter 15.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
