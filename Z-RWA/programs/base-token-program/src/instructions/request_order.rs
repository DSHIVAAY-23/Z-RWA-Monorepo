use super::*;

/// Function to place orders
///
/// Arguements:-
///   - order_id: OrderId
///   - token: token name
///   - user: address where the tokens will be minted / burned
///   - amount: amount of tokens to be minted / burned
///   - request_type: can be either mint or burn
///  
/// This function can throw following errors:
///   - amount Can't Be Zero (when user passes 0 amount for mint).
///   - any issue in mint and burn functionalities
///
/// Emits request event
pub fn request_order(ctx: Context<RequestOrderAccounts>, params: RequestOrder) -> Result<()> {
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let caller = &ctx.accounts.payer.key;

    // Ensuring authorized sender
    require!(sub_admins.contains(&caller), CustomError::Unauthorized);

    // Check user balance first
    require!(params.amount > 0, CustomError::AmountCantBeZero);

    let seeds = &[MINT_TAG, params.token.as_bytes(), &[ctx.bumps.mint_account]];
    let signer = [&seeds[..]];
    let cpi_program = ctx.accounts.token_program.to_account_info();

    match params.request_type {
        RequestType::Mint => {
            // Create the MintTo struct for our context
            let cpi_accounts = MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.user.to_account_info(),
                authority: ctx.accounts.mint_account.to_account_info(),
            };

            token_2022::mint_to(
                CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer),
                params.amount,
            )?;
        }
        RequestType::Burn => {
            // Create the MintTo struct for our context
            let cpi_accounts = Burn {
                mint: ctx.accounts.mint_account.to_account_info(),
                from: ctx.accounts.user.to_account_info(),
                authority: ctx.accounts.mint_account.to_account_info(),
            };

            token_2022::burn(
                CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer),
                params.amount,
            )?;
        }
    }

    // Emit request event
    emit!(RequestEvent::new(params));

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: RequestOrder)]
pub struct RequestOrderAccounts<'info> {
    #[account(
        mut,
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    /// CHECK: This is the token that we want to mint / burn
    #[account(
        mut,
        seeds = [MINT_TAG, params.token.as_bytes()],
        bump,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        seeds = [params.order_id.to_string().as_bytes()],
        bump,
        space = std::mem::size_of::<Request>() + 8
    )]
    pub request: Account<'info, Request>,

    /// CHECK: This is the token account that we want to mint tokens to (ATA)
    #[account(mut)]
    pub user: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
