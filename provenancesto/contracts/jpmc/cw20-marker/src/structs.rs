use super::*;

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

    pub fn as_bytes(&self) -> Result<Vec<u8>, StdError> {
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
pub struct Data {
    pub address: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct Request {
    pub request_type: RequestType,
    pub status: Status,
    pub requester: Addr,
    pub responder: Option<Addr>,
    pub amount: Uint128,
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
