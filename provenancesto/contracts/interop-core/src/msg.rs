use super::*;

#[cw_serde]
pub struct InitMsg {
    pub multi_sig: Addr,
    pub deployed_chain: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    ManageRoles {
        roles: Vec<Role>,
    },
    UpdateSourceChain {
        chain: String,
    },
    SendInstruction {
        params: SendParams,
    },
    ExecuteInstruction {
        source_chain: String,
        source_address: String,
        payload: String,
    },
    MintTokens {
        order: Order,
    },
    BurnTokens {
        order: Order,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<Addr>)]
    GetAdmins {},

    #[returns(String)]
    GetSourceChain {},
}

/// Migrate the contract.
#[cw_serde]
pub struct MigrateMsg {}
