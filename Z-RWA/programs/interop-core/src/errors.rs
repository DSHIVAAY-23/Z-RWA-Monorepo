use super::*;

#[error_code]
pub enum CustomError {
    #[msg("Error: Your balance is not enough!")]
    InsufficientFunds,

    #[msg("Error: Unauthorized User!")]
    Unauthorized,

    #[msg("Error while converting order_id!")]
    OrderIdConversionError,

    #[msg("Error while converting amount!")]
    AmountConversionError,

    #[msg("Error while converting address!")]
    AddressConversionError,

    #[msg("Error while converting uint!")]
    UintConversionError,

    #[msg("Error while decoding hex!")]
    HexDecodeError,

    #[msg("Error while decoding action!")]
    ActionDecodeError,

    #[msg("Request is invalid!")]
    InvalidRequest,

    #[msg("Action is invalid!")]
    InvalidAction,
}
