use super::*;

/// Function to transfer the tokens
///
/// This function can throw following errors:
///   - Amount Can't Be Zero (when user passes 0 amount for mint).
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

    // Ensure token limit not exceeded
    require!(
        (ctx.accounts.token_account.amount + params.amount) <= config.token_limit,
        CustomError::TokenLimitExceeded
    );

    // Check user balance first
    require!(params.amount > 0, CustomError::AmountCantBeZero);

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let whitelist = &mut ctx.accounts.whitelist;
    let country_codes = &ctx.accounts.config.country_codes;

    // Ensure country is authorised
    require!(
        country_codes.contains(&whitelist.country_code),
        CustomError::CountryCodeAuthorizationFailed
    );

    // Ensure balance available
    if ctx.accounts.token_account.amount > 0 {
        let frozen_amount = ctx.accounts.partial_freeze.amount;
        require!(
            (ctx.accounts.token_account.amount - params.amount) >= frozen_amount,
            CustomError::BalanceFrozen
        );
    }

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
        seeds = [WHITELIST_TAG, params.token.as_bytes(), params.to_account.as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, WhitelistedUser>,

    #[account(
        init_if_needed,
        seeds = [PARTIAL_FREEZE_TAG, params.token.as_bytes(), params.to_account.as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<PartialFreeze>() + 8
    )]
    pub partial_freeze: Account<'info, PartialFreeze>,

    #[account(
        seeds = [CONFIG_TAG, params.token.as_bytes()],
        bump,
    )]
    pub config: Account<'info, TokenConfiguration>,

    /// CHECK: This is the token that we want to mint
    #[account(
        mut,
        seeds = [MINT_TAG, params.token.as_bytes()],
        bump,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    /// CHECK: This is the token account that we want to transfer tokens from
    #[account(mut)]
    pub from_account: AccountInfo<'info>,

    /// CHECK: This is the token account that we want to transfer tokens to
    #[account(mut)]
    pub to_account: AccountInfo<'info>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: the authority of the mint account
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token2022>,

    pub system_program: Program<'info, System>,
}
