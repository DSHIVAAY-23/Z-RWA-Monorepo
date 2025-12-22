use super::*;

/// Function for burn tokens for same chain burning
///
/// Arguements:-
///     - Order Id
///     - Token Name
///     - User Address
///     - Amount of tokens
///
/// Fails when:-
///     - payload encoding fails during interop smart contract calls
///
/// Emits burn event
pub fn burn_token(ctx: Context<BurnTokens>, params: Order) -> Result<()> {
    let cpi_program = ctx.accounts.base_token_program.to_account_info();

    let cpi_accounts = RequestOrderAccounts {
        maintainers: ctx.accounts.maintainers.to_account_info(),
        mint_account: ctx.accounts.mint_account.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        user: ctx.accounts.user.to_account_info(),
        request: ctx.accounts.request.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    let params = RequestOrder {
        amount: params.amount,
        order_id: params.order_id,
        token: params.token,
        user: params.user,
        request_type: RequestType::Burn,
    };
    base_token_program::cpi::request_orders(cpi_ctx, params.clone())?;

    // Emit burn event
    emit!(BurnEvent {
        token: params.token,
        amount: params.amount
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct BurnTokens<'info> {
    /// CHECK: Maintainer pda of base token program
    pub maintainers: AccountInfo<'info>,

    /// CHECK: Mint Account of token program
    #[account(mut)]
    pub mint_account: AccountInfo<'info>,

    /// CHECK: To Account of token program
    #[account(mut)]
    pub user: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,

    /// CHECK: Custom Token Program Address
    pub base_token_program: AccountInfo<'info>,

    /// CHECK: Request
    pub request: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
