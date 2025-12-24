use super::*;

// Create a new marker contract for denom
#[cw_serde]
pub struct InitMsg {
    pub denom: String,
    pub tokenization_agent: Addr,
    pub config: DestConfig,
}
