#[cfg(not(feature = "library"))]
use super::*;

/// Called when migrating a contract instance to a new code ID.
/// This function is use to perform smart contract upgradability in case of logic changes or upgradation.
/// Only the smart contract initializer account can call this function.
///
/// Events:
///     - provwasm.contracts.fund.migrate
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default().add_attribute("action", "provwasm.contracts.fund.migrate"))
}
