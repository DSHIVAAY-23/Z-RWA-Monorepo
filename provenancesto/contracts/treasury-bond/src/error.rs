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

    #[error("Error while serializing key1: `{key_1}` & key2: `{key_2}`!")]
    SerializationFailed { key_1: String, key_2: String },

    #[error("Failed to deserialize into struct!")]
    DeserializationFailed {},

    #[error("Address: `{addr}` doesn't exist!")]
    NotFound { addr: Addr },

    #[error("Missing Token: `{denom}` Configuration!")]
    ConfigNotFound { denom: String },

    #[error("Address: `{addr}` already exists!")]
    AlreadyExists { addr: Addr },
}
