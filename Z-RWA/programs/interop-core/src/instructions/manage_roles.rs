use super::*;

/// Function to manage roles
///
/// Fails when:-
///     - Caller is not one of the admin
///
/// Emits update executer event
pub fn manage_role(ctx: Context<ManageRoles>, role: Role) -> Result<()> {
    let caller = ctx.accounts.caller.to_account_info().key();
    let maintainers = &ctx.accounts.maintainers;

    // Ensuring authorized sender
    require!(
        maintainers.admins.contains(&caller),
        CustomError::Unauthorized
    );

    match role {
        Role::Executer { addr } => {
            let executer = &mut ctx.accounts.executer;
            executer.address = addr;

            // Emit update executer event
            emit!(UpdateExecuterEvent { address: addr });
        }
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct ManageRoles<'info> {
    #[account(
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(
        mut,
        seeds = [EXECUTER_TAG],
        bump,
    )]
    pub executer: Account<'info, Executer>,

    pub caller: Signer<'info>,
}
