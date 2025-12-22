use super::*;

#[account]
pub struct WhitelistedUser {
    /// Country Code
    pub country_code: u16,
}
