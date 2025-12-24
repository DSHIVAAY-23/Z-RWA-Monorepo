use super::*;

// Create Marker Params
#[cw_serde]
pub struct CreateMarkerParams {
    pub id: String,
    pub supply: Uint128,
    pub denom: String,
    pub denom_config: DenomConfig,
    pub issuer: Addr,
    pub transfer_agent: Addr,
    pub tokenization_agent: Addr,
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

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, Default)]
pub struct DenomConfig {
    /// token limit for each token holder (eg. token limit for each user = 1000,
    /// users can only hold up to 1000 tokens.
    pub token_limit: Uint128,
    /// Country code
    pub country_codes: Vec<u8>,
}

impl DenomConfig {
    pub fn new(other: Self) -> Self {
        Self {
            token_limit: other.token_limit,
            country_codes: other.country_codes,
        }
    }

    pub fn add_token_limit(&mut self, token_limit: Uint128) {
        self.token_limit += token_limit;
    }

    pub fn sub_token_limit(&mut self, token_limit: Uint128) {
        self.token_limit -= token_limit;
    }

    pub fn add_country_codes(&mut self, country_codes: Vec<u8>) -> Result<(), ContractError> {
        for code in country_codes {
            add_country_codes(&mut self.country_codes, code)?;
        }

        Ok(())
    }

    pub fn remove_country_codes(&mut self, country_codes: Vec<u8>) -> Result<(), ContractError> {
        for code in country_codes {
            remove_country_codes(&mut self.country_codes, code)?;
        }

        Ok(())
    }
}

#[cw_serde]
pub struct MintBurnData {
    pub address: String,
    pub amount: Uint128,
}
