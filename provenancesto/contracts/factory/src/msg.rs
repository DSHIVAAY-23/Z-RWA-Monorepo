use super::*;

#[cw_serde]
pub struct InitMsg {
    /// token contract code id
    pub code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    MintTo {
        denom: String,
        params: Vec<MintBurnData>,
    },
    Transfer {
        amount: Uint128,
        denom: String,
        to: String,
    },
    Freeze {
        denom: String,
        update_type: UpdateType<Vec<Addr>>,
    },
    PartialFreeze {
        denom: String,
        params: Vec<PartialFreezeParams>,
    },
    Send {
        denom: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    BurnFrom {
        denom: String,
        burn_from_params: Vec<MintBurnData>,
    },
    ManageRoles {
        denom: String,
        roles: Vec<Role>,
    },
    DeployToken {
        params: token_contract::msg::Instantiate,
    },
    UpdateTokenContract {
        code_id: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(provwasm_std::Marker)]
    GetByAddress { address: String },

    #[returns(provwasm_std::Marker)]
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

    #[returns(Option<Vec<(Bytes, MintBurnProposalInfo)>>)]
    GetAllProposals {},

    #[returns(MintBurnProposalInfo)]
    GetMintProposalInfo { proposal_id: u128 },

    #[returns(MintBurnProposalInfo)]
    GetBurnProposalInfo { proposal_id: u128 },

    #[returns(Uint128)]
    GetFrozenTokens { denom: String },

    #[returns(Uint128)]
    GetCiculatingSupply { denom: String },

    #[returns(Addr)]
    GetContractAddressByDenom { denom: String },
}

/// Migrate the contract.
#[cw_serde]
pub struct MigrateMsg {}
