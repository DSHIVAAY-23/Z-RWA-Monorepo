use super::*;

/// Initialize the smart contract config state, then bind a name to the contract address.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    ADMIN.save(deps.storage, &info.sender)?;
    SUB_ADMIN.save(deps.storage, &vec![info.sender])?;

    CODE_ID.save(deps.storage, &msg.code_id)?;

    // Dispatch messages to the name module handler and emit an event.
    Ok(Response::new().add_attribute("action", "marker-factory.init"))
}
