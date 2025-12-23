use super::*;

pub type Bytes<'a> = &'a [u8];

// Admin
pub const ADMIN: Item<Vec<Addr>> = Item::new("admin");

// Agent
pub const AGENT: Map<Bytes, Addr> = Map::new("agent_access");

// Global Config
pub const GLOBAL_CONFIG: Map<Bytes, GlobalConfig> = Map::new("global_config");

// Payments
pub const PAYMENTS: Map<Bytes, Payments> = Map::new("payments");
