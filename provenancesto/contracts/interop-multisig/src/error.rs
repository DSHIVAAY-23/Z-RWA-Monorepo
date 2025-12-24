use super::*;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized caller: `{caller}`!")]
    Unauthorized { caller: Addr },

    #[error("Address {address} is not admin!")]
    NotAdmin { address: Addr },

    #[error("Threshold is not met for the transaction!")]
    ThresholdNotMet {},

    #[error("Same voter can't vote twice!")]
    PermissionDenied {},

    #[error("Not Found!")]
    NotFound {},
}
