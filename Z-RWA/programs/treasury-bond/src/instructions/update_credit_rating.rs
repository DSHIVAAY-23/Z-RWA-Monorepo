use super::*;

/// Function for updation of credit rating
pub fn update_cr_rate(ctx: Context<UpdateCreditRate>, token: String, rating: String) -> Result<()> {
    let caller = ctx.accounts.authority.to_account_info().key();
    let agent = &mut ctx.accounts.agent;
    let global_config = &mut ctx.accounts.global_config;

    // Ensuring authorized sender
    require!(agent.address.eq(&caller), CustomError::Unauthorized);

    global_config.credit_rating = rating.clone();

    // Emit update credit rating event
    emit!(UpdateCreditRatingEvent { token, rating });

    Ok(())
}

#[derive(Accounts)]
#[instruction(token: String)]
pub struct UpdateCreditRate<'info> {
    #[account(
        mut,
        seeds = [AGENT_TAG, token.as_ref()],
        bump,
    )]
    pub agent: Account<'info, Agent>,

    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_TAG, token.as_ref()],
        bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,
}
