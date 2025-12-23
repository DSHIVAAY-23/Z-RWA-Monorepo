use super::*;

#[cw_serde]
pub struct InitMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Create {
        params: CreateMarkerParams,
    },
    GrantAccess {
        denom: String,
        address: Addr,
    },
    Mint {
        amount: Uint128,
        denom: String,
    },
    Burn {
        amount: Uint128,
        denom: String,
    },
    Cancel {
        denom: String,
    },
    Destroy {
        denom: String,
    },
    ForceTransfer {
        denom: String,
        params: Vec<ForceTransferParams>,
    },
    Withdraw {
        amount: Uint128,
        denom: String,
    },
    Freeze {
        denom: String,
        update_type: UpdateType<Vec<Addr>>,
    },
    Whitelist {
        lists: Vec<WhiteListParams>,
    },
    PartialFreeze {
        denom: String,
        params: Vec<PartialFreezeParams>,
    },
    UpdateTokenLimit {
        denom: String,
        limit: Uint128,
    },
    UpdateCountryCode {
        update_type: UpdateType<u8>,
        denom: String,
    },
    Send {
        amount: Uint128,
        denom: String,
        to: Addr,
    },
    MintTo {
        mint_to_params: Vec<MintBurnParams>,
    },
    DeliveryVsPayment {
        denom: String,
        mint_data: Vec<MintBurnData>,
    },
    BurnFrom {
        burn_from_params: Vec<MintBurnParams>,
    },
    ManageRoles {
        denom: String,
        roles: Vec<Role>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Marker)]
    GetByAddress { address: String },

    #[returns(Marker)]
    GetByDenom { denom: String },

    #[returns(Vec<u8>)]
    GetAuthorizedCountries { denom: String },

    #[returns(Option<Vec<Addr>>)]
    GetFreezedAccounts { denom: String },

    #[returns(Option<Uint128>)]
    GetFrozenBalance { denom: String, address: Addr },

    #[returns(DenomConfig)]
    GetDenomConfig { denom: String },

    #[returns(u8)]
    GetCountryCodeByAddress { denom: String, address: Addr },

    #[returns(Vec<Addr>)]
    GetSubAdmins {},

    #[returns(Addr)]
    GetAdmin {},

    #[returns(cosmwasm_std::Coin)]
    GetBalance { denom: String, address: Addr },

    #[returns(Uint128)]
    GetFrozenTokens { denom: String },

    #[returns(Uint128)]
    GetCiculatingSupply { denom: String },

    #[returns(Uint64)]
    GetHoldPeriod { denom: String },
}

/// Migrate the contract.
#[cw_serde]
pub struct MigrateMsg {}

// Cancel Params
#[cw_serde]
pub struct CancelParams {
    pub request_type: RequestType,
    pub proposal_id: u128,
}
