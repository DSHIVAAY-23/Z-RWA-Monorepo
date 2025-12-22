use super::*;

#[event]
pub struct InitEvent {
    pub admin: Pubkey,
    pub threshold: u8,
}

#[event]
pub struct AddAdminsEvent {
    pub addresses: Vec<Pubkey>,
}

#[event]
pub struct RemoveAdminsEvent {
    pub addresses: Vec<Pubkey>,
}

#[event]
pub struct ManageValidatorsEvent {
    pub update_kind: UpdateKind,
    pub addresses: Vec<Pubkey>,
}

#[event]
pub struct UpdateExecuterEvent {
    pub address: Pubkey,
}

#[event]
pub struct UpdateThresholdEvent {
    pub old: u8,
    pub new: u8,
}

#[event]
pub struct CastVoteEvent {
    pub tx_hash: String,
    pub can_transact: bool,
}

#[event]
pub struct SendInstructionEvent {
    pub source_chain: String,
    pub source_address: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub sender: Pubkey,
    pub payload: String,
}

#[event]
pub struct ExecuteInstructionEvent {
    pub source_chain: String,
    pub source_address: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub sender: Pubkey,
    pub payload: String,
}

#[event]
pub struct ExecuteTransactionEvent {
    pub tx_hash: String,
}

#[event]
pub struct PayloadExtractedEvent {
    pub order_id: u128,
    pub token: String,
    pub investor: Pubkey,
}
