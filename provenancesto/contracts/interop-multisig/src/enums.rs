use super::*;

#[cw_serde]
pub enum UpdateType<T> {
    Add(T),
    Remove(T),
}

#[cw_serde]
pub enum Role {
    Admins { update_type: UpdateType<Vec<Addr>> },
    Validators { update_type: UpdateType<Vec<Addr>> },
}

#[cw_serde]
pub enum DestUpdateType {
    Chain(String),
    Address(String),
}

#[cw_serde]
pub enum Status {
    Pending,
    Ready,
    Approved,
}
