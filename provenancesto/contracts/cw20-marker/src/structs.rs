use super::*;
use ::prost::Message;

// Create Marker Params
#[cw_serde]
pub struct CreateMarkerParams {
    pub id: String,
    pub denom: String,
    pub issuer: Addr,
    pub transfer_agent: Addr,
    pub tokenization_agent: Addr,
    pub holding_period: Uint64,
}

// Force Transfer Params
#[cw_serde]
pub struct ForceTransferParams {
    pub amount: Uint128,
    pub from: Addr,
    pub to: Addr,
}

// Partial Freeze Params
#[cw_serde]
pub struct PartialFreezeParams {
    pub address: Addr,
    pub update_type: UpdateType<Uint128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Key<T, U> {
    pub key_1: T,
    pub key_2: U,
}

impl<T, U> Key<T, U>
where
    T: Serialize + DeserializeOwned + Clone + Display + Sized,
    U: Serialize + DeserializeOwned + Clone + Display + Sized,
{
    pub fn new(key_1: T, key_2: U) -> Self {
        Self { key_1, key_2 }
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, ContractError> {
        match serialize(&self) {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(ContractError::SerializationFailed {
                denom: self.key_1.to_string(),
                address: self.key_2.to_string(),
            }),
        }
    }

    pub fn as_bytes_std(&self) -> Result<Vec<u8>, StdError> {
        match serialize(&self) {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(StdError::serialize_err(
                "Struct",
                format!("key_1: `{}`, key_2: `{}`!", self.key_1, self.key_2),
            )),
        }
    }

    pub fn from_bytes(&self, bytes: Bytes) -> Result<Self, ContractError> {
        match deserialize::<Self>(bytes) {
            Ok(key) => Ok(key),
            Err(_) => Err(ContractError::DeserializationFailed {}),
        }
    }
}

#[cw_serde]
pub struct MintBurnParams {
    pub denom: String,
    pub mint_burn_data: Vec<MintBurnData>,
}

#[cw_serde]
pub struct MintBurnData {
    pub address: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct IBCResponse {
    pub registry: String,
    pub channel: String,
    pub sequence: u64,
    pub ack: String,
    pub success: bool,
}

#[cw_serde]
pub struct AckMessage {
    pub result: String,
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

#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct IbcCounterpartyHeight {
    #[prost(uint64, optional, tag = "1")]
    revision_number: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    revision_height: Option<u64>,
}

// We need to define the transfer here as a stargate message because this is
// not yet supported by cosmwasm-std. See https://github.com/CosmWasm/cosmwasm/issues/1477
#[derive(Clone, PartialEq, Eq, Message, Serialize, Deserialize, JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.applications.transfer.v1.MsgTransfer")]
pub struct MsgTransfer {
    #[prost(string, tag = "1")]
    pub source_port: String,
    #[prost(string, tag = "2")]
    pub source_channel: String,
    #[prost(message, optional, tag = "3")]
    pub token: ::core::option::Option<osmosis_std::types::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub sender: String,
    #[prost(string, tag = "5")]
    pub receiver: String,
    #[prost(message, optional, tag = "6")]
    pub timeout_height: Option<IbcCounterpartyHeight>,
    #[prost(uint64, optional, tag = "7")]
    pub timeout_timestamp: ::core::option::Option<u64>,
    #[prost(string, tag = "8")]
    pub memo: String,
}

#[cw_serde]
pub struct Request {
    pub request_type: RequestType,
    pub requester: Addr,
    pub responder: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct DestConfig {
    pub chain: String,
    pub address: String,
}
