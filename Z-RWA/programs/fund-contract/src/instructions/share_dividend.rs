use super::*;

/// Function for share dividend
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
pub fn share_dividend(ctx: Context<ShareDividends>, params: ShareDividendsParams) -> Result<()> {
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

    match params.asset_type {
        AssetType::Token => {
            let cpi_program = ctx.accounts.custom_token_program.to_account_info();

            let cpi_accounts = MintToken {
                authority: ctx.accounts.authority.to_account_info(),
                maintainers: ctx.accounts.maintainers.to_account_info(),
                whitelist: ctx.accounts.whitelist.to_account_info(),
                config: ctx.accounts.config.to_account_info(),
                mint_account: ctx.accounts.mint_account.to_account_info(),
                to_account: ctx.accounts.to_account.to_account_info(),
                token_account: ctx.accounts.token_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            };

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer);

            let mint_params = TokenParams {
                name: params.token.to_string(),
                to_account: params.to_account,
                amount: params.dividend,
            };
            token_program::cpi::mint_token(cpi_ctx, mint_params)?;
        }
        AssetType::StableCoin => {
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
                params.dividend,
                params.decimals,
            )?;
        }
    }

    // Emit share dividends event
    emit!(ShareDividendsEvent::new(params));

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: ShareDividendsParams)]
pub struct ShareDividends<'info> {
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

    pub system_program: Program<'info, System>,

    /// CHECK: Custom Token Program Address
    pub custom_token_program: AccountInfo<'info>,

    /// CHECK: Maintainers of token program
    pub maintainers: AccountInfo<'info>,

    /// CHECK: Whitelist of token program
    pub whitelist: AccountInfo<'info>,

    /// CHECK: Config of token program
    pub config: AccountInfo<'info>,

    /// CHECK: Mint Account of token program
    #[account(mut)]
    pub mint_account: AccountInfo<'info>,

    /// CHECK: This is the token account that we want to transfer tokens from
    #[account(mut)]
    pub from_account: AccountInfo<'info>,

    /// CHECK: To Account of token program
    #[account(mut)]
    pub to_account: AccountInfo<'info>,

    /// CHECK: Token Account of token program
    #[account(mut)]
    pub token_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,
}
