use super::*;

/// The struct containing instructions for creating tokens
#[account]
#[derive(Debug, Default)]
pub struct CreateTokenParams {
    /// Unique id
    pub id: String,

    /// Token Name
    pub name: String,

    /// Token Symbol
    pub symbol: String,

    /// Token URI
    pub uri: String,

    /// Decimals
    pub decimals: u8,

    /// Issuer with mint, burn, freeze, unfreeze and force transfer rights
    pub issuer: Pubkey,

    /// Transfer Agent with freeze, unfreeze and force transfer rights
    pub transfer_agent: Pubkey,

    /// Issuer with mint and burn rights
    pub tokenization_agent: Pubkey,

    /// Holding Period
    pub holding_period: i64,
}

/// The struct containing instructions for mint and burn tokens
#[account]
#[derive(Debug, Default)]
pub struct TokenParams {
    /// Token Name
    pub name: String,

    /// To Account
    pub to_account: Pubkey,

    /// Amount of tokens to be minted.
    pub amount: u64,
}

/// The struct containing instructions for orders
#[account]
pub struct RequestOrder {
    /// Order Id
    pub order_id: u128,

    /// Token Name
    pub token: String,

    /// User
    pub user: Pubkey,

    /// Amount of tokens to be ordered.
    pub amount: u64,

    // Request Type
    pub request_type: RequestType,
}

/// The struct containing instructions for transferring tokens
#[account]
#[derive(Debug, Default)]
pub struct TransferParams {
    /// Token Name
    pub token: String,

    /// To Token
    pub to_account: Pubkey,

    /// Amount of tokens to be transferred
    pub amount: u64,
}

/// The struct containing instructions for force transferring tokens
#[account]
#[derive(Debug, Default)]
pub struct ForceTransferParams {
    /// Token Name
    pub token: String,

    /// From Account
    pub from_account: Pubkey,

    /// To Account
    pub to_account: Pubkey,

    /// Amount of tokens to be transferred
    pub amount: u64,
}

/// The struct containing instructions for partial freeze
#[account]
#[derive(Debug, Default)]
pub struct PartialFreezeParams {
    /// Token Name
    pub token: String,

    /// Amount of tokens to be freezed
    pub amount: u64,
}
