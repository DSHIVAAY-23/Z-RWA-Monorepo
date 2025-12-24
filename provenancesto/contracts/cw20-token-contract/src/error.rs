use super::*;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error(" Address {address} is not sub_admin ")]
    NotSubAdmin { address: Addr },

    #[error("Address {address} is not admin ")]
    NotAdmin { address: Addr },

    #[error("No {denom} tokens sent")]
    EmptyBalance { denom: String },

    #[error("Must unbond at least {min_bonded} {denom}")]
    UnbondTooSmall { min_bonded: Uint128, denom: String },

    #[error("Insufficient balance in contract to process claim")]
    BalanceTooSmall {},

    #[error("No claims that can be released currently")]
    NothingToClaim {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Allowance is expired")]
    Expired {},

    #[error("No allowance for this account")]
    NoAllowance {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Balance is frozen for address: `{address}`!")]
    BalanceFrozen { address: Addr },

    #[error("Invalid expiration value")]
    InvalidExpiration {},

    #[error("Duplicate initial balance addresses")]
    DuplicateInitialBalanceAddresses {},

    #[error("Address {addr} already added!")]
    AlreadyAdded { addr: Addr },

    #[error("Address: `{addr}` doesn't exist!")]
    NotFound { addr: Addr },

    #[error("Token limit exceeded for address: `{address}`!")]
    TokenLimitExceeded { address: Addr },

    #[error("Address:`{address}` is not an Issuer!")]
    NotAnIssuer { address: Addr },

    #[error("Address:`{address}` is not a Transfer Agent!")]
    NotATransferAgent { address: Addr },

    #[error("Address:`{address}` is not a Tokenization Agent!")]
    NotATokenizationAgent { address: Addr },

    #[error("Address {address} has no ADMIN ACCESS")]
    NoAdminAccess { address: Addr },

    #[error("Address {address} has no MINT ACCESS")]
    NoMintAccess { address: Addr },

    #[error("Address {address} has no BURN ACCESS")]
    NoBurnAccess { address: Addr },

    #[error("Address {address} has no DELETE ACCESS")]
    NoDeleteAccess { address: Addr },

    #[error("Address {address} has no DEPOSIT ACCESS")]
    NoDepositAccess { address: Addr },

    #[error("Address {address} has no TRANSFER ACCESS")]
    NoTransferAccess { address: Addr },

    #[error("Address {address} has no UNSPECIFIED ACCESS")]
    NoUnspecifiedAccess { address: Addr },

    #[error("Address {address} has no WITHDRAW ACCESS")]
    NoWithdrawAccess { address: Addr },

    #[error("Address {address} has no FREEZE ACCESS")]
    NoFreezeAccess { address: Addr },

    #[error("Address {address} has no UNFREEZE ACCESS")]
    NoUnfreezeAccess { address: Addr },

    #[error("Address {address} has no FORCETRANSFER ACCESS")]
    NoForceTransferAccess { address: Addr },

    #[error("Error while serializing address: `{address}`!")]
    SerializationFailed { address: String },

    #[error("Failed to deserialize into struct!")]
    DeserializationFailed {},
}

impl From<cw20_base::ContractError> for ContractError {
    fn from(err: cw20_base::ContractError) -> Self {
        match err {
            cw20_base::ContractError::Std(error) => ContractError::Std(error),
            cw20_base::ContractError::Unauthorized {} => ContractError::Unauthorized {},
            cw20_base::ContractError::CannotSetOwnAccount {} => {
                ContractError::CannotSetOwnAccount {}
            }
            cw20_base::ContractError::InvalidZeroAmount {} => ContractError::InvalidZeroAmount {},
            cw20_base::ContractError::Expired {} => ContractError::Expired {},
            cw20_base::ContractError::NoAllowance {} => ContractError::NoAllowance {},
            cw20_base::ContractError::CannotExceedCap {} => ContractError::CannotExceedCap {},
            // This should never happen, as this contract doesn't use logo
            cw20_base::ContractError::LogoTooBig {}
            | cw20_base::ContractError::InvalidPngHeader {}
            | cw20_base::ContractError::InvalidXmlPreamble {} => {
                ContractError::Std(StdError::generic_err(err.to_string()))
            }
            cw20_base::ContractError::InvalidExpiration {} => ContractError::InvalidExpiration {},
            cw20_base::ContractError::DuplicateInitialBalanceAddresses {} => {
                ContractError::DuplicateInitialBalanceAddresses {}
            }
        }
    }
}
