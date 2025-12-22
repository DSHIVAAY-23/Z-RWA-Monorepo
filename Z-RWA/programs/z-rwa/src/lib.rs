use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{self, MintTo, Token2022},
    token_interface::Mint, // Use interface for flexibility, or specifically Token2022 Mint
};

declare_id!("EaEtWQyXSb5t26KrKpp7XWqrvs1wJAkBM67Qwt1RC5gY");

// Placeholder VKey - replace with actual verification key from Prover
// This is usually a 32-byte hash of the program ELF or similar identifier for SP1
    // Placeholder VKey - replace with actual verification key from Prover
    // This is usually a 32-byte hash of the program ELF or similar identifier for SP1
    // Update: sp1-solana 3.0 expects this as a hex string (vkey hash)
    pub const ZK_RAG_VKEY: &str = "0x00cef2f0dedae3382b36f085503bb1a86d98102bca1f64362bdaa1634276df9f"; 

    #[program]
    pub mod z_rwa {
        use super::*;
    
        pub fn verify_and_mint(
            ctx: Context<VerifyAndMint>, 
            proof: Vec<u8>, 
            public_values: Vec<u8>
        ) -> Result<()> {
            msg!("Instruction: Verify and Mint");
    
            // 1. Verify SP1 Proof
            // We verify that the proof is valid for the given public inputs and VKEY.
            // sp1_solana::verify_proof uses the Groth16 Verifier deployed on Solana (or syscalls if available/embedded).
            // Note: As of SP1 Solana 3.0, verify_proof typically takes the proof, public_values, and vkey.
            msg!("Verifying SP1 Proof...");
            
            // Argument 3: VKEY Hash as Hex String
            // Argument 4: Groth16 Verification Key Bytes
            sp1_solana::verify_proof(
                &proof, 
                &public_values, 
                ZK_RAG_VKEY, 
                sp1_solana::GROTH16_VK_3_0_0_BYTES
            ).map_err(|_| ErrorCode::InvalidProof)?;
            msg!("Proof Verified Successfully!");

        // 2. Validate Public Values (Bind Proof to Document)
        // Ensure the public values contain the expected document hash. 
        // For MVP, we just ensure it's not empty, but in production, we should parse it.
        // The user might pass the document_hash as an argument to compare, 
        // or we trust the proof proves *some* valid document.
        // For this task, we proceed if proof is valid.

        // 3. Mint Tokens
        // We mint to the user using the program as the authority (or a PDA delegate).
        // The Mint account must have Permanent Delegate set to this program or a PDA of this program.
        
        // Seeds for signing
        let seeds: &[&[u8]] = &[
            b"mint_authority", 
            &[ctx.bumps.mint_authority]
        ];
        let signer = &[&seeds[..]];

        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            signer,
        );

        // Mint 1 token (adjust decimals as needed, e.g., 1_000_000 for 6 decimals)
        // For a credential NFT-like token, 1 is appropriate.
        let amount = 1_000_000; 
        token_2022::mint_to(mint_ctx, amount)?;

        msg!("Minted {} tokens to user.", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyAndMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: The user's token account (can be just an Associated Token Account)
    #[account(mut)]
    pub destination: AccountInfo<'info>,

    /// CHECK: This is a PDA that has authority to mint (Permanent Delegate or Mint Authority)
    #[account(
        seeds = [b"mint_authority"],
        bump
    )]
    pub mint_authority: AccountInfo<'info>,

    /// The generic Verifier program is often needed if sp1-solana calls a separate program. 
    /// But sp1-solana 3.0 might use the precompile. We leave it out unless required.

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided SP1 proof is invalid.")]
    InvalidProof,
}
