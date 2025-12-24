use super::*;

// Create  Params
#[cw_serde]
pub struct CreateParams {
    pub denom: String,
    pub issue_size: u128,
    pub face_value: u128,
    pub coupon_rate: u16,
    pub accrued_interest: u16,
    pub maturity_date: u64,
    pub issuer_name: String,
    pub coupon_frequency: String,
}

// Shared Stable Coin Params
#[cw_serde]
pub struct ShareParams {
    pub to: Addr,
    pub payment: Uint128,
}

#[cw_serde]
pub struct GlobalConfig {
    pub bond_name: String,
    pub issue_size: u128,
    pub face_value: u128,
    pub coupon_rate: u16,
    pub accrued_interest: u16,
    pub issue_date: u64,
    pub maturity_date: u64,
    pub issuer_name: String,
    pub coupon_frequency: String,
    pub treasury_manager: Addr,
    pub credit_rating: String,
}

impl GlobalConfig {
    pub fn new(params: CreateParams, issue_date: u64, treasury_manager: Addr) -> Self {
        GlobalConfig {
            issue_date,
            bond_name: params.denom,
            issue_size: params.issue_size,
            face_value: params.face_value,
            coupon_rate: params.coupon_rate * 100,
            accrued_interest: params.accrued_interest,
            maturity_date: params.maturity_date,
            issuer_name: params.issuer_name,
            coupon_frequency: params.coupon_frequency,
            treasury_manager,
            credit_rating: String::default(),
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
                key_1: self.key_1.to_string(),
                key_2: self.key_2.to_string(),
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
pub struct Payments {
    pub total_coupon_payment: Uint128,
    pub coupon_payment_paid: Uint128,
}

impl Payments {
    pub fn new(payment: Uint128) -> Self {
        Self {
            total_coupon_payment: payment,
            coupon_payment_paid: payment,
        }
    }

    pub fn add(&mut self, payment: Self) {
        self.total_coupon_payment += payment.total_coupon_payment;
        self.coupon_payment_paid += payment.coupon_payment_paid;
    }
}
