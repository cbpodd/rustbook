fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Maximum configured to {}", max),
        None => (), // Annoying boilerplate code, but required to add.
    };

    if let Some(max) = config_max { // Syntactic sugar over match - unit value for all others.
        println!("Maximum configured to {}", max);
    } else {
        println!("No maximum"); // Can include an else case
    }
}
