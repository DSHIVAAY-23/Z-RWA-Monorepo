use super::*;

/// Function to update agent
pub fn update_agent(ctx: Context<UpdateAgent>, token: String, address: Pubkey) -> Result<()> {
    let caller = ctx.accounts.authority.to_account_info().key();
    let maintainers = &mut ctx.accounts.maintainers;
    let agent = &mut ctx.accounts.agent;

    // Ensuring authorized sender
    require!(
        maintainers.admins.contains(&caller),
        CustomError::Unauthorized
    );
    agent.address = address;

    // Emit add agent event
    emit!(UpdateAgentEvent { token, address });

    Ok(())
}

#[derive(Accounts)]
#[instruction(token: String)]
pub struct UpdateAgent<'info> {
    #[account(
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(
        mut,
        seeds = [AGENT_TAG, token.as_ref()],
        bump,
    )]
    pub agent: Account<'info, Agent>,

    pub authority: Signer<'info>,
}
