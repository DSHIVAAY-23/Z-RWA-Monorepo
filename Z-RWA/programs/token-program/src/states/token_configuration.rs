use super::*;

#[account]
pub struct TokenConfiguration {
    /// token limit for each token holder (eg. token limit for each user = 1000,
    /// users can only hold up to 1000 tokens.
    pub token_limit: u64,

    /// Country code
    pub country_codes: Vec<u16>,

    /// Frozen Tokens
    pub frozen_tokens: u64,

    /// Issuer with mint, burn, freeze, unfreeze and force transfer rights
    pub issuer: Pubkey,

    /// Transfer Agent with freeze, unfreeze and force transfer rights
    pub transfer_agent: Pubkey,

    /// Issuer with mint and burn rights
    pub tokenization_agent: Pubkey,

    /// Holding Period
    pub holding_period: i64,
}

impl TokenConfiguration {
    pub fn save(&mut self, params: CreateTokenParams) {
        self.issuer = params.issuer;
        self.tokenization_agent = params.tokenization_agent;
        self.transfer_agent = params.transfer_agent;
        self.token_limit = params.token_limit;
        self.country_codes = params.country_codes;
        self.holding_period = params.holding_period;
    }

    pub fn add_country_codes(&mut self, codes: Vec<u16>) {
        self.country_codes.extend(codes);
        self.country_codes.sort();
        self.country_codes.dedup();
    }

    pub fn remove_country_codes(&mut self, codes: Vec<u16>) {
        self.country_codes.retain(|code| !codes.contains(code));
    }
}
