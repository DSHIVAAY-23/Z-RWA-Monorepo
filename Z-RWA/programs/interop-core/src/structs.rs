use super::*;

/// The struct containing instructions for contract initialization
#[account]
pub struct InitParams {
    pub multisig: Pubkey,
    pub deployed_chain: String,
}

/// Order
#[account]
pub struct Order {
    pub order_id: u128,
    pub token: String,
    pub user: Pubkey,
    pub amount: u64,
}

#[account]
pub struct SendParams {
    pub portfolios: Vec<Portfolio>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Eq, PartialEq)]
pub struct Portfolio {
    pub dest_chain: String,
    pub dest_address: String,
    pub investor: String,
    pub token: String,
    pub amount: String,
    pub order_id: u128,
    pub action: Action,
}
