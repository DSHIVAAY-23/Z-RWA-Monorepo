use super::*;

#[account]
pub struct TokenConfiguration {
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
        self.holding_period = params.holding_period;
    }
}
