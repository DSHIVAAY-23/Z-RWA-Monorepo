use super::*;

/// Function for execute transaction
/// Anyone can call this function
///
/// Arguements:-
///     - Source Chain
///     - Source Address
///     - Transaction Hash
///     - Payload Data
///
/// Fails when:-
///     - threshold is not met for the transaction
///     - cross program invocation fails
///
/// Emits execute transaction event
pub fn execute_transaction(
    ctx: Context<ExecuteTransactions>,
    params: ExecuteTransactionParams,
) -> Result<()> {
    let votes = &mut ctx.accounts.votes;
    let threshold = &ctx.accounts.threshold;

    let seeds = &[THRESHOLD_TAG, &[ctx.bumps.threshold]];
    let signer = [&seeds[..]];

    // Ensuring threshold is met
    require!(votes.yes >= threshold.value, CustomError::ThresholdNotMet);

    votes.set_status(Status::Approved);

    let cpi_program = ctx.accounts.interop_core_program.to_account_info();

    let cpi_accounts = ExecuteInstructions {
        maintainers: ctx.accounts.maintainers.to_account_info(),
        mint_account: ctx.accounts.mint_account.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        user: ctx.accounts.user.to_account_info(),
        executer: ctx.accounts.executer.to_account_info(),
        caller: ctx.accounts.caller.to_account_info(),
        base_token_program: ctx.accounts.base_token_program.to_account_info(),
        request: ctx.accounts.request.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer);

    interop_core::cpi::execute_instructions(
        cpi_ctx,
        params.source_chain,
        params.source_address,
        params.payload,
    )?;

    votes.clear();

    // Emit execute transaction event
    emit!(ExecuteTransactionEvent {
        tx_hash: params.tx_hash
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct ExecuteTransactions<'info> {
    #[account(
        seeds = [THRESHOLD_TAG],
        bump,
    )]
    pub threshold: Account<'info, Threshold>,

    #[account(
        mut,
        seeds = [VOTES_TAG],
        bump,
    )]
    pub votes: Account<'info, Votes>,

    /// CHECK: Maintainer pda of base token program
    pub maintainers: AccountInfo<'info>,

    /// CHECK: Executer of core program
    pub executer: AccountInfo<'info>,

    pub caller: Signer<'info>,

    /// CHECK: Mint Account of token program
    #[account(mut)]
    pub mint_account: AccountInfo<'info>,

    /// CHECK: To Account of token program
    #[account(mut)]
    pub user: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,

    /// CHECK: Interop Core Program
    pub interop_core_program: AccountInfo<'info>,

    /// CHECK: Custom Token Program Address
    pub base_token_program: AccountInfo<'info>,

    /// CHECK: Request
    pub request: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
