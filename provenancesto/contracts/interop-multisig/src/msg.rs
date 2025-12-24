use super::*;

#[cw_serde]
pub struct InitMsg {
    pub threshold: u8,
}

#[cw_serde]
pub enum ExecuteMsg {
    ManageRoles {
        roles: Vec<Role>,
    },
    ExecuteTransaction {
        source_chain: String,
        source_address: String,
        tx_hash: String,
        payload: String,
    },
    CastVote {
        tx_hash: String,
        can_transact: bool,
    },
    UpdateThreshold {
        threshold: u8,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<Addr>)]
    GetAdmins {},

    #[returns(Votes)]
    GetVotes { tx_hash: String },

    #[returns(Vec<Addr>)]
    GetValidators {},
}

/// Migrate the contract.
#[cw_serde]
pub struct MigrateMsg {}
