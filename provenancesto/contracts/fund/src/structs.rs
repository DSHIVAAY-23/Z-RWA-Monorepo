use super::*;

// Create  Params
#[cw_serde]
pub struct CreateParams {
    pub denom: String,
    pub fund_name: String,
    pub asset_type: AssetType,
    pub issuer_name: String,
    pub target_aum: Uint128,
    pub nav_launch_price: Uint128,
    pub ccy: String,
}

// Managed Users
#[cw_serde]
pub struct ManagedUser {
    pub user: Addr,
    pub fee: Uint128,
}

// Shared Dividend Struct
#[cw_serde]
pub struct SharedDividend {
    pub to: Addr,
    pub dividend: Uint128,
    pub asset_type: AssetType,
}

// Distribution
#[cw_serde]
pub struct Distribution {
    pub investor: Addr,
    pub amount: Uint128,
    pub token: Uint128,
}

#[cw_serde]
pub struct GlobalConfig {
    pub fund_name: String,
    pub asset_type: AssetType,
    pub issuer_name: String,
    pub target_aum: Uint128,
    pub nav_launch_price: Uint128,
    pub issue_timestamp: u64,
    pub nav_latest_price: Uint128,
    pub ccy: String,
}

impl GlobalConfig {
    pub fn new(params: CreateParams, timestamp: u64) -> Self {
        GlobalConfig {
            fund_name: params.fund_name,
            asset_type: params.asset_type,
            issuer_name: params.issuer_name.to_string(),
            target_aum: params.target_aum,
            nav_launch_price: params.nav_launch_price,
            issue_timestamp: timestamp,
            nav_latest_price: Uint128::zero(),
            ccy: params.ccy,
        }
    }
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
