use super::*;

/// Update Type
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum UpdateType {
    Add { addresses: Vec<Pubkey> },
    Remove { addresses: Vec<Pubkey> },
}

/// Roles
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Role {
    Validators { update_type: UpdateType },
}

/// Update Kind
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum UpdateKind {
    Add,
    Remove,
}

/// Struct indicating Voting Status
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Status {
    Init,
    Pending,
    Ready,
    Approved,
}

impl Default for Status {
    fn default() -> Self {
        Self::Init
    }
}
