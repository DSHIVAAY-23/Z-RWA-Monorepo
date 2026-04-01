use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::extension::transfer_hook::TransferHook;
use anchor_spl::token_interface::{Mint, TokenAccount};
use spl_transfer_hook_interface::instruction::TransferHookInstruction;
use spl_tlv_account_resolution::state::ExtraAccountMeta;
use spl_tlv_account_resolution::account_data::ExtraAccountMetaList;
use spl_pod::optional_keys::OptionalNonZeroPubkey;

use crate::states::*;
use crate::errors::*;
use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList PDA
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(1).unwrap(),
        payer = payer,
    )]
    pub extra_account_meta_list: AccountInfo<'info>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_extra_account_meta_list(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
    let account_metas = vec![
        // Index 5: ComplianceRecord PDA of the destination wallet
        // We use a seed-based resolution: [b"compliance", destination_wallet.owner]
        ExtraAccountMeta::new_with_seeds(
            &[
                spl_tlv_account_resolution::state::Seed::Literal { bytes: b"compliance".to_vec() },
                spl_tlv_account_resolution::state::Seed::AccountKey { index: 2 }, // destination_token_account
            ],
            false, // is_signer
            false, // is_writable
        ).map_err(|_| CustomError::InvalidPayload)?,
    ];

    // Initialize the ExtraAccountMetaList data
    ExtraAccountMetaList::init::<TransferHookInstruction>(
        &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
        &account_metas,
    ).map_err(|_| CustomError::InvalidPayload)?;

    Ok(())
}

#[derive(Accounts)]
pub struct TransferHookExecute<'info> {
    #[account(
        token::mint = mint,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source_token owner
    pub owner: AccountInfo<'info>,
    /// CHECK: ExtraAccountMetaList PDA
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
    )]
    pub extra_account_meta_list: AccountInfo<'info>,

    /// CHECK: ComplianceRecord of the destination wallet
    /// This is automatically provided by the ExtraAccountMetaList logic
    #[account(
        seeds = [b"compliance", destination_token.owner.as_ref()],
        bump,
    )]
    pub compliance_record: Account<'info, ComplianceRecord>,
}

pub fn transfer_hook_execute(ctx: Context<TransferHookExecute>, amount: u64) -> Result<()> {
    msg!("Transfer Hook Execute: Checking compliance for {}", ctx.accounts.destination_token.owner);

    // Enforce compliance check
    require!(
        ctx.accounts.compliance_record.is_verified,
        CustomError::UnverifiedIdentity
    );

    Ok(())
}
