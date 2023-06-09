use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // For copyable values, they are copied, for others they are moved and hashmap takes ownership.
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 25); // Value will be overwritten - keys are unique.

    println!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(32); // This inserts a key only if the score isn't there already.
    println!("{:?}", scores); // Unchanged, as blue already existed.

    // By default, hashing is done using a hashing function SipHash. You can change it to anything implementing the BuildHasher trait.
}