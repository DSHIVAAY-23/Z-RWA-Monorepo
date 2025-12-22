use super::*;

/// Function to transfer the tokens
///
/// Arguements:-
///   - token: token name
///   - to_account: target account where the token is going to be transferred to
///   - amount: amount of tokens to be transferred
///
/// This function can throw following errors:
///   - amount can't Be Zero (when user passes 0 amount for force transfer).
///   - signer does not belong to sub_admins, issuer and transfer agent
///   - required balance is frozen
///   - sufficient balance is available
///   
/// Emits transfer event
pub fn transfer(ctx: Context<TransferTokens>, params: TransferParams) -> Result<()> {
    let config = &ctx.accounts.config;
    let caller = ctx.accounts.authority.to_account_info().key();

    let current_timestamp = Clock::get()
        .expect("Error getting current timestamp.")
        .unix_timestamp;

    // Ensure holding period passed
    require!(
        current_timestamp > config.holding_period,
        CustomError::TokenHeld
    );

    // Check user balance first
    require!(params.amount > 0, CustomError::AmountCantBeZero);

    let cpi_program = ctx.accounts.token_program.to_account_info();

    // Ensure balance available
    let frozen_amount = ctx.accounts.partial_freeze.amount;
    require!(
        (ctx.accounts.from_account.amount - params.amount) >= frozen_amount,
        CustomError::BalanceFrozen
    );

    let seeds = &[MINT_TAG, params.token.as_bytes(), &[ctx.bumps.mint_account]];
    let signer = [&seeds[..]];

    // Create the Transfer struct for our context
    let cpi_accounts = TransferChecked {
        mint: ctx.accounts.mint_account.to_account_info(),
        to: ctx.accounts.to_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        from: ctx.accounts.from_account.to_account_info(),
    };

    token_2022::transfer_checked(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer),
        params.amount,
        0,
    )?;

    // Emit transfer event
    emit!(TransferEvent {
        token: params.token,
        amount: params.amount,
        from: caller,
        to: ctx.accounts.to_account.to_account_info().key()
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: TransferParams)]
pub struct TransferTokens<'info> {
    #[account(
        init_if_needed,
        seeds = [PARTIAL_FREEZE_TAG, params.token.as_bytes(), params.to_account.as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<PartialFreeze>() + 8
    )]
    pub partial_freeze: Account<'info, PartialFreeze>,

    /// CHECK: This is the token that we want to mint
    #[account(
        mut,
        seeds = [MINT_TAG, params.token.as_bytes()],
        bump,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [CONFIG_TAG, params.token.as_bytes()],
        bump,
    )]
    pub config: Account<'info, TokenConfiguration>,

    /// CHECK: This is the token account that we want to transfer tokens from
    #[account(mut)]
    pub from_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: This is the token account that we want to transfer tokens to
    #[account(mut)]
    pub to_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: the authority of the mint account
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token2022>,

    pub system_program: Program<'info, System>,
}
