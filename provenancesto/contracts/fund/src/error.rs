use super::*;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Argument Mismatched! `{err}`")]
    ArgumentsMismatched { err: String },

    #[error("Address {address} is not admin ")]
    NotAdmin { address: Addr },

    #[error("Address:`{address}` is not a  Agent!")]
    NotAnAgent { address: Addr },

    #[error("Error while serializing denom: `{denom}` & address: `{address}`!")]
    SerializationFailed { denom: String, address: String },

    #[error("Failed to deserialize into struct!")]
    DeserializationFailed {},

    #[error("Address: `{addr}` doesn't exist!")]
    NotFound { addr: Addr },

    #[error("Address: `{addr}` already exists!")]
    AlreadyExists { addr: Addr },
}
