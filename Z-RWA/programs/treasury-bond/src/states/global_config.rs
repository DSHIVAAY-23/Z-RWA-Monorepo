use super::*;

#[account]
pub struct GlobalConfig {
    /// Token Name
    pub bond_name: String,

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

    /// Issue Date
    pub issue_date: i64,

    /// Treasury Manager
    pub treasury_manager: Pubkey,

    /// Credit Rating
    pub credit_rating: String,
}

impl GlobalConfig {
    pub fn save(&mut self, params: CreateParams, issue_date: i64, treasury_manager: Pubkey) {
        self.issue_date = issue_date;
        self.bond_name = params.token;
        self.issue_size = params.issue_size;
        self.face_value = params.face_value;
        self.coupon_rate = params.coupon_rate * 100;
        self.accrued_interest = params.accrued_interest;
        self.maturity_date = params.maturity_date;
        self.issuer_name = params.issuer_name;
        self.coupon_frequency = params.coupon_frequency;
        self.treasury_manager = treasury_manager;
        self.credit_rating = String::default();
    }
}
