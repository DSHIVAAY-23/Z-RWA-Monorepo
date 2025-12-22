use super::*;

/// Function for send instructions
/// This function supports batch operations, means multiple instructions can be sent simultaneously
///
/// Arguements:-
///     - List of Potfolios containing:-
///         1. Destination Chain
///         2. Destination Address
///         3. Investor Address
///         4. Token Address
///         5. Amount of Tokens
///         6. Order Id
///         7. Action, can be either mint, burn or acknowledgement
///
/// Fails when:-
///     - investor address parse fails
///     - token address parse fails
///     - amount parse fails
///
/// Emits send instruction events
pub fn send_instruction(ctx: Context<SendInstructions>, params: SendParams) -> Result<()> {
    let caller = ctx.accounts.caller.to_account_info().key;

    for portfolio in params.portfolios {
        let investor_address = Address::from_str(&portfolio.investor)
            .map_err(|_| CustomError::AddressConversionError)?;
        let token_address =
            Address::from_str(&portfolio.token).map_err(|_| CustomError::AddressConversionError)?;
        let amount: u128 = portfolio
            .amount
            .parse()
            .map_err(|_| CustomError::UintConversionError)?;

        let payload_bytes = encode(&[
            Token::Uint(portfolio.action.to_other_uint()),
            Token::Address(investor_address),
            Token::Uint(Uint::from(amount)),
            Token::Address(token_address),
            Token::String(id().to_string()),
            Token::Uint(Uint::from(portfolio.order_id)),
        ]);

        let payload = hex::encode(payload_bytes);

        // Emit send instruction event
        emit!(SendInstructionEvent {
            action: portfolio.action,
            source_chain: String::from("Solana"),
            source_address: id().to_string(),
            destination_chain: portfolio.dest_chain,
            destination_address: portfolio.dest_address,
            sender: *caller,
            payload: payload.to_string()
        });
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct SendInstructions<'info> {
    pub caller: Signer<'info>,
}
