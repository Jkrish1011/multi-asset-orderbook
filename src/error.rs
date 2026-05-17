use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("InvalidFillAmount: {0}")]
    InvalidFillAmount(String),

    #[error("Duplicate Order: {0}")]
    DuplicateOrder(String),

    #[error("Cancel Order: {0}")]
    CancelOrder(String),

    #[error("Add Order Error: {0}")]
    AddOrderError(String),
}
