#[cfg(not(feature = "library"))]
use super::*;

/// Handle query requests for fund contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetAdmins {} => try_get_admins(deps),
        QueryMsg::GetManagementFees { denom, user } => try_get_management_fees(deps, denom, user),
        QueryMsg::GetNav { denom } => try_get_nav(deps, denom),
        QueryMsg::GetAum { denom } => try_get_aum(deps, denom),
        QueryMsg::GetAgentByDenom { denom } => try_get_agent_by_denom(deps, denom),
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

/// Query Management Fee for particular denom and user address combination.
///
/// Returns:-
///     Management Fees
fn try_get_management_fees(
    deps: Deps,
    denom: String,
    user: Addr,
) -> Result<QueryResponse, StdError> {
    let key = Key::new(denom, user).as_bytes_std()?;
    let management_fee = MANAGEMENT_FEES.load(deps.storage, &key).unwrap_or_default();
    to_json_binary(&management_fee)
}

/// Query Net Asset Value (NAV) for a particular denom.
///
/// Returns:-
///     Net Asset Value (NAV)
fn try_get_nav(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let global_config = GLOBAL_CONFIG.load(deps.storage, denom.as_bytes())?;
    let nav = global_config.nav_latest_price;
    to_json_binary(&nav)
}

/// Query Asset Under Management (AUM) for a particular denom.
///
/// Returns:-
///     Asset Under Management (AUM)
fn try_get_aum(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let global_config = GLOBAL_CONFIG.load(deps.storage, denom.as_bytes())?;
    let query_msg = custom_marker::msg::QueryMsg::GetCiculatingSupply { denom };
    let circulating_supply: Uint128 = deps
        .querier
        .query_wasm_smart(MARKER_CONTRACT_ADDRESS, &query_msg)?;

    to_json_binary(&(circulating_supply * global_config.nav_latest_price))
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
