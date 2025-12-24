use super::*;

#[cw_serde]
pub enum MessageType {
    Message,
    MessageWithToken,
    Token,
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
pub enum UpdateType<T> {
    Add(T),
    Remove(T),
}
