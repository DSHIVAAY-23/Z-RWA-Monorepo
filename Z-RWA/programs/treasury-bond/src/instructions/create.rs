use super::*;

/// Function to create the fund
pub fn create(ctx: Context<Create>, params: CreateParams) -> Result<()> {
    let now = Clock::get()
        .expect("Error getting current timestamp.")
        .unix_timestamp;

    let caller = ctx.accounts.authority.to_account_info().key();
    let agent = &mut ctx.accounts.agent;
    agent.address = caller;

    let global_config = &mut ctx.accounts.global_config;
    global_config.save(params.clone(), now, caller);

    // Emit create fund event
    emit!(CreateFundEvent::new(params));

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: CreateParams)]
pub struct Create<'info> {
    #[account(
        init,
        seeds = [GLOBAL_CONFIG_TAG, params.token.as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<GlobalConfig>() + 8
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        init,
        seeds = [AGENT_TAG, params.token.as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<Agent>() + 8
    )]
    pub agent: Account<'info, Agent>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
