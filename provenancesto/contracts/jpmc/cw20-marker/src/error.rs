use super::*;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Error: Unauthorized! {err}")]
    Unauthorized { err: String },

    #[error("Error: Address {address} is not sub_admin!")]
    NotSubAdmin { address: Addr },

    #[error("Error: Address {address} is not an operator!")]
    NotAnOperator { address: Addr },

    #[error("Error: Address {address} is not admin ")]
    NotAdmin { address: Addr },

    #[error("Error: Address {address} is not present")]
    CanNotRemove { address: Addr },

    #[error("Error: Balance is frozen for address: `{address}` and denom: `{denom}`!")]
    BalanceFrozen { denom: String, address: Addr },

    #[error("Error: Address {addr} already added!")]
    AlreadyAdded { addr: Addr },

    #[error("Error: Error while serializing denom: `{denom}` & address: `{address}`!")]
    SerializationFailed { denom: String, address: String },

    #[error("Error: Failed to deserialize into struct!")]
    DeserializationFailed {},

    #[error("Error: Address: `{addr}` doesn't exist!")]
    NotFound { addr: Addr },

    #[error("Error: Amount must be greater than zero!")]
    AmountCannotBeZero {},

    #[error("Error: Balance: `{bal}` must be greater than amount: `{cap}`!")]
    BalanceLow { bal: Uint128, cap: Uint128 },

    #[error("Request: {request_id} already exists!")]
    RequestExists { request_id: String },

    #[error("Error: Request: {request_id} does not exist!")]
    RequestNotExists { request_id: String },

    #[error("Error: Request status is invalid: `{req}`!")]
    InvalidRequestStatus { req: String },

    #[error("Error: Request type is invalid: `{typ}`!")]
    InvalidRequestType { typ: String },

    #[error("Error: Not a Responder!")]
    NotAResponder {},

    #[error("Error: Allowance: `{allowance}` too low from amount: `{amount}`!")]
    AllowanceTooLow { allowance: Uint128, amount: Uint128 },

    #[error("Error: Allowance not found for sender: `{owner}` and spender: `{spender}`!")]
    AllowanceNotFound { owner: Addr, spender: Addr },
}
