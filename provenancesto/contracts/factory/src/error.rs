use super::*;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized! `{err}`")]
    Unauthorized { err: String },

    #[error(" Address {address} is not sub_admin ")]
    NotSubAdmin { address: Addr },

    #[error("Address {address} is not admin ")]
    NotAdmin { address: Addr },

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

    #[error("Address {address} is not present")]
    CanNotRemove { address: Addr },

    #[error("Country_code authentication failed! denom: `{denom}`, address: `{address}`!")]
    CountryCodeAuthorizationFailed { denom: String, address: Addr },

    #[error("Country code: `{code}` already exists!")]
    CountryCodeAlreadyExists { code: u8 },

    #[error("Country code: `{code}` does not exists!")]
    CountryCodeNotExists { code: u8 },

    #[error("Address:`{address}` is not an Issuer!")]
    NotAnIssuer { address: Addr },

    #[error("Address:`{address}` is not a Transfer Agent!")]
    NotATransferAgent { address: Addr },

    #[error("Address:`{address}` is not a Tokenization Agent!")]
    NotATokenizationAgent { address: Addr },

    #[error("Token limit exceeded for address: `{address}`!")]
    TokenLimitExceeded { address: Addr },

    #[error("Balance is frozen for address: `{address}` and denom: `{denom}`!")]
    BalanceFrozen { denom: String, address: Addr },

    #[error("Supply must be greater than token limit!")]
    SupplyUnderFlow {},

    #[error("proposal_id {proposal_id} is invalid ! ")]
    InvalidProposalId { proposal_id: u128 },

    #[error("Address {address} has already approved")]
    AlreadyApproved { address: Addr },

    #[error("Need more than half of signers approval. Current approvals are {approvals}")]
    NotEnoughApproval { approvals: u128 },

    #[error("You are not proposer of the proposal")]
    NotProposer {},

    #[error("Address {addr} already added!")]
    AlreadyAdded { addr: Addr },

    #[error("The proposal with proposal id  {proposal_id} has been cancelled")]
    Cancelled { proposal_id: u128 },

    #[error("Proposal has been expired at {expiration_time}")]
    ProposalExpired { expiration_time: u64 },

    #[error("The proposal with proposal_id {proposal_id} has been complete")]
    Completed { proposal_id: u128 },

    #[error("No denom config found for denom: `{denom}`!")]
    MissingDenomConfig { denom: String },

    #[error("Error while serializing denom: `{denom}` & address: `{address}`!")]
    SerializationFailed { denom: String, address: String },

    #[error("Failed to deserialize into struct!")]
    DeserializationFailed {},

    #[error("Address: `{addr}` doesn't exist!")]
    NotFound { addr: Addr },
}
