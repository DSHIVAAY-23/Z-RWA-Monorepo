use super::*;

// Factory contract address
pub const FACTORY_CONTRACT_ADDRESS: &str =
    "tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla";

pub const VSPN: &str = "vspn";

#[cw_serde]
pub struct Message {
    pub sender: String,
    pub message: String,
}

pub const STORED_MESSAGE: Item<Vec<Message>> = Item::new("stored_message");

#[cw_serde]
pub struct DestConfig {
    pub chain: String,
    pub address: String,
}

pub const DEST_CONFIG: Item<DestConfig> = Item::new("destination_config");

// The name of the chain on which this contract is instantiated
pub const CONTRACT_CHAIN: &str = "provenance";

pub const IBC_RESPONSE: Item<IBCResponse> = Item::new("ibc_response");

pub const OPERATORS: Item<Vec<String>> = Item::new("operators");
