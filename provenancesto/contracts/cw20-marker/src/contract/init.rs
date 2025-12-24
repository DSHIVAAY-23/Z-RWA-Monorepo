use super::*;

/// Initialize the smart contract config state, then bind a name to the contract address.
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.init
#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InitMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // admin info
    let admin = info.sender;
    ADMIN.save(deps.storage, &admin)?;

    // Dispatch messages to the name module handler and emit an event.
    Ok(Response::new().add_attribute("action", "provwasm.contracts.custom_marker.init"))
}
