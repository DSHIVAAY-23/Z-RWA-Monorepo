use super::*;

/// Function to update issuer
///
/// Arguements:-
///   - token: token name
///   - address: address of the new issuer
///  
/// Fails when:-
///   - signer is not sub admin
///
/// Emits update issuer event
pub fn update_issuer(
    ctx: Context<UpdateTokenConfig>,
    token: String,
    address: Pubkey,
) -> Result<()> {
    let caller = ctx.accounts.caller.to_account_info().key();
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let config = &mut ctx.accounts.config;

    // Ensuring authorized sender
    require!(sub_admins.contains(&caller), CustomError::Unauthorized);

    let old = config.issuer;
    config.issuer = address;

    // Emit update issuer event
    emit!(UpdateIssuerEvent {
        token,
        old,
        new: address,
    });
    Ok(())
}

/// Function to tokenization agent
///
/// Arguements:-
///   - token: token name
///   - address: address of the new tokenization agent
///   
/// Fails when:-
///   - signer is not sub admin
///
/// Emits update tokenization agent event
pub fn update_tokenization_agent(
    ctx: Context<UpdateTokenConfig>,
    token: String,
    address: Pubkey,
) -> Result<()> {
    let caller = ctx.accounts.caller.to_account_info().key();
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let config = &mut ctx.accounts.config;

    // Ensuring authorized sender
    require!(sub_admins.contains(&caller), CustomError::Unauthorized);

    let old = config.tokenization_agent;
    config.tokenization_agent = address;

    // Emit update tokenization agent event
    emit!(UpdateTokenizationAgentEvent {
        token,
        old,
        new: address,
    });
    Ok(())
}

/// Function to transfer agent
///
/// Arguements:-
///   - token: token name
///   - address: address of the new transfer agent
///    
/// Fails when:-
///   - signer is not sub admin
///
/// Emits update transfer agent event
pub fn update_transfer_agent(
    ctx: Context<UpdateTokenConfig>,
    token: String,
    address: Pubkey,
) -> Result<()> {
    let caller = ctx.accounts.caller.to_account_info().key();
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let config = &mut ctx.accounts.config;

    // Ensuring authorized sender
    require!(sub_admins.contains(&caller), CustomError::Unauthorized);

    let old = config.transfer_agent;
    config.transfer_agent = address;

    // Emit update transfer agent event
    emit!(UpdateTransferAgentEvent {
        token,
        old,
        new: address,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(token: String)]
pub struct UpdateTokenConfig<'info> {
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

    /// CHECK: The caller
    #[account(mut)]
    pub caller: Signer<'info>,
}
