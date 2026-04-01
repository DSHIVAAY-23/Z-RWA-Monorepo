use anchor_lang::prelude::*;

#[account]
pub struct ComplianceRecord {
    pub owner: Pubkey,
    pub is_verified: bool,
    pub document_hash: [u8; 32],
}

impl ComplianceRecord {
    pub const LEN: usize = 8 // discriminator
        + 32 // owner
        + 1 // is_verified
        + 32; // document_hash
}
