use super::*;

/// Function to initialize the contract
///
/// Arguements:-
///     - multisig: address of multisig contract's caller
///     - deployed_chain: name of deployed chain
///
/// Emits init event
pub fn initialize(ctx: Context<Initialize>, params: InitParams) -> Result<()> {
    let caller = ctx.accounts.authority.to_account_info().key();

    let maintainers = &mut ctx.accounts.maintainers;
    maintainers.save(caller);

    let executer = &mut ctx.accounts.executer;
    executer.address = params.multisig;

    // Emit init event
    emit!(InitEvent::new(caller, params));

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [MAINTAINERS_TAG],
        bump,
        payer = authority,
        space = std::mem::size_of::<Maintainers>() + 32
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(
        init,
        seeds = [EXECUTER_TAG],
        bump,
        payer = authority,
        space = std::mem::size_of::<Executer>() + 32
    )]
    pub executer: Account<'info, Executer>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
