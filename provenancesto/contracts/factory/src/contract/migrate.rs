#[cfg(not(feature = "library"))]
use super::*;

/// Called when migrating a contract instance to a new code ID.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
