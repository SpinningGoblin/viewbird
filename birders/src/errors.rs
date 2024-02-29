use thiserror::Error;

#[derive(Error, Debug)]
pub enum BirderError {
    #[error("Unknown error occurred - {message}")]
    UnknownError { message: String },
    #[error("Error during request to ebird API")]
    EBirdRequestError(#[from] reqwest::Error),
}
