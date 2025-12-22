use super::*;

#[account]
pub struct Request {
    pub request_type: RequestType,
    pub requester: Pubkey,
    pub responder: Pubkey,
    pub amount: u64,
}
