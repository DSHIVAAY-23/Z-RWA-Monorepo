use super::*;

/// Function to create token
///
/// Arguements:-
///   - id: unique id, emit as a part of event
///   - name: token name
///   - symbol: token symbol
///   - uri: url for the metadata stored
///   - decimals: decimal precision for the token
///   - issuer: issuer public key
///   - transfer_agent: transfer agent public key
///   - tokenization_agent: tokenization agent public key
///
/// Fails when:-
///   - signer is not from the sub admins set
///   - token metadata initialization fails
///   - update mint authority fails
///   - error with mint account reload
///   - update account lamports to minimum fails
///
/// Emits create token event
pub fn create_token(ctx: Context<CreateToken>, params: CreateTokenParams) -> Result<()> {
    let sub_admins = &ctx.accounts.maintainers.sub_admins;
    let caller = &ctx.accounts.payer.key;

    // Ensuring authorized sender
    require!(sub_admins.contains(&caller), CustomError::Unauthorized);

    let config = &mut ctx.accounts.config;
    config.save(params.clone());

    ctx.accounts.initialize_token_metadata(params.clone())?;
    ctx.accounts.update_mint_authority()?;
    ctx.accounts.mint_account.reload()?;

    // transfer minimum rent to mint account
    update_account_lamports_to_minimum_balance(
        ctx.accounts.mint_account.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    // Emit create token event
    emit!(CreateTokenEvent {
        id: params.id,
        name: params.name
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: CreateTokenParams)]
pub struct CreateToken<'info> {
    #[account(
        mut,
        seeds = [MAINTAINERS_TAG],
        bump,
    )]
    pub maintainers: Account<'info, Maintainers>,

    /// CHECK: Validate with constraint, also checked by metadata program
    #[account(
        init,
        seeds = [CONFIG_TAG, params.name.as_bytes()],
        bump,
        payer = payer,
        space = std::mem::size_of::<TokenConfiguration>() + 16
    )]
    pub config: Account<'info, TokenConfiguration>,

    /// CHECK: mint initialisation
    #[account(
        init,
        seeds = [MINT_TAG, params.name.as_bytes()],
        bump,
        payer = payer,
        mint::token_program = token_program,
        mint::decimals = params.decimals,
        mint::authority = payer,
        mint::freeze_authority = mint_account,
        extensions::metadata_pointer::authority = payer,
        extensions::metadata_pointer::metadata_address = mint_account,
        extensions::group_member_pointer::authority = payer,
        extensions::group_member_pointer::member_address = mint_account,
        // extensions::transfer_hook::authority = mint_account,
        // extensions::transfer_hook::program_id = crate::ID,
        extensions::close_authority::authority = payer,
        extensions::permanent_delegate::delegate = mint_account,
    )]
    pub mint_account: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token2022>,
}

impl<'info> CreateToken<'info> {
    #[inline(never)]
    fn initialize_token_metadata(&self, params: CreateTokenParams) -> ProgramResult {
        let cpi_accounts = TokenMetadataInitialize {
            program_id: self.token_program.to_account_info(),
            mint: self.mint_account.to_account_info(),
            metadata: self.mint_account.to_account_info(), // metadata account is the mint, since data is stored in mint
            mint_authority: self.payer.to_account_info(),
            update_authority: self.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        token_metadata_initialize(cpi_ctx, params.name, params.symbol, params.uri)?;

        Ok(())
    }

    #[inline(never)]
    fn update_mint_authority(&self) -> ProgramResult {
        let cpi_accounts = SetAuthority {
            current_authority: self.payer.to_account_info(),
            account_or_mint: self.mint_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        set_authority(
            cpi_ctx,
            anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType::MintTokens,
            Some(self.mint_account.key()),
        )?;

        Ok(())
    }
}

#[inline(never)]
pub fn update_account_lamports_to_minimum_balance<'info>(
    account: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()> {
    let extra_lamports = Rent::get()?.minimum_balance(account.data_len()) - account.get_lamports();
    if extra_lamports > 0 {
        invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                payer.key,
                account.key,
                extra_lamports,
            ),
            &[payer, account, system_program],
        )?;
    }
    Ok(())
}
