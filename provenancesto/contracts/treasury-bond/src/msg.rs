use super::*;

#[cw_serde]
pub struct InitMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Create {
        params: CreateParams,
    },
    ManageRoles {
        role: Role,
    },
    ShareStableCoin {
        denom: String,
        coin_type: CoinType,
        share_params: Vec<ShareParams>,
    },
    UpdateCreditRating {
        denom: String,
        rating: String,
    },
    SendStableCoins {
        denom: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<Addr>)]
    GetAdmins {},

    #[returns(Addr)]
    GetAgentByDenom { denom: String },

    #[returns(GlobalConfig)]
    GetConfig { denom: String },

    #[returns(Payments)]
    GetPayments { denom: String, user: Addr },
}

/// Migrate the contract.
#[cw_serde]
pub struct MigrateMsg {}
