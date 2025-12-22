use super::*;

#[event]
pub struct InitEvent {
    pub admin: Pubkey,
}

#[event]
pub struct CreateFundEvent {
    pub token: String,
}

impl CreateFundEvent {
    pub fn new(params: CreateParams) -> Self {
        Self {
            token: params.token,
        }
    }
}

#[event]
pub struct AddAdminsEvent {
    pub addresses: Vec<Pubkey>,
}

#[event]
pub struct RemoveAdminsEvent {
    pub addresses: Vec<Pubkey>,
}

#[event]
pub struct UpdateAgentEvent {
    pub token: String,
    pub address: Pubkey,
}

#[event]
pub struct ShareStableCoinEvent {
    pub token: String,
    pub coin_type: CoinType,
    pub address: Pubkey,
    pub amount: u64,
}

impl ShareStableCoinEvent {
    pub fn new(params: ShareStableCoinParams) -> Self {
        Self {
            token: params.token,
            coin_type: params.coin_type,
            address: params.to_account,
            amount: params.payment,
        }
    }
}

#[event]
pub struct UpdateStableCoinEvent {
    pub coin_type: CoinType,
    pub update_type: UpdateType,
}

#[event]
pub struct UpdateCreditRatingEvent {
    pub token: String,
    pub rating: String,
}
