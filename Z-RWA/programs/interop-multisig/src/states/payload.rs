use super::*;

#[account]
pub struct Payload {
    pub order_id: u128,

    pub investor: Pubkey,

    pub token: String,
}
