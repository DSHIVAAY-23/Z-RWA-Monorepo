use super::*;

/// The struct containing instructions for creating tokens
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Default, Clone)]
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

    /// token limit for each token holder (eg. token limit for each user = 1000,
    /// users can only hold up to 1000 tokens.
    pub token_limit: u64,

    /// Country code
    pub country_codes: Vec<u16>,

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

    /// Token Name
    pub to_account: Pubkey,

    /// Amount of tokens to be minted.
    pub amount: u64,
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

/// The struct containing instructions for whitelisting
#[account]
#[derive(Debug, Default)]
pub struct WhitelistParams {
    /// Token Name
    pub token: String,

    /// User to be whitelisted
    pub user: Pubkey,

    /// Country Code
    pub code: u16,
}

/// The struct containing instructions for blacklisting
#[account]
#[derive(Debug, Default)]
pub struct BlacklistParams {
    /// Token Name
    pub token: String,

    /// User to be whitelisted
    pub user: Pubkey,
}

/// The struct containing instructions for partial freeze
#[account]
#[derive(Debug, Default)]
pub struct PartialFreezeParams {
    /// Token Name
    pub token: String,

    /// Country Code
    pub amount: u64,
}
