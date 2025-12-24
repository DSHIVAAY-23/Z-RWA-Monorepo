use super::*;

pub type Bytes<'a> = &'a [u8];

pub const FREEZE_LIST: Item<Vec<Addr>> = Item::new("freeze_list");
pub const FROZEN_TOKENS: Item<Uint128> = Item::new("frozen_tokens");
pub const PARTIAL_FREEZE: Map<Bytes, Uint128> = Map::new("partial_freeze");

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
