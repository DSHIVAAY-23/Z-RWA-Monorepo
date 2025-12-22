use super::*;

/// Function to update destination configuration
///
/// Arguements:-
///     - New threshold value
///
/// Fails when:-
///     - caller is not admin
///
/// Emits update threshold event
pub fn update_threshold_value(ctx: Context<UpdateThreshold>, threshold: u8) -> Result<()> {
    let caller = ctx.accounts.caller.to_account_info().key();
    let maintainers = &mut ctx.accounts.maintainers;

    // Ensuring authorized sender
    require!(
        maintainers.admins.contains(&caller),
        CustomError::Unauthorized
    );

    let threshold_store = &mut ctx.accounts.threshold;
    let old = threshold_store.value;
    threshold_store.value = threshold;

    // Emit update threshold event
    emit!(UpdateThresholdEvent {
        old,
        new: threshold
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct UpdateThreshold<'info> {
    #[account(
        mut,
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(
        mut,
        seeds = [THRESHOLD_TAG],
        bump,
    )]
    pub threshold: Account<'info, Threshold>,

    pub caller: Signer<'info>,
}
