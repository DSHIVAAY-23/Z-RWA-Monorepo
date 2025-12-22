use super::*;

/// Function to burn the tokens fom
///
/// This function can throw following errors:
///   - Amount Can't Be Zero (when user passes 0 amount for burn).
pub fn burn_from(ctx: Context<BurnTokenFrom>, params: TokenParams) -> Result<()> {
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let config = &ctx.accounts.config;
    let caller = ctx.accounts.authority.to_account_info().key();

    // Ensuring authorized sender
    require!(
        sub_admins.contains(&caller)
            || config.issuer.eq(&caller)
            || config.tokenization_agent.eq(&caller),
        CustomError::Unauthorized
    );

    // Check amount first
    require!(params.amount > 0, CustomError::AmountCantBeZero);

    // Ensure country is authorised
    let whitelist = &mut ctx.accounts.whitelist;
    let country_codes = &ctx.accounts.config.country_codes;

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

    let seeds = &[MINT_TAG, params.name.as_bytes(), &[ctx.bumps.mint_account]];
    let signer = [&seeds[..]];

    let cpi_program = ctx.accounts.token_program.to_account_info();

    // Create the MintTo struct for our context
    let cpi_accounts = Burn {
        mint: ctx.accounts.mint_account.to_account_info(),
        from: ctx.accounts.from.to_account_info(),
        authority: ctx.accounts.mint_account.to_account_info(),
    };
    token_2022::burn(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer),
        params.amount,
    )?;

    // Emit burn event
    emit!(BurnEvent {
        token: params.name,
        amount: params.amount
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: TokenParams)]
pub struct BurnTokenFrom<'info> {
    #[account(
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(
        seeds = [CONFIG_TAG, params.name.as_bytes()],
        bump,
    )]
    pub config: Account<'info, TokenConfiguration>,

    #[account(
        seeds = [WHITELIST_TAG, params.name.as_bytes(), params.to_account.as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, WhitelistedUser>,

    #[account(
        init_if_needed,
        seeds = [PARTIAL_FREEZE_TAG, params.name.as_bytes(), params.to_account.as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<PartialFreeze>() + 8
    )]
    pub partial_freeze: Account<'info, PartialFreeze>,

    /// CHECK: This is the token that we want to mint
    #[account(
        mut,
        seeds = [MINT_TAG, params.name.as_bytes()],
        bump,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    /// CHECK: This is the token account that we want to burn tokens from
    #[account(mut)]
    pub from: AccountInfo<'info>,

    /// CHECK: the authority of the mint account
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,

    pub system_program: Program<'info, System>,
}
