use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub compliance_module: Addr,
    pub token_service: Addr,
    pub verification_key: Binary,
}

pub const CONFIG: Item<Config> = Item::new("config");

// Map<AssetId, ProofHash>
// AssetId is likely generated during minting or provided (hash of doc?). 
// Using document_hash (in hex or base64) as key? Or a counter?
// User said: "Store a Map<AssetId, ProofHash>"
// We will use document_hash as AssetId for now or generate one. 
// Let's assume document_hash is the AssetId identifier in this context.
pub const AUDIT_TRAIL: Map<&[u8], Binary> = Map::new("audit_trail");
