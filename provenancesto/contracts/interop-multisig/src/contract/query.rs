#[cfg(not(feature = "library"))]
use super::*;

/// Handle query requests for the provenance marker module.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    use QueryMsg::*;

    match msg {
        GetAdmins {} => try_get_admins(deps),
        GetVotes { tx_hash } => try_get_votes(deps, tx_hash),
        GetValidators {} => try_get_validators(deps),
    }
}

/// Query to get admin addresses.
///
/// Returns:-
///     List of admin addresses
fn try_get_admins(deps: Deps) -> Result<QueryResponse, StdError> {
    let admins = ADMINS.load(deps.storage)?;
    to_json_binary(&admins)
}

/// Query to get vote details.
///
/// Arguements:-
///     - Transaction Hash
///
/// Returns:-
///     Votes
fn try_get_votes(deps: Deps, tx_hash: String) -> Result<QueryResponse, StdError> {
    let votes = VOTES.load(deps.storage, tx_hash.as_bytes())?;
    to_json_binary(&votes)
}

/// Query to get validator addresses.
///
/// Returns:-
///     List of validator addresses
fn try_get_validators(deps: Deps) -> Result<QueryResponse, StdError> {
    let validators = VALIDATORS.load(deps.storage)?;
    to_json_binary(&validators)
}
