use super::*;

/// Initialize the smart contract config state, then bind a name to the contract address.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    SUB_ADMIN.save(deps.storage, &vec![info.sender])?;
    TOKENIZATION_AGENT.save(deps.storage, &msg.tokenization_agent)?;
    DEST_CONFIG.save(deps.storage, &msg.config)?;

    // Storing Denom
    DENOM.save(deps.storage, &msg.denom)?;

    let msgs = create_marker(0, msg.denom.clone(), env.contract.address)?;

    // Dispatch messages to the name module handler and emit an event.
    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "provwasm.contracts.cw20.marker.init")
        .add_attribute("denom", msg.denom))
}
