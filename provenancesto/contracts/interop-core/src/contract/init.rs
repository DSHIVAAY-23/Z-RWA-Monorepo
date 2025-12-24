#[cfg(not(feature = "library"))]
use super::*;

/// Initialize the smart contract config state.
///
/// Event:-
///     provwasm.contracts.interop_core.init
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    // admin info, granting sender the admin role
    let admin = info.sender;
    ADMINS.save(deps.storage, &vec![admin])?;
    EXECUTER.save(deps.storage, &msg.multi_sig)?;

    SOURCE_CHAIN.save(deps.storage, &msg.deployed_chain)?;

    // Dispatch messages to the name module handler and emit an event.
    Ok(Response::new().add_attribute("action", "provwasm.contracts.interop_core.init"))
}
