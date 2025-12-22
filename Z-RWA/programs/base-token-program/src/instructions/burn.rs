use super::*;

/// Function to burn the tokens
///
/// Arguements:-
///   - name: token name
///   - to_account: target account where the token is going to be burned from
///   - amount: amount of tokens to be burned
///
/// This function can throw following errors:
///   - Amount Can't Be Zero (when user passes 0 amount for burn).
///   - Signer is not the owner of the token
///   - Amount is frozen
///   
/// Emits burn event
pub fn burn(ctx: Context<BurnToken>, params: TokenParams) -> Result<()> {
    // Check amount first
    require!(params.amount > 0, CustomError::AmountCantBeZero);

    // Ensure balance available
    if ctx.accounts.from.amount > 0 {
        let frozen_amount = ctx.accounts.partial_freeze.amount;
        require!(
            (ctx.accounts.from.amount - params.amount) >= frozen_amount,
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
        authority: ctx.accounts.authority.to_account_info(),
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
pub struct BurnToken<'info> {
    #[account(
        mut,
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    /// CHECK: This is the token that we want to mint
    #[account(
        mut,
        seeds = [MINT_TAG, params.name.as_bytes()],
        bump,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    /// CHECK: This is the token account that we want to burn tokens from
    #[account(mut)]
    pub from: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds = [PARTIAL_FREEZE_TAG, params.name.as_bytes(), params.to_account.as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<PartialFreeze>() + 8
    )]
    pub partial_freeze: Account<'info, PartialFreeze>,

    /// CHECK: the authority of the mint account
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token2022>,

    pub system_program: Program<'info, System>,
}
