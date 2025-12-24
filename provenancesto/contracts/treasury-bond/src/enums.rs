use super::*;

#[cw_serde]
pub enum UpdateType<T> {
    Add(T),
    Remove(T),
    Update(T),
}

#[cw_serde]
pub enum CoinType {
    Dai,
    Usdt,
    Usdc,
}

impl CoinType {
    pub fn get_denom(&self) -> String {
        match self {
            CoinType::Dai => String::from("DAI-Test"),
            CoinType::Usdt => String::from("USDT-Test"),
            CoinType::Usdc => String::from("USDC-Test"),
        }
    }
}

#[cw_serde]
pub enum Role {
    Admin { update_type: UpdateType<Vec<Addr>> },
    Agent { denom: String, address: Addr },
}
