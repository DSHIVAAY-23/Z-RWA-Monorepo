#[cfg(not(feature = "library"))]
use super::*;

/// Called when migrating a contract instance to a new code ID.
///
/// Event:-
///     provwasm.contracts.interop_core.migrate
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "provwasm.contracts.interop_core.migrate"))
}
