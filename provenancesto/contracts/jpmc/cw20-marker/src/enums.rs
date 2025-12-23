use super::*;

#[cw_serde]
pub enum UpdateType<T> {
    Add(T),
    Remove(T),
}

#[cw_serde]
pub enum UpdateKind<T> {
    Set(T),
    Unset {},
}

#[cw_serde]
pub enum Role {
    SubAdmin { update_type: UpdateType<Vec<Addr>> },
    TokenizationAgent { update_type: UpdateType<Addr> },
}

#[cw_serde]
pub enum MessageType {
    Message = 1,
    MessageWithToken = 2,
    Token = 3,
}

impl MessageType {
    pub fn into_i64(&self) -> i64 {
        use MessageType::*;

        match self {
            Message => 1,
            MessageWithToken => 2,
            Token => 3,
        }
    }
}

#[cw_serde]
pub enum Status {
    Pending,
    Approved,
    Rejected,
}

impl Status {
    pub fn into_string(&self) -> String {
        use Status::*;

        match self {
            Pending => String::from("Pending"),
            Approved => String::from("Approved"),
            Rejected => String::from("Rejected"),
        }
    }
}

#[cw_serde]
pub enum RequestType {
    Mint,
    Burn,
}

impl RequestType {
    pub fn into_string(&self) -> String {
        use RequestType::*;

        match self {
            Mint => String::from("mint"),
            Burn => String::from("burn"),
        }
    }
}
