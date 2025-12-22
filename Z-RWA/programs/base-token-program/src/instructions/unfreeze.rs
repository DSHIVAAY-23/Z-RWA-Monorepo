use super::*;

/// Function to unfreeze the accounts
///
/// Arguements:-
///   - token: token name
///
/// This function can throw following errors:-
///   - Signer does not belong to sub_admins, issuer and transfer agent
///
/// Emits unfreeze event
pub fn unfreeze(ctx: Context<UnfreezeUserAccount>, token: String) -> Result<()> {
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let config = &mut ctx.accounts.config;
    let caller = &mut ctx.accounts.caller.to_account_info().key();

    // Ensuring authorized sender
    require!(
        sub_admins.contains(caller) || config.issuer.eq(caller) || config.transfer_agent.eq(caller),
        CustomError::Unauthorized
    );

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let seeds = &[MINT_TAG, token.as_bytes(), &[ctx.bumps.mint_account]];
    let signer = [&seeds[..]];

    // Create the ThawAccount struct for our context
    let cpi_accounts = ThawAccount {
        mint: ctx.accounts.mint_account.to_account_info(),
        authority: ctx.accounts.mint_account.to_account_info(),
        account: ctx.accounts.user.to_account_info(),
    };
    token_2022::thaw_account(CpiContext::new_with_signer(
        cpi_program,
        cpi_accounts,
        &signer,
    ))?;

    // Emit unfreeze event
    emit!(UnfreezeEvent {
        address: ctx.accounts.user.to_account_info().key()
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(token: String)]
pub struct UnfreezeUserAccount<'info> {
    #[account(
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(
        seeds = [CONFIG_TAG, token.as_bytes()],
        bump,
    )]
    pub config: Account<'info, TokenConfiguration>,

    #[account(
        mut,
        seeds = [MINT_TAG, token.as_bytes()],
        bump,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    /// CHECK: This is the user to be unfreezed
    #[account(mut)]
    pub user: AccountInfo<'info>,

    /// CHECK: The caller
    #[account(mut)]
    pub caller: Signer<'info>,

    pub token_program: Program<'info, Token2022>,
}
