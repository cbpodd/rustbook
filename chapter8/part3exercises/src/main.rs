use std::io;
use part3exercises::{vec_stats, pig_latin};

fn main() {
    test_vec_stats();
    convert_pig_latin();
}

fn convert_pig_latin() {
    let word = get_string_from_user();
    let pig_latin = pig_latin::to_pig_latin(&word);
    println!("Your word as pig latin: {pig_latin}");
}

fn get_string_from_user() -> String {
    let mut input = String::new();

    println!("Please input a word to be converted into pig latin.");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read word");

    let trimmed = input.trim();

    let words = trimmed.split_whitespace();

    if words.count() != 1 {
        panic!("User provided more than one word");
    }

    String::from(trimmed)
}

fn test_vec_stats() {
    test_vec1();
    test_vec2();
    test_vec3();
    test_vec4();
}

fn test_vec1() {
    let list1 = vec![1, 2, 2, 3, 2, 5, 4, 2];
    let mode1 = vec![2];
    let median1 = 2;

    let actual_mode = vec_stats::mode(&list1);
    let actual_median = vec_stats::median(&list1)
        .expect("Should have a median");

    assert_eq!(mode1, actual_mode);
    assert_eq!(median1, actual_median);
}
fn test_vec2() {
    let list = vec![4, 1, 2, 2, 1, 3, 3];
    let mode = vec![1, 2, 3];
    let median = 2;

    let actual_mode = vec_stats::mode(&list);
    let actual_median = vec_stats::median(&list)
        .expect("Should have a median");

    assert_eq!(median, actual_median);
    assert_eq!(mode.len(), actual_mode.len());

    for item in actual_mode {
        assert!(mode.contains(&item));
    }
}

fn test_vec3() {
    let list = vec![3, 1, 2];
    let median = 2;
    let mode = [1, 2, 3];

    let actual_mode = vec_stats::mode(&list);
    let actual_median = vec_stats::median(&list)
        .expect("Should have a median");

    assert_eq!(median, actual_median);
    assert_eq!(mode.len(), actual_mode.len());

    for item in actual_mode {
        assert!(mode.contains(&item));
    }
}

fn test_vec4() {
    let list = Vec::new();
    let mode: Vec<i32> = Vec::new();

    let actual_mode = vec_stats::mode(&list);

    if let Some(_) = vec_stats::median(&list) {
        panic!("Median must be none");
    }

    assert_eq!(mode, actual_mode);
}
