use super::*;

#[account]
pub struct PartialFreeze {
    /// Frozen Amount
    pub amount: u64,
}
