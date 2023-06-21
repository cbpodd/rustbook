// error.rs

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // TODO: Get rid of this.
    #[error("General error: {0}")]
    General(String),
}
