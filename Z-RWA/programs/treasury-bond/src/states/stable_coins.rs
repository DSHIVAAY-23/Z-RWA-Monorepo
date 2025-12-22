use super::*;

#[account]
pub struct StableCoinStorage {
    /// USDC Address,
    pub usdc: Option<Pubkey>,

    /// USDT Address,
    pub usdt: Option<Pubkey>,

    /// DAI Address,
    pub dai: Option<Pubkey>,
}
