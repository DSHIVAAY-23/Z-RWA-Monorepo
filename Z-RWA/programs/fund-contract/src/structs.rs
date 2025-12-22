use super::*;

/// The struct containing instructions for creating fund
#[account]
pub struct CreateParams {
    /// Token Name
    pub token: String,

    /// Fund Name
    pub fund: String,

    /// Fund Manager Address
    pub fund_manager: Pubkey,

    /// Asset Type
    pub asset_type: AssetType,

    /// Issuer Name
    pub issuer: String,

    /// Target AUM
    pub target_aum: u64,

    /// NAV Launch Price
    pub nav_launch_price: u64,

    /// Currency
    pub ccy: String,
}

/// The struct containing instructions for share dividends
#[account]
pub struct ShareDividendsParams {
    /// Token Name
    pub token: String,

    /// Coin Type
    pub coin_type: CoinType,

    /// To Account
    pub to_account: Pubkey,

    /// Dividend
    pub dividend: u64,

    /// Asset Type
    pub asset_type: AssetType,

    /// Decimals
    pub decimals: u8,
}

/// The struct containing instructions for waterfall distribution
#[account]
pub struct DistributionParams {
    /// Token Name
    pub token: String,

    /// Coin Type
    pub coin_type: CoinType,

    /// Investor
    pub investor: Pubkey,

    /// Distribution Amount
    pub distribution_amount: u64,

    /// Burn Amount
    pub burn_amount: u64,

    /// Decimals
    pub decimals: u8,
}

/// The struct containing instructions for stable coin
#[account]
pub struct StableCoinParams {
    /// Coin Type
    pub coin_type: CoinType,

    /// Type of updation
    pub update_type: UpdateType,
}
