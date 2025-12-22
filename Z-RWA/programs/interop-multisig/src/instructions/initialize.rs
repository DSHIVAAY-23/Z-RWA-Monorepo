use super::*;

/// Function to initialize the contract
///
/// Arguements:-
///     - threshold: threshold value for the transaction to happen
///
/// Emits init event
pub fn initialize(ctx: Context<Initialize>, threshold: u8) -> Result<()> {
    let caller = ctx.accounts.authority.to_account_info().key();

    let maintainers = &mut ctx.accounts.maintainers;
    maintainers.save(caller);

    let threshold_store = &mut ctx.accounts.threshold;
    threshold_store.value = threshold;

    let validators = &mut ctx.accounts.validators;
    validators.addresses = Vec::default();

    // Emit init event
    emit!(InitEvent {
        admin: caller,
        threshold
    });

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
        seeds = [VALIDATORS_TAG],
        bump,
        payer = authority,
        space = std::mem::size_of::<Validators>() + 32
    )]
    pub validators: Account<'info, Validators>,

    #[account(
        init,
        seeds = [THRESHOLD_TAG],
        bump,
        payer = authority,
        space = std::mem::size_of::<Threshold>() + 8
    )]
    pub threshold: Account<'info, Threshold>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
