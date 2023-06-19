// error.rs

#[derive(thiserror::Error, Debug)]
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
