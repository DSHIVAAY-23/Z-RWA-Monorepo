use super::*;

#[cw_serde]
pub enum UpdateType<T> {
    Add(T),
    Remove(T),
    Update(T),
}

#[cw_serde]
pub enum AssetType {
    Token,
    StableCoin,
    Fiat,
}

impl AssetType {
    pub fn as_str(&self) -> &str {
        match self {
            AssetType::Token => "Token",
            AssetType::StableCoin => "Stable Coin",
            AssetType::Fiat => "Fiat",
        }
    }
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
pub enum Dividend {
    Token(Uint128),
    StableCoin(Uint128),
    Fiat(Uint128),
}
