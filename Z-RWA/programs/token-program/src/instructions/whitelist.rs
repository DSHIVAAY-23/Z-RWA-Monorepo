use super::*;

/// Function to whitelist the user
///
/// This function can throw following errors:
pub fn whitelist(ctx: Context<WhitelistUser>, params: WhitelistParams) -> Result<()> {
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

    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.country_code = params.code;

    // Emit whitelist event
    emit!(WhitelistEvent {
        token: params.token,
        address: params.user,
        country_code: params.code
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: WhitelistParams)]
pub struct WhitelistUser<'info> {
    #[account(
        mut,
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(
        seeds = [CONFIG_TAG, params.token.as_bytes()],
        bump,
    )]
    pub config: Account<'info, TokenConfiguration>,

    /// CHECK: Whitelist
    #[account(
        init_if_needed,
        seeds = [WHITELIST_TAG, params.token.as_bytes(), params.user.as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<WhitelistedUser>() + 8
    )]
    pub whitelist: Account<'info, WhitelistedUser>,

    /// CHECK: The authority of whitelist
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
