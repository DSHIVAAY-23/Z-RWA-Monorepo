use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Addr};

#[cw_serde]
pub struct InstantiateMsg {
    pub compliance_module: Addr,
    pub token_service: Addr,
    pub verification_key: Binary, // Verification Key for the ZK Circuit
}

#[cw_serde]
pub enum ExecuteMsg {
    MintRwaAsset {
        document_hash: Binary,
        proof: Binary,
        // Public inputs might be derived or passed partially.
        // User said: "Verify that the proof includes document_hash, msg.sender, and env.contract.address as public inputs"
        // So we probably don't need to pass them explicitly if we derive them, 
        // OR we pass them and enforce equality. 
        // Usually we DERIVE them to ensure integrity.
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Binary)]
    GetProof { asset_id: String },
}
