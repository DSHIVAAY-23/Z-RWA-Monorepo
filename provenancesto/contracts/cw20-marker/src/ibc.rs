use super::*;

/// Function for receive acknowledgement
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.receive_acknowledgement
pub fn receive_ack(
    deps: DepsMut,
    registry: String,
    channel: String,
    sequence: u64,
    ack: String,
    success: bool,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // let bin = Binary::from_base64(&ack).map_err(|_| ContractError::BinaryConversionError {
    //     msg: ack.to_string(),
    // })?;
    // let ack_msg: AckMessage = from_binary(&bin).map_err(|_| ContractError::DecodeError {
    //     msg: ack.to_string(),
    // })?;

    let ibc_response = IBCResponse {
        registry,
        channel,
        sequence,
        ack,
        success,
    };

    if IBC_RESPONSE
        .update(deps.storage, |mut responses| -> Result<_, ContractError> {
            responses.push(ibc_response.clone());
            Ok(responses)
        })
        .is_err()
    {
        IBC_RESPONSE.save(deps.storage, &vec![ibc_response])?;
    };

    Ok(Response::new().add_attribute(
        "action",
        "provwasm.contracts.custom_marker.receive_acknowledgement",
    ))
}

/// Function for receive timeout
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.receive_timseout
pub fn receive_timeout(
    deps: DepsMut,
    registry: String,
    channel: String,
    sequence: u64,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let ibc_response = IBCResponse {
        registry,
        channel,
        sequence,
        ack: String::default(),
        success: bool::default(),
    };

    if IBC_RESPONSE
        .update(deps.storage, |mut responses| -> Result<_, ContractError> {
            responses.push(ibc_response.clone());
            Ok(responses)
        })
        .is_err()
    {
        IBC_RESPONSE.save(deps.storage, &vec![ibc_response])?;
    };

    Ok(Response::new().add_attribute(
        "action",
        "provwasm.contracts.custom_marker.receive_timseout",
    ))
}
