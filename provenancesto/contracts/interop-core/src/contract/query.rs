#[cfg(not(feature = "library"))]
use super::*;

/// Handle query requests for the provenance marker module.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetAdmins {} => try_get_admins(deps),
        QueryMsg::GetSourceChain {} => try_get_source_config(deps),
    }
}

/// Query to get admin addresses.
///
/// Returns:-
///     List of admin addresses
fn try_get_admins(deps: Deps) -> Result<QueryResponse, StdError> {
    let addresses = ADMINS.load(deps.storage)?;
    to_json_binary(&addresses)
}

/// Query to get source configuration.
///
/// Returns:-
///     Source Configuration
fn try_get_source_config(deps: Deps) -> Result<QueryResponse, StdError> {
    let source_chain = SOURCE_CHAIN.load(deps.storage)?;
    to_json_binary(&source_chain)
}
