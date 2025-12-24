use super::*;

#[cw_serde]
pub struct Message {
    pub sender: String,
    pub message: String,
}

pub const STORED_MESSAGE: Item<Message> = Item::new("stored_message");

// The name of the chain on which this contract is instantiated
pub const CONTRACT_CHAIN: &str = "provenance";

pub const IBC_RESPONSE: Item<IBCResponse> = Item::new("ibc_response");
