use super::*;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SendMessageEvm {
        destination_chain: String,
        destination_address: String,
        message: String,
        msg_type: MessageType,
    },
    SendMessageCosmos {
        destination_chain: String,
        destination_address: String,
        message: String,
        msg_type: MessageType,
    },
    ReceiveMessageCosmos {
        sender: String,
        message: String,
    },
    ReceiveMessageEvm {
        source_chain: String,
        source_address: String,
        payload: Binary,
    },
    SendFund {
        to: String,
        amount: Uint128,
    },
    Call {
        to: String,
        amount: Uint128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetStoredMessageResp)]
    GetStoredMessage {},

    #[returns(IBCResponse)]
    #[serde(rename = "get_ibc_response")]
    GetIBCResponse {},
}

#[cw_serde]
pub struct GetStoredMessageResp {
    pub sender: String,
    pub message: String,
}

#[cw_serde]
pub struct Fee {
    pub amount: String,
    pub recipient: String,
}

#[cw_serde]
pub struct GmpMessage {
    pub destination_chain: String,
    pub destination_address: String,
    pub payload: Vec<u8>,
    #[serde(rename = "type")]
    pub type_: i64,
    pub fee: Option<Fee>,
}

#[cw_serde]
pub enum IBCLifecycleComplete {
    #[serde(rename = "ibc_ack")]
    IBCAck {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
        /// String encoded version of the ack as seen by OnAcknowledgementPacket(..)
        ack: String,
        /// Whether an ack is a success of failure according to the transfer spec
        success: bool,
    },
    #[serde(rename = "ibc_timeout")]
    IBCTimeout {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
    },
}

// Import the queries from the package to avoid cyclic dependencies
pub use registry::msg::{
    GetAddressFromAliasResponse, GetChannelFromChainPairResponse,
    GetDestinationChainFromSourceChainViaChannelResponse,
    QueryGetBech32PrefixFromChainNameResponse,
};

/// Message type for `sudo` entry_point
#[cw_serde]
pub enum SudoMsg {
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
}

#[cw_serde]
#[derive(Default)]
pub struct IBCResponse {
    pub registry: String,
    pub channel: String,
    pub sequence: u64,
    pub ack: String,
    pub success: bool,
}
