use thiserror::Error;

#[derive(Debug, Error)]
pub enum FindRequestError {
    #[error("offset value {offset} cannot be less than 1, please choose higher value")]
    PerPageOffsetTooLow { offset: u64 },
    #[error("per_page value {per_page} cannot be less than 1, please choose higher value")]
    PerPageValueTooLow { per_page: u16 },
    #[error("per_page value {per_page} is too high, please choose lower value")]
    PerPageValueTooHigh { per_page: u16 },
}
