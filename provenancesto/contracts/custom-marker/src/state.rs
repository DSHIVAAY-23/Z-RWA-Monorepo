use super::*;

pub type Bytes<'a> = &'a [u8];

pub const FREEZE_LIST: Map<Bytes, Vec<Addr>> = Map::new("freeze_list");
pub const MINTED_TOKENS: Map<Bytes, Uint128> = Map::new("minted_tokens");
pub const FROZEN_TOKENS: Map<Bytes, Uint128> = Map::new("frozen_tokens");
pub const WHITELIST: Map<Bytes, u8> = Map::new("whitelist");
pub const DENOM_CONFIG: Map<Bytes, DenomConfig> = Map::new("denom_config");
pub const PARTIAL_FREEZE: Map<Bytes, Uint128> = Map::new("partial_freeze");

pub const HOLDING_PERIOD: Map<Bytes, Uint64> = Map::new("holding_period");

// Admin
pub const ADMIN: Item<Addr> = Item::new("admin");

// Sub Admin
pub const SUB_ADMIN: Item<Vec<Addr>> = Item::new("sub_admin");

// Access to Agents
pub const AGENTS: Map<Bytes, Vec<Addr>> = Map::new("agents_access");

// Access To Issuer, Transfer and Tokenizaion Agents
pub const ISSUER: Map<Bytes, Vec<AccessControls>> = Map::new("issuer");
pub const TRANSFER_AGENT: Map<Bytes, Vec<AccessControls>> = Map::new("transfer_agent");
pub const TOKENIZATION_AGENT: Map<Bytes, Vec<AccessControls>> = Map::new("tokenization_agent");
