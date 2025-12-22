use super::*;

#[event]
pub struct InitEvent {
    pub admin: Pubkey,
    pub sub_admin: Pubkey,
}

#[event]
pub struct CreateTokenEvent {
    /// Unique id
    pub id: String,

    /// Token Name
    pub name: String,
}

#[event]
pub struct MintEvent {
    pub token: String,
    pub amount: u64,
}

#[event]
pub struct TransferEvent {
    pub token: String,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ForceTransferEvent {
    pub token: String,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct BurnEvent {
    pub token: String,
    pub amount: u64,
}

#[event]
pub struct FreezeEvent {
    pub address: Pubkey,
}

#[event]
pub struct PartialFreezeEvent {
    pub token: String,
    pub address: Pubkey,
    pub amount: u64,
    pub total: u64,
}

#[event]
pub struct PartialUnfreezeEvent {
    pub token: String,
    pub address: Pubkey,
    pub amount: u64,
    pub total: u64,
}

#[event]
pub struct UnfreezeEvent {
    pub address: Pubkey,
}

#[event]
pub struct UpdateAdminEvent {
    pub from: Pubkey,
    pub to: Pubkey,
}

#[event]
pub struct AddSubAdminsEvent {
    pub addresses: Vec<Pubkey>,
}

#[event]
pub struct RemoveSubAdminsEvent {
    pub addresses: Vec<Pubkey>,
}

#[event]
pub struct UpdateIssuerEvent {
    pub token: String,
    pub old: Pubkey,
    pub new: Pubkey,
}

#[event]
pub struct UpdateTokenizationAgentEvent {
    pub token: String,
    pub old: Pubkey,
    pub new: Pubkey,
}

#[event]
pub struct UpdateTransferAgentEvent {
    pub token: String,
    pub old: Pubkey,
    pub new: Pubkey,
}

#[event]
pub struct RequestEvent {
    pub request_type: RequestType,
    pub order_id: u128,
    pub user: Pubkey,
    pub amount: u64,
}

impl RequestEvent {
    pub fn new(params: RequestOrder) -> Self {
        Self {
            request_type: params.request_type,
            order_id: params.order_id,
            user: params.user,
            amount: params.amount,
        }
    }
}
