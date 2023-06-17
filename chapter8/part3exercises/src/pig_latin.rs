const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn to_pig_latin(input: &str) -> String {
    let mut is_first_char = true;
    let mut first_char = 'h';
    let mut pig_latin = String::new();

    for c in String::from(input).chars() {
        if is_first_char {
            if !VOWELS.contains(&c) {
                first_char = c;
            } else {
                pig_latin.push(c);
            }

            is_first_char = false;
            continue;
        }

        pig_latin.push(c);
    }

    let suffix = format!("-{first_char}ay");
    pig_latin.push_str(suffix.as_str());
    pig_latin
}