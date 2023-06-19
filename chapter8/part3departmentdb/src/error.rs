// error.rs

use derive_getters::Getters;

#[derive(thiserror::Error, Debug)] // try thiserror
pub enum Error {
    #[error("Wrong number of words found. Expected {expected_num_words:?} words, found {actual_num_words:?} words.")]
    WrongNumberOfWords{
        expected_num_words: usize,
        actual_num_words: usize,
    },

    #[error("Invalid word found. Expected {expected_word:?}, found {actual_word:?}.")]
    WrongWord {
        expected_word: String,
        actual_word: String,
    },
}

#[derive(Debug, Copy, Getters, Clone)]
pub struct WrongNumberOfWordsError {
    expected_num_words: usize,
    actual_num_words: usize,
}

impl WrongNumberOfWordsError {
    pub fn new(expected_num_words: usize, actual_num_words: usize) -> Self {
        Self {
            expected_num_words,
            actual_num_words,
        }
    }
}

#[derive(Debug, Getters, Clone)]
pub struct WrongWordError {
    expected_word: String,
    actual_word: String,
}

impl WrongWordError {
    pub fn new(expected_word: impl Into<String>, actual_word: impl Into<String>) -> Self {
        Self {
            expected_word: expected_word.into(),
            actual_word: actual_word.into(),
        }
    }
}
