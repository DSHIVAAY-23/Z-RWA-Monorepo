use super::*;

/// The struct containing instructions for creating fund
#[account]
pub struct CreateParams {
    /// Token Name
    pub token: String,

    /// Issue Size
    pub issue_size: u128,

    /// Face Value
    pub face_value: u128,

    /// Coupon Rate
    pub coupon_rate: u16,

    /// Accrued Interest
    pub accrued_interest: u16,

    /// Maturity Date
    pub maturity_date: i64,

    /// Issuer Name
    pub issuer_name: String,

    /// Coupon Frequency
    pub coupon_frequency: String,
}

/// The struct containing instructions for share dividends
#[account]
pub struct ShareStableCoinParams {
    /// Token Name
    pub token: String,

    /// Coin Type
    pub coin_type: CoinType,

    /// To Account
    pub to_account: Pubkey,

    /// Payment
    pub payment: u64,

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
