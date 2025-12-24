use super::*;

pub type Bytes<'a> = &'a [u8];

pub const CONTRACT_ADDRESS: &str = "tp1wkwy0xh89ksdgj9hr347dyd2dw7zesmtrue6kfzyml4vdtz6e5wsvaczas";
pub const FREEZE_LIST: Map<Bytes, Vec<Addr>> = Map::new("freeze_list");
pub const MINTED_TOKENS: Map<Bytes, Uint128> = Map::new("minted_tokens");
pub const FROZEN_TOKENS: Map<Bytes, Uint128> = Map::new("frozen_tokens");
pub const PARTIAL_FREEZE: Map<Bytes, Uint128> = Map::new("partial_freeze");

// Admin
pub const ADMIN: Item<Addr> = Item::new("admin");

// Sub Admin
pub const SUB_ADMIN: Item<Vec<Addr>> = Item::new("sub_admin");

// Access To Issuer, Transfer and Tokenizaion Agents
pub const ISSUER: Map<Bytes, Vec<AccessControls>> = Map::new("issuer");
pub const TRANSFER_AGENT: Map<Bytes, Vec<AccessControls>> = Map::new("transfer_agent");
pub const TOKENIZATION_AGENT: Map<Bytes, Vec<AccessControls>> = Map::new("tokenization_agent");

pub const HOLDING_PERIOD: Map<Bytes, Uint64> = Map::new("holding_period");

pub const IBC_RESPONSE: Item<Vec<IBCResponse>> = Item::new("ibc_response");

pub const OPERATORS: Item<Vec<String>> = Item::new("operators");

#[cw_serde]
pub struct Message {
    pub sender: String,
    pub message: String,
}
pub const STORED_MESSAGE: Item<Message> = Item::new("stored_message");

pub const REQUESTS: Map<String, Request> = Map::new("requests");
pub const MINT_BALANCES: Map<Addr, Uint128> = Map::new("mint_balances");

// pub const REQUESTS: Map<Bytes, Request> = Map::new("burn_requests");
pub const BURN_BALANCES: Map<Addr, Uint128> = Map::new("burn_balances");
pub const BURN_REQUEST_ALLOWANCES: Map<Bytes, Uint128> = Map::new("burn_request_allowances");

pub const DEST_CONFIG: Item<DestConfig> = Item::new("destination_config");

pub const IS_IBC_RESPONSE_REQUIRED: Item<bool> = Item::new("is_ibc_response_required");
