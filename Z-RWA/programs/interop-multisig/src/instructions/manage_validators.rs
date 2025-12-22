use super::*;

/// Function to add validators
///
/// Arguements:-
///     - List of addresses to be added as validators
///
/// Fails when:-
///     - Caller is not one of the admin
///
/// Emits manage validator event with update_kind as Add
pub fn add_validators(ctx: Context<ManageValidators>, addresses: Vec<Pubkey>) -> Result<()> {
    let authority = ctx.accounts.authority.to_account_info().key();
    let maintainers = &ctx.accounts.maintainers;
    let validators = &mut ctx.accounts.validators;

    // Ensuring authorized sender
    require!(
        maintainers.admins.contains(&authority),
        CustomError::Unauthorized
    );

    validators.add_validators(addresses.clone());

    // Emit add validators event
    emit!(ManageValidatorsEvent {
        update_kind: UpdateKind::Add,
        addresses
    });

    Ok(())
}

/// Function to remove validators
///
/// Arguements:-
///     - List of addresses to be removed from validators
///
/// Fails when:-
///     - Caller is not one of the admin
///
/// Emits manage validator event with update_kind as Remove  
pub fn remove_validators(ctx: Context<ManageValidators>, addresses: Vec<Pubkey>) -> Result<()> {
    let authority = ctx.accounts.authority.to_account_info().key();
    let maintainers = &ctx.accounts.maintainers;
    let validators = &mut ctx.accounts.validators;

    // Ensuring authorized sender
    require!(
        maintainers.admins.contains(&authority),
        CustomError::Unauthorized
    );

    validators.remove_validators(addresses.clone());

    // Emit remove validators event
    emit!(ManageValidatorsEvent {
        update_kind: UpdateKind::Remove,
        addresses
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(addresses: Vec<Pubkey>)]
pub struct ManageValidators<'info> {
    #[account(
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(
        mut,
        seeds = [VALIDATORS_TAG],
        bump,
        realloc = std::mem::size_of::<Validators>() + ((addresses.len() + validators.addresses.len()) * 32),
        realloc::payer = authority,
        realloc::zero = false,
    )]
    pub validators: Account<'info, Validators>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
