use super::*;

/// Function for updation of stable coins
pub fn update_stable_coin(ctx: Context<StableCoins>, params: StableCoinParams) -> Result<()> {
    let caller = ctx.accounts.authority.to_account_info().key();
    let maintainers = &mut ctx.accounts.maintainers;
    let stable_coins = &mut ctx.accounts.stable_coins;

    // Ensuring authorized sender
    require!(
        maintainers.admins.contains(&caller),
        CustomError::Unauthorized
    );

    match params.coin_type {
        CoinType::Usdc => match params.update_type {
            UpdateType::Add { address } => stable_coins.usdc = Some(address),
            UpdateType::Remove => stable_coins.usdc = None,
        },
        CoinType::Usdt => match params.update_type {
            UpdateType::Add { address } => stable_coins.usdt = Some(address),
            UpdateType::Remove => stable_coins.usdt = None,
        },
        CoinType::Dai => match params.update_type {
            UpdateType::Add { address } => stable_coins.dai = Some(address),
            UpdateType::Remove => stable_coins.dai = None,
        },
    }

    // Emit update stable coin event
    emit!(UpdateStableCoinEvent {
        coin_type: params.coin_type,
        update_type: params.update_type
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct StableCoins<'info> {
    #[account(
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    #[account(
        init_if_needed,
        seeds = [STABLE_COIN_TAG],
        bump,
        payer = authority,
        space = std::mem::size_of::<StableCoinStorage>() + 8
    )]
    pub stable_coins: Account<'info, StableCoinStorage>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
