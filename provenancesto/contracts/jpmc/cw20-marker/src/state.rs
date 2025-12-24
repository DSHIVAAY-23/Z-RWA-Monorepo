use super::*;

pub type Bytes<'a> = &'a [u8];

pub const DENOM: Item<String> = Item::new("denom");
pub const FREEZE_LIST: Item<Vec<Addr>> = Item::new("freeze_list");
pub const FROZEN_TOKENS: Item<Uint128> = Item::new("frozen_tokens");
pub const PARTIAL_FREEZE: Map<Addr, Uint128> = Map::new("partial_freeze");
pub const ALLOWANCE: Map<Bytes, Uint128> = Map::new("partial_freeze");

// Admin
pub const SUB_ADMIN: Item<Vec<Addr>> = Item::new("sub_admin");
pub const TOKENIZATION_AGENT: Item<Addr> = Item::new("tokenization_agent");

pub const REQUESTS: Map<Bytes, Request> = Map::new("requests");
pub const MINT_ALLOWANCES: Map<Bytes, Uint128> = Map::new("mint_allowances");

pub const BURN_BALANCES: Map<Addr, Uint128> = Map::new("burn_balances");
pub const BURN_ALLOWANCES: Map<Bytes, Uint128> = Map::new("burn_allowances");

#[cw_serde]
pub struct DestConfig {
    pub chain: String,
    pub address: String,
}

pub const VSPN: &str = "vspn";
pub const DEST_CONFIG: Item<DestConfig> = Item::new("destination_config");

#[cw_serde]
pub struct Message {
    pub sender: String,
    pub message: String,
}

pub const STORED_MESSAGE: Item<Message> = Item::new("stored_message");
