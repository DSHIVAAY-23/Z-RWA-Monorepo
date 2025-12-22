use super::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum CoinType {
    Dai,
    Usdt,
    Usdc,
}

/// Asset Type, use to choose between different assets
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum UpdateType {
    Add { address: Pubkey },
    Remove,
}
