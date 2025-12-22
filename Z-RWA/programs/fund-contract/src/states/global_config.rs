use super::*;

#[account]
pub struct GlobalConfig {
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

    /// Issue Timestamp
    pub issue_timestamp: i64,
}

impl GlobalConfig {
    pub fn save(&mut self, params: CreateParams, timestamp: i64) {
        self.fund = params.fund;
        self.fund_manager = params.fund_manager;
        self.asset_type = params.asset_type;
        self.issuer = params.issuer;
        self.target_aum = params.target_aum;
        self.nav_launch_price = params.nav_launch_price;
        self.ccy = params.ccy;
        self.issue_timestamp = timestamp;
    }
}
