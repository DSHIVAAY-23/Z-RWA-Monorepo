#[cfg(not(feature = "library"))]
use super::*;

/// Initialize the smart contract config state, then bind a name to the contract address.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    // admin info
    let admin = info.sender;
    ADMIN.save(deps.storage, &admin)?;
    SUB_ADMIN.save(deps.storage, &vec![admin])?;

    CODE_ID.save(deps.storage, &msg.code_id)?;

    // Dispatch messages to the name module handler and emit an event.
    Ok(Response::new().add_attributes(vec![attr("action", "provwasm.contracts.factory.init")]))
}
