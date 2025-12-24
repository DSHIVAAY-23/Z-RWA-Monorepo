use super::*;

#[cw_serde]
pub struct InitMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Create {
        params: CreateParams,
    },
    ManageAdmins {
        update_type: UpdateType<Vec<Addr>>,
    },
    ManageAgent {
        denom: String,
        update_type: UpdateType<Addr>,
    },
    ManagementFees {
        denom: String,
        managed_users: UpdateType<Vec<ManagedUser>>,
    },
    ShareDividend {
        denom: String,
        coin_type: CoinType,
        shared_dividends: Vec<SharedDividend>,
    },
    DistributeAndBurn {
        denom: String,
        coin_type: CoinType,
        distributions: Vec<Distribution>,
    },
    FetchPrice {
        denom: String,
    },
    UpdateCurrency {
        denom: String,
        ccy: String,
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

    #[returns(Uint128)]
    GetManagementFees { denom: String, user: Addr },

    #[returns(Uint128)]
    GetNav { denom: String },

    #[returns(Uint128)]
    GetAum { denom: String },

    #[returns(Addr)]
    GetAgentByDenom { denom: String },
}

/// Migrate the contract.
#[cw_serde]
pub struct MigrateMsg {}
