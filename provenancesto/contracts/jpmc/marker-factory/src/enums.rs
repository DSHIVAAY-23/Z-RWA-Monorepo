use super::*;

#[cw_serde]
pub enum Role {
    TokenSubAdmin {
        denom: String,
        update_type: UpdateType<Vec<Addr>>,
    },
    SubAdmin {
        update_type: UpdateType<Vec<Addr>>,
    },
    Admin {
        address: Addr,
    },
}
