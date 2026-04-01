use super::*;

#[error_code]
pub enum CustomError {
    #[msg("Error: Your balance is not enough!")]
    InsufficientFunds,

    #[msg("Error: Amount can't be zero!")]
    AmountCantBeZero,

    #[msg("Error: Unauthorized User!")]
    Unauthorized,

    #[msg("Error: Token is on Hold!")]
    TokenHeld,
    
    #[msg("Error: Account is frozen!")]
    AccountFrozen,

    #[msg("Error: Balance is frozen!")]
    BalanceFrozen,

    #[msg("Error while converting address!")]
    AddressConversionError,

    #[msg("Error while decoding hex!")]
    HexDecodeError,

    #[msg("Payload is invalid!")]
    InvalidPayload,

    #[msg("Error: Invalid SP1 Proof!")]
    InvalidSP1Proof,

    #[msg("Error: Invalid Public Values length!")]
    InvalidPublicValues,

    #[msg("Error: Identity has not been verified!")]
    UnverifiedIdentity,
}
