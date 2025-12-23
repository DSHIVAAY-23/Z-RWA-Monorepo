use super::*;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] cw_utils::PaymentError),

    #[error("Unauthorized! `{err}`")]
    Unauthorized { err: String },

    #[error("Token held till: {timestamp}!")]
    TokenHeld { timestamp: u64 },

    #[error(" Address {address} is not sub_admin!")]
    NotSubAdmin { address: Addr },

    #[error(" Address {address} is not an operator!")]
    NotAnOperator { address: String },

    #[error("Address {address} is not admin ")]
    NotAdmin { address: Addr },

    #[error("Address {address} is not present")]
    CanNotRemove { address: Addr },

    #[error("Address:`{address}` is not an Issuer!")]
    NotAnIssuer { address: Addr },

    #[error("Address:`{address}` is not a Transfer Agent!")]
    NotATransferAgent { address: Addr },

    #[error("Address:`{address}` is not a Tokenization Agent!")]
    NotATokenizationAgent { address: Addr },

    #[error("Balance is frozen for address: `{address}` and denom: `{denom}`!")]
    BalanceFrozen { denom: String, address: Addr },

    #[error("Address {addr} already added!")]
    AlreadyAdded { addr: Addr },

    #[error("Error while serializing denom: `{denom}` & address: `{address}`!")]
    SerializationFailed { denom: String, address: String },

    #[error("Failed to deserialize into struct!")]
    DeserializationFailed {},

    #[error("Address: `{addr}` doesn't exist!")]
    NotFound { addr: Addr },

    #[error("Error while decoding `{msg}` from base64!")]
    DecodeError { msg: String },

    #[error("Error while encoding `{msg}` to Binary!")]
    BinaryConversionError { msg: String },

    #[error("Amount must be greater than zero!")]
    AmountCannotBeZero {},

    #[error("Request already exists!")]
    RequestExists {},
}
