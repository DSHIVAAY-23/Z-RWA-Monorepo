use super::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:axelar-router";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OPERATORS.save(deps.storage, &vec![info.sender.to_string()])?;

    Ok(Response::new())
}
