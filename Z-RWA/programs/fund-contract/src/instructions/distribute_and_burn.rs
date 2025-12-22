use super::*;

/// Function for Distribute and Burn
/// Stable coins must be transferred from `from` account to agent account before this function call
/// This function is used to exchange stable coins with the a particular token holding by the investor. The stable
/// coins are transferred from agent account to the investor's account and tokens are burnt from the investor's
/// accounts.
///
/// Fails when:-
///     - caller is not agent
///     - investor doesn't have the tokens
///     - agent doesn't have the stable coins
pub fn distribute_and_burn(
    ctx: Context<DistributeAndBurn>,
    params: DistributionParams,
) -> Result<()> {
    let caller = ctx.accounts.authority.to_account_info().key();
    let agent = ctx.accounts.agent.address;
    let fund_manager = ctx.accounts.global_config.fund_manager;

    // Ensuring authorised sender
    require!(
        agent.eq(&caller) || fund_manager.eq(&caller),
        CustomError::Unauthorized
    );

    let seeds = &[
        GLOBAL_CONFIG_TAG,
        params.token.as_bytes(),
        &[ctx.bumps.global_config],
    ];
    let signer = [&seeds[..]];

    let mut cpi_program = ctx.accounts.custom_token_program.to_account_info();

    let cpi_accounts = BurnTokenFrom {
        authority: ctx.accounts.authority.to_account_info(),
        maintainers: ctx.accounts.maintainers.to_account_info(),
        whitelist: ctx.accounts.whitelist.to_account_info(),
        config: ctx.accounts.config.to_account_info(),
        mint_account: ctx.accounts.mint_account_token.to_account_info(),
        from: ctx.accounts.investor_token.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        partial_freeze: ctx.accounts.partial_freeze.to_account_info(),
        token_account: ctx.accounts.token_account.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer);

    let burn_params = TokenParams {
        name: params.token.to_string(),
        to_account: params.investor,
        amount: params.burn_amount,
    };
    token_program::cpi::burn_token_from(cpi_ctx, burn_params)?;

    cpi_program = ctx.accounts.token_program.to_account_info();

    // Create the Transfer struct for our context
    let cpi_accounts = TransferChecked {
        mint: ctx.accounts.mint_account_stable.to_account_info(),
        to: ctx.accounts.investor_stable.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        from: ctx.accounts.from_account_stable.to_account_info(),
    };

    transfer_checked(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer),
        params.distribution_amount,
        params.decimals,
    )?;

    // Emit distribute and burn event
    emit!(DistributeAndBurnEvent::new(params));

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: DistributionParams)]
pub struct DistributeAndBurn<'info> {
    #[account(
        seeds = [GLOBAL_CONFIG_TAG, params.token.as_ref()],
        bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        seeds = [AGENT_TAG, params.token.as_ref()],
        bump,
    )]
    pub agent: Account<'info, Agent>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Custom Token Program Address
    pub custom_token_program: AccountInfo<'info>,

    /// CHECK: Maintainers of token program
    pub maintainers: AccountInfo<'info>,

    /// CHECK: Whitelist of token program
    pub whitelist: AccountInfo<'info>,

    /// CHECK: Config of token program
    pub config: AccountInfo<'info>,

    /// CHECK: Stable Coin Mint Account Address
    #[account(mut)]
    pub mint_account_stable: AccountInfo<'info>,

    /// CHECK: Token Mint Account Address
    #[account(mut)]
    pub mint_account_token: AccountInfo<'info>,

    /// CHECK: This is the token account (ATA) that we want to transfer stable coins from
    #[account(mut)]
    pub from_account_stable: AccountInfo<'info>,

    /// CHECK: Stable Coin ATA for investor
    #[account(mut)]
    pub investor_stable: AccountInfo<'info>,

    /// CHECK: Token ATA for investor
    #[account(mut)]
    pub investor_token: AccountInfo<'info>,

    /// CHECK: Partial Freeze
    #[account(mut)]
    pub partial_freeze: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}
