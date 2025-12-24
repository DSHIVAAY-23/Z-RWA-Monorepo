use super::*;

/// Called when migrating a contract instance to a new code ID.
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.migrate
#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    Ok(Response::new().add_attribute("action", "provwasm.contracts.custom_marker.migrate"))
}
