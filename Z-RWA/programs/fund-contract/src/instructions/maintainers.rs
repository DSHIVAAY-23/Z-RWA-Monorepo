use super::*;

/// Function to add admins
pub fn add_admins(ctx: Context<ManageAdmins>, addresses: Vec<Pubkey>) -> Result<()> {
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
pub fn remove_admins(ctx: Context<ManageAdmins>, addresses: Vec<Pubkey>) -> Result<()> {
    let caller = ctx.accounts.authority.to_account_info().key();
    let maintainers = &mut ctx.accounts.maintainers;

    // Ensuring authorized sender
    require!(
        maintainers.admins.contains(&caller),
        CustomError::Unauthorized
    );
    maintainers.remove_admins(addresses.clone());

    // Emit remove sub admins event
    emit!(RemoveAdminsEvent { addresses });

    Ok(())
}

#[derive(Accounts)]
#[instruction(addresses: Vec<Pubkey>)]
pub struct ManageAdmins<'info> {
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
