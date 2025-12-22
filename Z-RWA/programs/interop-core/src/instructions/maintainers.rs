use super::*;

/// Function to add admins
///
/// Arguements:-
///     - List of addresses to be added as admin
///  
/// Fails when:-
///     - Caller is not one of the admin
///
/// Emits add admin event
pub fn add_admins(ctx: Context<UpdateAdmins>, addresses: Vec<Pubkey>) -> Result<()> {
    let caller = ctx.accounts.authority.to_account_info().key();
    let maintainers = &mut ctx.accounts.maintainers;

    // Ensuring authorized sender
    require!(
        maintainers.admins.contains(&caller),
        CustomError::Unauthorized
    );

    maintainers.add_admins(addresses.clone());

    // Emit add admins event
    emit!(AddAdminsEvent { addresses });

    Ok(())
}

/// Function to remove admins
///
/// Arguements:-
///     - List of addresses to be removed from admin
///
/// Fails when:-
///     - Caller is not one of the admin
///
/// Emits remove admin event
pub fn remove_admins(ctx: Context<UpdateAdmins>, addresses: Vec<Pubkey>) -> Result<()> {
    let caller = ctx.accounts.authority.to_account_info().key();
    let maintainers = &mut ctx.accounts.maintainers;

    // Ensuring authorized sender
    require!(
        maintainers.admins.contains(&caller),
        CustomError::Unauthorized
    );
    maintainers.remove_admins(addresses.clone());

    // Emit remove admins event
    emit!(RemoveAdminsEvent { addresses });

    Ok(())
}

#[derive(Accounts)]
#[instruction(addresses: Vec<Pubkey>)]
pub struct UpdateAdmins<'info> {
    #[account(
        mut,
        seeds = [MAINTAINERS_TAG],
        bump,
        realloc = std::mem::size_of::<Maintainers>() + ((addresses.len() + maintainers.admins.len()) * 32),
        realloc::payer = authority,
        realloc::zero = false,
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
