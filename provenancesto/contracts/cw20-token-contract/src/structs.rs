use super::*;

// Partial Freeze Params
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PartialFreezeParams {
    pub address: Addr,
    pub update_type: UpdateType<Uint128>,
}
