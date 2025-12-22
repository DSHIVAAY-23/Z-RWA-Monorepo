use super::*;

/// Function to unfreeze the tokens partially
///
/// Arguements:-
///   - token: token name
///   - amount: amount of tokens to be partially unfreezed
///
/// This function can throw following errors:
///  - Signer does not belong to sub_admins, issuer and transfer agent
///
/// Emits partial unfreeze event
pub fn partial_unfreeze(
    ctx: Context<PartialUnfreezeAccount>,
    token: String,
    amount: u64,
) -> Result<()> {
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let config = &mut ctx.accounts.config;
    let caller = ctx.accounts.caller.to_account_info().key();

    // Ensuring authorized sender
    require!(
        sub_admins.contains(&caller)
            || config.issuer.eq(&caller)
            || config.transfer_agent.eq(&caller),
        CustomError::Unauthorized
    );

    let partial_freeze = &mut ctx.accounts.partial_freeze;

    // Ensuring frozen amount must be greater than or equal to the reduced amount
    require!(
        partial_freeze.amount >= amount,
        CustomError::InsufficientFunds
    );

    partial_freeze.amount -= amount;
    config.frozen_tokens -= amount;

    // Emit partially unfreeze event
    emit!(PartialUnfreezeEvent {
        address: ctx.accounts.user.to_account_info().key(),
        token,
        amount,
        total: partial_freeze.amount
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(token: String)]
pub struct PartialUnfreezeAccount<'info> {
    #[account(
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(
        mut,
        seeds = [CONFIG_TAG, token.as_bytes()],
        bump,
    )]
    pub config: Account<'info, TokenConfiguration>,

    #[account(
        mut,
        seeds = [PARTIAL_FREEZE_TAG, token.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub partial_freeze: Account<'info, PartialFreeze>,

    /// CHECK: This is the user to be partial freezed
    #[account(mut)]
    pub user: UncheckedAccount<'info>,

    /// CHECK: The caller
    #[account(mut)]
    pub caller: Signer<'info>,
}
