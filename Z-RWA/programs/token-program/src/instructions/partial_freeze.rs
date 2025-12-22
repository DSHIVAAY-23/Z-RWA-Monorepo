use super::*;

/// Function to freeze the tokens partially
///
/// This function can throw following errors:
pub fn partial_freeze(
    ctx: Context<PartialFreezeAccount>,
    token: String,
    amount: u64,
) -> Result<()> {
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let config = &mut ctx.accounts.config;
    let caller = &mut ctx.accounts.authority.to_account_info().key();

    // Ensuring authorized sender
    require!(
        sub_admins.contains(caller) || config.issuer.eq(caller) || config.transfer_agent.eq(caller),
        CustomError::Unauthorized
    );

    let partial_freeze = &mut ctx.accounts.partial_freeze;
    partial_freeze.amount += amount;
    config.frozen_tokens += amount;

    // Emit partially freeze event
    emit!(PartialFreezeEvent {
        address: ctx.accounts.user.to_account_info().key(),
        token,
        amount,
        total: partial_freeze.amount
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(token: String)]
pub struct PartialFreezeAccount<'info> {
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
        init_if_needed,
        seeds = [PARTIAL_FREEZE_TAG, token.as_bytes(), user.key().as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<PartialFreeze>() + 8
    )]
    pub partial_freeze: Account<'info, PartialFreeze>,

    /// CHECK: This is the user to be partial freezed
    #[account(mut)]
    pub user: UncheckedAccount<'info>,

    /// CHECK: the authority of the mint account
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
