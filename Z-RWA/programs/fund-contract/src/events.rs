use super::*;

#[event]
pub struct InitEvent {
    pub admin: Pubkey,
}

#[event]
pub struct CreateFundEvent {
    pub token: String,
    pub fund: String,
    pub asset_type: AssetType,
    pub issuer: String,
    pub target_aum: u64,
    pub nav_launch_price: u64,
    pub ccy: String,
}

impl CreateFundEvent {
    pub fn new(params: CreateParams) -> Self {
        Self {
            token: params.token,
            fund: params.fund,
            asset_type: params.asset_type,
            issuer: params.issuer,
            target_aum: params.target_aum,
            nav_launch_price: params.nav_launch_price,
            ccy: params.ccy,
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
pub struct ShareDividendsEvent {
    pub token: String,
    pub coin_type: CoinType,
    pub address: Pubkey,
    pub amount: u64,
    pub asset_type: AssetType,
}

impl ShareDividendsEvent {
    pub fn new(params: ShareDividendsParams) -> Self {
        Self {
            token: params.token,
            coin_type: params.coin_type,
            address: params.to_account,
            amount: params.dividend,
            asset_type: params.asset_type,
        }
    }
}

#[event]
pub struct DistributeAndBurnEvent {
    pub token: String,
    pub coin_type: CoinType,
    pub address: Pubkey,
    pub burn_amount: u64,
    pub distribution_amount: u64,
}

impl DistributeAndBurnEvent {
    pub fn new(params: DistributionParams) -> Self {
        Self {
            token: params.token,
            coin_type: params.coin_type,
            address: params.investor,
            burn_amount: params.burn_amount,
            distribution_amount: params.distribution_amount,
        }
    }
}

#[event]
pub struct UpdateStableCoinEvent {
    pub coin_type: CoinType,
    pub update_type: UpdateType,
}
