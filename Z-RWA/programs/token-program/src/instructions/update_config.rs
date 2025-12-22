use super::*;

/// Function to update token limit
pub fn update_token_limit(
    ctx: Context<UpdateTokenConfig>,
    token: String,
    limit: u64,
) -> Result<()> {
    let caller = ctx.accounts.caller.to_account_info().key();
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let config = &mut ctx.accounts.config;

    // Ensuring authorized sender
    require!(sub_admins.contains(&caller), CustomError::Unauthorized);

    let old_limit = config.token_limit;
    config.token_limit = limit;

    // Emit update token limit event
    emit!(UpdateTokenLimitEvent {
        token,
        old_limit,
        new_limit: limit
    });
    Ok(())
}

/// Function to add country codes
pub fn add_country_codes(
    ctx: Context<UpdateTokenConfig>,
    token: String,
    codes: Vec<u16>,
) -> Result<()> {
    let caller = ctx.accounts.caller.to_account_info().key();
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let config = &mut ctx.accounts.config;

    // Ensuring authorized sender
    require!(sub_admins.contains(&caller), CustomError::Unauthorized);

    let old_codes = config.country_codes.clone();
    config.add_country_codes(codes.clone());

    // Emit update country code event
    emit!(UpdateCountryCodesEvent {
        token,
        old_codes,
        new_codes: codes,
    });
    Ok(())
}

/// Function to remove country codes
pub fn remove_country_codes(
    ctx: Context<UpdateTokenConfig>,
    token: String,
    codes: Vec<u16>,
) -> Result<()> {
    let caller = ctx.accounts.caller.to_account_info().key();
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let config = &mut ctx.accounts.config;

    // Ensuring authorized sender
    require!(sub_admins.contains(&caller), CustomError::Unauthorized);

    let old_codes = config.country_codes.clone();
    config.remove_country_codes(codes.clone());

    // Emit update country code event
    emit!(UpdateCountryCodesEvent {
        token,
        old_codes,
        new_codes: codes,
    });
    Ok(())
}

/// Function to update issuer
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
