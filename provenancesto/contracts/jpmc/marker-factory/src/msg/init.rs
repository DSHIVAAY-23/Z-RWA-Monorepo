use super::*;

#[cw_serde]
pub struct InitMsg {
    /// token contract code id
    pub code_id: u64,
}
