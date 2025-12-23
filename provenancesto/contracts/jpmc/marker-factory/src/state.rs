use super::*;

pub const LABEL: &str = "cw20-marker";

pub const INSTANTIATE_REPLY_ID: u64 = 1;
pub const DENOM: Item<String> = Item::new("denom");

// Admin
pub const ADMIN: Item<Addr> = Item::new("admin");

// Sub Admin
pub const SUB_ADMIN: Item<Vec<Addr>> = Item::new("sub_admin");

pub const CONTRACT_TO_DENOM: Map<Addr, String> = Map::new("denom_to_contract_mapping");
pub const DENOM_TO_CONTRACT: Map<String, Addr> = Map::new("contract_to_denom_mapping");
pub const CONTRACTS: Item<Vec<String>> = Item::new("contracts");
pub const CODE_ID: Item<u64> = Item::new("code_id");
