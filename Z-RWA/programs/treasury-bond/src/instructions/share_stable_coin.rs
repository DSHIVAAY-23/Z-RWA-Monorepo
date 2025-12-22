use super::*;

/// Function for share stable coins
/// Stable coins must be transferred from `from` account to agent account before this function call.
/// For dividend share in token, `asset_type` will be `token`, for dividend share in stable coins, `asset_type` will be
/// `stable_coin` and for dividend share in fiat, `asset_type` will be `fiat`.
///
/// Notes
///     - Dividend stored during this function call is not used anywhere at the moment, that will be used in future
///       verisons.
///     - Logic for Fiat is not implemented yet, which is subjected to be added on future versions.
///
/// Fails when:-
///     - caller is not agent
///     - agent doesn't have the tokens for the token transactions
///     - agent doesn't have the stable coins for the stable coins
pub fn share_stable_coin(
    ctx: Context<ShareStableCoin>,
    params: ShareStableCoinParams,
) -> Result<()> {
    let caller = ctx.accounts.authority.to_account_info().key();
    let agent = ctx.accounts.agent.address;
    let treasury_manager = ctx.accounts.global_config.treasury_manager;

    // Ensuring authorised sender
    require!(
        agent.eq(&caller) || treasury_manager.eq(&caller),
        CustomError::Unauthorized
    );

    let seeds = &[
        GLOBAL_CONFIG_TAG,
        params.token.as_bytes(),
        &[ctx.bumps.global_config],
    ];
    let signer = [&seeds[..]];

    let cpi_program = ctx.accounts.token_program.to_account_info();

    // Create the Transfer struct for our context
    let cpi_accounts = TransferChecked {
        mint: ctx.accounts.mint_account.to_account_info(),
        to: ctx.accounts.to_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        from: ctx.accounts.from_account.to_account_info(),
    };

    transfer_checked(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer),
        params.payment,
        params.decimals,
    )?;

    // Emit share dividends event
    emit!(ShareStableCoinEvent::new(params));

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: ShareStableCoinParams)]
pub struct ShareStableCoin<'info> {
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

    /// CHECK: Mint Account of token program
    #[account(mut)]
    pub mint_account: AccountInfo<'info>,

    /// CHECK: This is the token account that we want to transfer tokens from
    #[account(mut)]
    pub from_account: AccountInfo<'info>,

    /// CHECK: To Account of token program
    #[account(mut)]
    pub to_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,
}
