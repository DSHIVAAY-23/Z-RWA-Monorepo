use super::*;

/// Function for execute instructions
///
/// Arguements:-
///     - source_chain: source chain name
///     - source_address: address of the source chain
///     - payload: payload data
///   
/// Fails when:-
///     - caller is not the executer
///
/// Fails when:-
///     - caller is not agent
///     - agent doesn't have the tokens for the token transactions
///     - agent doesn't have the stable coins for the stable coins
///
/// Emits execute instruction event
pub fn execute_instruction(
    ctx: Context<ExecuteInstructions>,
    source_chain: String,
    source_address: String,
    payload: String,
) -> Result<()> {
    let caller = ctx.accounts.caller.to_account_info().key;
    let executer = &ctx.accounts.executer;
    let payload_bytes = hex::decode(payload).map_err(|_| CustomError::HexDecodeError)?;

    // Ensuring authorized sender
    require_keys_eq!(executer.address, *caller, CustomError::Unauthorized);

    if let Ok(decoded) = decode(
        &[
            ParamType::Uint(4),
            ParamType::String,
            ParamType::Uint(4),
            ParamType::String,
            ParamType::Uint(4),
        ],
        &payload_bytes,
    ) {
        let action_uint = decoded[0]
            .clone()
            .into_uint()
            .ok_or(CustomError::ActionDecodeError)?;
        let action = Action::from_u32(action_uint.as_u32());

        let investor = Pubkey::from_str(&decoded[1].to_string())
            .map_err(|_| CustomError::AddressConversionError)?;

        let amount_uint = decoded[2]
            .clone()
            .into_uint()
            .ok_or(CustomError::AmountConversionError)?;
        let amount = amount_uint.as_u64();

        let token = decoded[3].to_string();

        let order_id_uint = decoded[4]
            .clone()
            .into_uint()
            .ok_or(CustomError::OrderIdConversionError)?;
        let order_id = order_id_uint.as_u128();

        let cpi_program = ctx.accounts.base_token_program.to_account_info();

        let cpi_accounts = RequestOrderAccounts {
            maintainers: ctx.accounts.maintainers.to_account_info(),
            mint_account: ctx.accounts.mint_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            request: ctx.accounts.request.to_account_info(),
            payer: ctx.accounts.caller.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let params = RequestOrder {
            amount,
            order_id,
            token,
            user: investor,
            request_type: action.to_request_type()?,
        };
        base_token_program::cpi::request_orders(cpi_ctx, params.clone())?;

        let ack = encode(&[
            Token::Uint(Action::Ack.to_other_uint()),
            Token::Uint(Uint::from(order_id)),
        ]);

        let payload = hex::encode(ack);

        // Emit execute instruction event
        emit!(ExecuteInstructionEvent {
            action: Action::Ack,
            source_chain: String::from("Solana"),
            source_address: id().to_string(),
            destination_chain: source_chain,
            destination_address: source_address,
            sender: *caller,
            payload
        });
    } else if let Ok(decoded) = decode(&[ParamType::Uint(4), ParamType::Uint(4)], &payload_bytes) {
        let action_uint = decoded[0]
            .clone()
            .into_uint()
            .ok_or(CustomError::ActionDecodeError)?;
        let action = Action::from_u32(action_uint.as_u32());

        let order_id_uint = decoded[1]
            .clone()
            .into_uint()
            .ok_or(CustomError::OrderIdConversionError)?;
        let order_id = order_id_uint.as_u128();

        // Ensuring valid action
        require!(action.eq(&Action::Ack), CustomError::InvalidAction);

        let ack = encode(&[
            Token::Uint(action.to_other_uint()),
            Token::Uint(Uint::from(order_id)),
        ]);
        let payload = hex::encode(ack);

        // Emit execute instruction event
        emit!(ExecuteInstructionEvent {
            action: Action::Ack,
            source_chain: String::from("Solana"),
            source_address: id().to_string(),
            destination_chain: source_chain,
            destination_address: source_address,
            sender: *caller,
            payload
        });
    } else {
        return Err(CustomError::InvalidAction.into());
    };

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct ExecuteInstructions<'info> {
    #[account(
        seeds = [EXECUTER_TAG],
        bump,
    )]
    pub executer: Account<'info, Executer>,

    /// CHECK: Maintainer pda of base token program
    pub maintainers: AccountInfo<'info>,

    pub caller: Signer<'info>,

    /// CHECK: Mint Account of token program
    #[account(mut)]
    pub mint_account: AccountInfo<'info>,

    /// CHECK: To Account of token program
    #[account(mut)]
    pub user: AccountInfo<'info>,

    pub token_program: Program<'info, Token2022>,

    /// CHECK: Custom Token Program Address
    pub base_token_program: AccountInfo<'info>,

    /// CHECK: Request
    pub request: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
