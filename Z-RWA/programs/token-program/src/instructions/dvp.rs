use super::*;

/// Function to mint the tokens and freeze same amount of tokens partially
///
/// This function can throw following errors:
///   - Amount Can't Be Zero (when user passes 0 amount for mint).
///   - If the caller is neither sub admin nor issuer
///   - If token limit exceeded
///   - If account is frozen
///   - If account is not whitelisted
///
/// Emits Dvp event
pub fn delivery_vs_payment(ctx: Context<DeliveryVsPayment>, params: TokenParams) -> Result<()> {
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let config = &ctx.accounts.config;
    let caller = ctx.accounts.authority.to_account_info().key();

    // Ensuring authorized sender
    require!(
        sub_admins.contains(&caller) || config.issuer.eq(&caller),
        CustomError::Unauthorized
    );

    // Ensure token limit not exceeded
    require!(
        (ctx.accounts.token_account.amount + params.amount) <= config.token_limit,
        CustomError::TokenLimitExceeded
    );

    // Ensure account not frozen
    require!(
        !ctx.accounts.token_account.is_frozen(),
        CustomError::AccountFrozen
    );

    // Check user balance first
    require!(params.amount > 0, CustomError::AmountCantBeZero);

    let whitelist = &mut ctx.accounts.whitelist;
    let country_codes = &ctx.accounts.config.country_codes;

    // Ensure country is authorised
    require!(
        country_codes.contains(&whitelist.country_code),
        CustomError::CountryCodeAuthorizationFailed
    );

    let seeds = &[MINT_TAG, params.name.as_bytes(), &[ctx.bumps.mint_account]];
    let signer = [&seeds[..]];
    let cpi_program = ctx.accounts.token_program.to_account_info();

    // Create the MintTo struct for our context
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint_account.to_account_info(),
        to: ctx.accounts.to_account.to_account_info(),
        authority: ctx.accounts.mint_account.to_account_info(),
    };

    token_2022::mint_to(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer),
        params.amount,
    )?;

    let partial_freeze = &mut ctx.accounts.partial_freeze;
    partial_freeze.amount += params.amount;

    let config = &mut ctx.accounts.config;
    config.frozen_tokens += params.amount;

    // Emit dvp event
    emit!(DvpEvent {
        token: params.name,
        amount: params.amount
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: TokenParams)]
pub struct DeliveryVsPayment<'info> {
    #[account(
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(
        seeds = [WHITELIST_TAG, params.name.as_bytes(), params.to_account.as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, WhitelistedUser>,

    #[account(
        mut,
        seeds = [CONFIG_TAG, params.name.as_bytes()],
        bump,
    )]
    pub config: Account<'info, TokenConfiguration>,

    /// CHECK: This is the token that we want to mint
    #[account(
        mut,
        seeds = [MINT_TAG, params.name.as_bytes()],
        bump,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        seeds = [PARTIAL_FREEZE_TAG, params.name.as_bytes(), params.to_account.key().as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<PartialFreeze>() + 8
    )]
    pub partial_freeze: Account<'info, PartialFreeze>,

    /// CHECK: This is the token account that we want to mint tokens to (ATA)
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
