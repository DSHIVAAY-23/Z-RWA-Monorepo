use super::*;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized caller: `{caller}`!")]
    Unauthorized { caller: Addr },

    #[error("Address {address} is not admin!")]
    NotAdmin { address: Addr },

    #[error("Address {address} is not executer!")]
    NotAnExecuter { address: Addr },

    #[error("Action is invalid!")]
    InvalidAction {},

    #[error("Conversion Error! type:`{err}`!")]
    ConversionError { err: String },
}

impl From<FromStrRadixErr> for ContractError {
    fn from(err: FromStrRadixErr) -> Self {
        ContractError::ConversionError {
            err: err.to_string(),
        }
    }
}

impl From<FromHexError> for ContractError {
    fn from(err: FromHexError) -> Self {
        ContractError::ConversionError {
            err: err.to_string(),
        }
    }
}

impl From<hex::FromHexError> for ContractError {
    fn from(err: hex::FromHexError) -> Self {
        ContractError::ConversionError {
            err: err.to_string(),
        }
    }
}

impl From<ParseIntError> for ContractError {
    fn from(err: ParseIntError) -> Self {
        ContractError::ConversionError {
            err: err.to_string(),
        }
    }
}
