use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("CPvaLtbD6WeA5j1Q5sR7T5rJ6n1VbM5Fj8yQd1d11Zq");

#[program]
pub mod compliance_vault {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        compliance_requirements: ComplianceRequirements,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.admin = ctx.accounts.admin.key();
        vault.kamino_vault = ctx.accounts.kamino_vault.key();
        vault.requirements = compliance_requirements;
        vault.total_compliant_deposits = 0;
        vault.bump = ctx.bumps.vault;
        Ok(())
    }

    pub fn deposit_with_proof(
        ctx: Context<DepositWithProof>,
        amount: u64,
        _proof: Vec<u8>,           // Groth16 proof bytes
        public_signals: Vec<u8>,  // Public outputs from circuit
    ) -> Result<()> {
        // Step 1: Verify ZK proof
        // TODO: Import your existing Groth16 verifier from Z-RWA main

        // Step 2: Parse public signals
        require!(public_signals.len() == 3, ErrorCode::InvalidProof);
        require!(public_signals[0] == 1, ErrorCode::AgeCheckFailed);
        require!(public_signals[1] == 1, ErrorCode::NationalityCheckFailed);
        require!(public_signals[2] == 1, ErrorCode::IncomeCheckFailed);

        // Step 3: CPI to Kamino vault deposit
        let mut data = anchor_lang::solana_program::hash::hash(b"global:deposit_reserve_liquidity").to_bytes()[..8].to_vec();
        data.extend_from_slice(&amount.to_le_bytes());

        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.kamino_program.key(),
            accounts: vec![
                AccountMeta::new(ctx.accounts.vault.key(), true), // owner (signer)
                AccountMeta::new(ctx.accounts.reserve.key(), false),
                AccountMeta::new_readonly(ctx.accounts.lending_market.key(), false),
                AccountMeta::new_readonly(ctx.accounts.lending_market_authority.key(), false),
                AccountMeta::new_readonly(ctx.accounts.reserve_liquidity_mint.key(), false),
                AccountMeta::new(ctx.accounts.reserve_liquidity_supply.key(), false),
                AccountMeta::new(ctx.accounts.reserve_collateral_mint.key(), false),
                AccountMeta::new(ctx.accounts.user_token_account.key(), false),
                AccountMeta::new(ctx.accounts.user_destination_collateral.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false), // token program
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false), // liquidity token program (assuming same)
                AccountMeta::new_readonly(ctx.accounts.instruction_sysvar_account.key(), false),
            ],
            data,
        };

        let seeds = &[
            b"compliance_vault",
            ctx.accounts.kamino_vault.key().as_ref(),
            &[ctx.accounts.vault.bump],
        ];
        let pda_signer = &[&seeds[..]];

        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.reserve.to_account_info(),
                ctx.accounts.lending_market.to_account_info(),
                ctx.accounts.lending_market_authority.to_account_info(),
                ctx.accounts.reserve_liquidity_mint.to_account_info(),
                ctx.accounts.reserve_liquidity_supply.to_account_info(),
                ctx.accounts.reserve_collateral_mint.to_account_info(),
                ctx.accounts.user_token_account.to_account_info(),
                ctx.accounts.user_destination_collateral.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.instruction_sysvar_account.to_account_info(),
                ctx.accounts.kamino_program.to_account_info(),
            ],
            pda_signer,
        )?;

        // Step 4: Update vault stats
        let vault = &mut ctx.accounts.vault;
        vault.total_compliant_deposits += amount;

        emit!(ComplianceDepositEvent {
            user: ctx.accounts.user.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn withdraw_with_proof(
        _ctx: Context<WithdrawWithProof>,
        _amount: u64,
        _proof: Vec<u8>,
        _public_signals: Vec<u8>,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + ComplianceVault::INIT_SPACE,
        seeds = [b"compliance_vault", kamino_vault.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, ComplianceVault>,

    /// CHECK: Kamino market or vault this wraps
    pub kamino_vault: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositWithProof<'info> {
    #[account(
        mut,
        seeds = [b"compliance_vault", kamino_vault.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, ComplianceVault>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    /// CHECK: Kamino program
    pub kamino_program: AccountInfo<'info>,
    /// CHECK: Kamino market
    pub kamino_vault: AccountInfo<'info>,
    /// CHECK: lending market ptr
    pub lending_market: AccountInfo<'info>,
    /// CHECK: lending market authority
    pub lending_market_authority: AccountInfo<'info>,
    /// CHECK: reserve
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    
    /// CHECK: reserve liquidity mint
    pub reserve_liquidity_mint: AccountInfo<'info>,
    /// CHECK: reserve liquidity supply
    #[account(mut)]
    pub reserve_liquidity_supply: AccountInfo<'info>,
    /// CHECK: reserve collateral mint
    #[account(mut)]
    pub reserve_collateral_mint: AccountInfo<'info>,

    /// CHECK: user destination collateral (minted ctokens go here)
    #[account(mut)]
    pub user_destination_collateral: AccountInfo<'info>,

    /// CHECK: instruction sysvar
    pub instruction_sysvar_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawWithProof<'info> {
    pub vault: Account<'info, ComplianceVault>,
}

#[account]
#[derive(InitSpace)]
pub struct ComplianceVault {
    pub admin: Pubkey,
    pub kamino_vault: Pubkey,
    pub requirements: ComplianceRequirements,
    pub total_compliant_deposits: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ComplianceRequirements {
    pub min_age: u8,
    pub required_nationality: [u8; 2],
    pub min_income_bracket: u8,
}

#[event]
pub struct ComplianceDepositEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("ZK proof verification failed")]
    InvalidProof,
    #[msg("Age requirement not met")]
    AgeCheckFailed,
    #[msg("Nationality requirement not met")]
    NationalityCheckFailed,
    #[msg("Income requirement not met")]
    IncomeCheckFailed,
}
