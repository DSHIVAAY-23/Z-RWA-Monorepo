use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid ZK Proof")]
    InvalidProof {},

    #[error("Compliance Check Failed")]
    ComplianceFailed {},
    
    #[error("Verification Error: {0}")]
    VerificationError(String),
}
