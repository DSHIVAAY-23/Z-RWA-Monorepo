use super::*;

/// Asset Type, use to choose between different assets
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum AssetType {
    Token,
    StableCoin,
}

impl AssetType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Token => "Token",
            Self::StableCoin => "Stable Coin",
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
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

/// Asset Type, use to choose between different assets
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum UpdateType {
    Add { address: Pubkey },
    Remove,
}
