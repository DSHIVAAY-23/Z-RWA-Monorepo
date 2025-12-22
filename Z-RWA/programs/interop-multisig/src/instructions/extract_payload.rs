use super::*;

/// Function for extract payload
/// Stable coins must be transferred from `from` account to agent account before this function call.
/// For dividend share in token, `asset_type` will be `token`, for dividend share in stable coins, `asset_type` will be
/// `stable_coin` and for dividend share in fiat, `asset_type` will be `fiat`.
///
/// Notes
///     - Dividend stored during this function call is not used anywhere at the moment, that will be used in future
///       verisons.
///     - Logic for Fiat is not implemented yet, which is subjected to be added on future versions.
///
/// Fails when:-
///     - caller is not agent
///     - agent doesn't have the tokens for the token transactions
///     - agent doesn't have the stable coins for the stable coins
pub fn extract_payload_data(ctx: Context<ExtractPayload>, payload: String) -> Result<()> {
    let payload_store = &mut ctx.accounts.payload;

    let payload_bytes = hex::decode(payload).map_err(|_| CustomError::HexDecodeError)?;

    let decoded = decode(
        &[
            ParamType::Uint(4),
            ParamType::String,
            ParamType::Uint(4),
            ParamType::String,
            ParamType::Uint(4),
        ],
        &payload_bytes,
    )
    .map_err(|_| CustomError::HexDecodeError)?;

    let investor = Pubkey::from_str(&decoded[1].to_string())
        .map_err(|_| CustomError::AddressConversionError)?;

    let token = decoded[3].to_string();

    let order_id_uint = decoded[4]
        .clone()
        .into_uint()
        .ok_or(CustomError::OrderIdConversionError)?;
    let order_id = order_id_uint.as_u128();

    payload_store.order_id = order_id;
    payload_store.investor = investor;
    payload_store.token = token.to_string();

    // Extract Payload event
    emit!(PayloadExtractedEvent {
        order_id,
        token,
        investor
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct ExtractPayload<'info> {
    #[account(
        init_if_needed,
        seeds = [PAYLOAD_TAG],
        bump,
        payer = authority,
        space = std::mem::size_of::<Payload>() + 32
    )]
    pub payload: Account<'info, Payload>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
