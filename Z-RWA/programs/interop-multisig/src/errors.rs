use super::*;

#[error_code]
pub enum CustomError {
    #[msg("Error: Your balance is not enough!")]
    InsufficientFunds,

    #[msg("Error: Unauthorized User!")]
    Unauthorized,

    #[msg("Error while converting order_id!")]
    OrderIdConversionError,

    #[msg("Same voter can't vote twice!")]
    PermissionDenied,

    #[msg("Error while converting amount!")]
    AmountConversionError,

    #[msg("Error while converting address!")]
    AddressConversionError,

    #[msg("Error while decoding hex!")]
    HexDecodeError,

    #[msg("Error while decoding action!")]
    ActionDecodeError,

    #[msg("Request is invalid!")]
    InvalidRequest,

    #[msg("Action is invalid!")]
    InvalidAction,

    #[msg("Threshold is not met for the transaction!")]
    ThresholdNotMet,
}
