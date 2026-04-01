use crate::{errors::*, states::*};
use anchor_lang::prelude::*;
use sp1_solana::{verify_proof, GROTH16_VK_3_0_0_BYTES};

// A dummy Hardcoded VKey for the SP1 ZK program. In production, this would be passed in or configured.
pub const ZK_RAG_VKEY: &[u32; 8] = &[0,0,0,0,0,0,0,0]; // Replace with real VKey

#[derive(Accounts)]
pub struct VerifyAndRecord<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        space = ComplianceRecord::LEN,
        seeds = [b"compliance", payer.key().as_ref()],
        bump
    )]
    pub compliance_record: Account<'info, ComplianceRecord>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn verify_and_record(
    ctx: Context<VerifyAndRecord>,
    proof: Vec<u8>,
    public_values: Vec<u8>,
) -> Result<()> {
    // 1. Verify SP1 Groth16 Proof using Succinct's crate
    // Heavily compute intensive - recommended to use ed25519 precompiles or AltBN128 if available
    verify_proof(
        &proof,
        &public_values,
        ZK_RAG_VKEY,
        GROTH16_VK_3_0_0_BYTES,
    ).map_err(|_| CustomError::InvalidSP1Proof)?;

    // 2. Extract the document hash
    // Assuming the first 32 bytes of public values map to the document hash
    let mut document_hash = [0u8; 32];
    if public_values.len() >= 32 {
        document_hash.copy_from_slice(&public_values[0..32]);
    } else {
        return err!(CustomError::InvalidPublicValues);
    }

    // 3. Mark the PDA as verified
    let record = &mut ctx.accounts.compliance_record;
    record.owner = ctx.accounts.payer.key();
    record.is_verified = true;
    record.document_hash = document_hash;

    Ok(())
}
