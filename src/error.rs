use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError{

    #[error("InvalidFillAmount: {0}")]
    InvalidFillAmount(String)
}