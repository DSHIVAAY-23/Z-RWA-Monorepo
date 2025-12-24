use super::*;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    ParseReply(#[from] ParseReplyError),

    #[error("{0}")]
    Instantiate2Address(#[from] Instantiate2AddressError),

    #[error("Unauthorized! `{err}`")]
    Unauthorized { err: String },

    #[error(" Address {address} is not sub_admin!")]
    NotSubAdmin { address: Addr },

    #[error(" Address {address} is not an operator!")]
    NotAnOperator { address: String },

    #[error("Address {address} is not admin ")]
    NotAdmin { address: Addr },

    #[error("Address {address} is not present")]
    CanNotRemove { address: Addr },

    #[error("Balance is frozen for address: `{address}` and denom: `{denom}`!")]
    BalanceFrozen { denom: String, address: Addr },

    #[error("Address {addr} already added!")]
    AlreadyAdded { addr: Addr },

    #[error("Denom {denom} or Address {addr} already exist!")]
    AlreadyExist { denom: String, addr: Addr },

    #[error("Error while serializing denom: `{denom}` & address: `{address}`!")]
    SerializationFailed { denom: String, address: String },

    #[error("Failed to deserialize into struct!")]
    DeserializationFailed {},

    #[error("Address: `{addr}` doesn't exist!")]
    NotFound { addr: Addr },

    #[error("Amount must be greater than zero!")]
    AmountCannotBeZero {},

    #[error("Request already exists!")]
    RequestExists {},

    #[error("Request does not exist!")]
    RequestNotExists {},

    #[error("Request status is invalid!")]
    IncorrectRequestStatus {},

    #[error("Not a Responder!")]
    NotAResponder {},

    #[error("Allowance: `{allowance}` too low from amount: `{amount}`!")]
    AllowanceTooLow { allowance: Uint128, amount: Uint128 },

    #[error("Invalid reply id")]
    InvalidReplyId,
}
