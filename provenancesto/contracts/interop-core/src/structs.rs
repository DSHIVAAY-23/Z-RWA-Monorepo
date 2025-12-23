use super::*;

#[cw_serde]
pub struct SendParams {
    pub portfolios: Vec<Portfolio>,
}

#[cw_serde]
pub struct Portfolio {
    pub dest_chain: String,
    pub dest_address: String,
    pub investor: String,
    pub token: String,
    pub amount: u128,
    pub order_id: u128,
    pub action: Action,
}

#[cw_serde]
pub struct Order {
    pub order_id: String,
    pub denom: String,
    pub user: Addr,
    pub amount: Uint128,
}
