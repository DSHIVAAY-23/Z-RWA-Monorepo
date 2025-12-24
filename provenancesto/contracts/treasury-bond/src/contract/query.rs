#[cfg(not(feature = "library"))]
use super::*;

/// Handle query requests for treasury_contract contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    use QueryMsg::*;

    match msg {
        GetAdmins {} => try_get_admins(deps),
        GetAgentByDenom { denom } => try_get_agent_by_denom(deps, denom),
        GetConfig { denom } => try_get_config(deps, denom),
        GetPayments { denom, user } => try_get_payments(deps, denom, user),
    }
}

/// Query for admin.
///
/// Fails when:-
///     - admin list is not initialized
///
/// Returns:-
///     List of Admins
fn try_get_admins(deps: Deps) -> Result<QueryResponse, StdError> {
    let admins = ADMIN.load(deps.storage)?;
    to_json_binary(&admins)
}

/// Query Agent by Denom
///
///
/// Fails when:-
///     - agent is missing
/// Returns:-
///     Agent Address
fn try_get_agent_by_denom(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let agent = AGENT.load(deps.storage, denom.as_bytes())?;
    to_json_binary(&agent)
}

/// Query to get global config
///
///
/// Fails when:-
///     - global config is missing
/// Returns:-
///     Global Config
fn try_get_config(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let config = GLOBAL_CONFIG.load(deps.storage, denom.as_bytes())?;
    to_json_binary(&config)
}

/// Query to get payments
///
///
/// Fails when:-
///     - payment is missing
/// Returns:-
///     Payments
fn try_get_payments(deps: Deps, denom: String, user: Addr) -> Result<QueryResponse, StdError> {
    let key = Key::new(denom, user.clone()).as_bytes_std()?;
    let payments = PAYMENTS.load(deps.storage, &key)?;

    to_json_binary(&payments)
}
