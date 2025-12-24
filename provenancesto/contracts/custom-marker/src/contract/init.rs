#[cfg(not(feature = "library"))]
use super::*;

/// Initialize the smart contract config state.
///
/// Event:-
///     provwasm.contracts.custom_marker.init
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<provwasm_std::ProvenanceQuery>,
    _env: Env,
    info: MessageInfo,
    _msg: InitMsg,
) -> Result<Response, ContractError> {
    // admin info, granting sender the admin role
    let admin = info.sender;
    ADMIN.save(deps.storage, &admin)?;

    // Dispatch messages to the name module handler and emit an event.
    Ok(Response::new().add_attribute("action", "provwasm.contracts.custom_marker.init"))
}
