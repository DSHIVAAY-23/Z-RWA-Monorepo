use super::*;

#[error_code]
pub enum CustomError {
    #[msg("Error: Your balance is not enough!")]
    InsufficientFunds,

    #[msg("Error: Unauthorized User!")]
    Unauthorized,
}
