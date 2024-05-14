use thiserror::Error;

/// All of the errors that could happen while using the birders crate.
#[derive(Error, Debug)]
pub enum BirderError {
    #[error("Unknown error occurred - {message}")]
    UnknownError { message: String },
    #[error("Error during request to ebird API")]
    EBirdRequestError(#[from] reqwest::Error),
}
