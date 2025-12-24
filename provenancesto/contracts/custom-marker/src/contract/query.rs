#![cfg(not(feature = "library"))]
use super::*;
use crate::{
    get_marker_by_address, get_marker_by_denom, Key, DENOM_CONFIG, FREEZE_LIST, HOLDING_PERIOD,
    MINTED_TOKENS, PARTIAL_FREEZE,
};
use cosmwasm_std::{to_json_binary, Addr, Deps, Env, QueryResponse, StdError, Uint128};
use provwasm_std::{ProvenanceQuerier, ProvenanceQuery};

/// Handle query requests for the provenance marker module.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps<ProvenanceQuery>,
    _env: Env,
    msg: QueryMsg,
) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetByAddress { address } => try_get_marker_by_address(deps, address),
        QueryMsg::GetByDenom { denom } => try_get_marker_by_denom(deps, denom),
        QueryMsg::GetAuthorizedCountries { denom } => try_get_auth_countries(deps, denom),
        QueryMsg::GetFreezedAccounts { denom } => try_get_freezed_accounts(deps, denom),
        QueryMsg::GetFrozenBalance { denom, address } => {
            try_get_frozen_balance(deps, denom, address)
        }
        QueryMsg::GetDenomConfig { denom } => try_get_denom_config(deps, denom),
        QueryMsg::GetCountryCodeByAddress { denom, address } => {
            try_get_country_code_by_address(deps, denom, address)
        }
        QueryMsg::GetSubAdmins {} => try_get_sub_admins(deps),
        QueryMsg::GetAdmin {} => try_get_admin(deps),
        QueryMsg::GetBalance { denom, address } => try_get_balance(deps, denom, address),
        QueryMsg::GetFrozenTokens { denom } => try_get_frozen_tokens(deps, denom),
        QueryMsg::GetCiculatingSupply { denom } => try_get_circultating_tokens(deps, denom),
        QueryMsg::GetHoldPeriod { denom } => try_get_hold_period(deps, denom),
    }
}

/// Query a marker by address.
///
/// Fails when:-
///     marker account address is missing
///
/// Returns:-
///     Marker Struct, fields can be found here https://docs.rs/provwasm-std/1.2.0/provwasm_std/struct.Marker.html#fields
fn try_get_marker_by_address(
    deps: Deps<ProvenanceQuery>,
    address: String,
) -> Result<QueryResponse, StdError> {
    let address = deps.api.addr_validate(&address)?;
    let prov = ProvenanceQuerier::new(&deps.querier);
    let marker = get_marker_by_address(address, &prov)?;
    to_json_binary(&marker)
}

/// Query a marker by denom.
///
/// Fails when:-
///    denom is missing
///
/// Returns:-
///     Marker Struct, fields can be found here https://docs.rs/provwasm-std/1.2.0/provwasm_std/struct.Marker.html#fields
fn try_get_marker_by_denom(
    deps: Deps<ProvenanceQuery>,
    denom: String,
) -> Result<QueryResponse, StdError> {
    let prov = ProvenanceQuerier::new(&deps.querier);
    let marker = get_marker_by_denom(denom, &prov)?;
    to_json_binary(&marker)
}

/// Query authorized countries.
///
/// Returns:-
///     List of authorized country codes
fn try_get_auth_countries(
    deps: Deps<ProvenanceQuery>,
    denom: String,
) -> Result<QueryResponse, StdError> {
    let denom_config = DENOM_CONFIG.load(deps.storage, denom.as_bytes())?;
    to_json_binary(&denom_config.country_codes)
}

/// Query freezed accounts.
///
/// Returns:-
///     List of freezed addresses
fn try_get_freezed_accounts(
    deps: Deps<ProvenanceQuery>,
    denom: String,
) -> Result<QueryResponse, StdError> {
    let accounts = FREEZE_LIST.load(deps.storage, denom.as_bytes()).ok();
    to_json_binary(&accounts)
}

/// Query frozen balances by address.
///
/// Returns:-
///     Frozen Balance
fn try_get_frozen_balance(
    deps: Deps<ProvenanceQuery>,
    denom: String,
    address: Addr,
) -> Result<QueryResponse, StdError> {
    let key = Key::new(denom, address).as_bytes_std()?;
    let bal = PARTIAL_FREEZE.load(deps.storage, &key).unwrap_or_default();
    to_json_binary(&bal)
}

/// Query denom config by denom.
///
/// Returns:-
///     Denom Config
fn try_get_denom_config(
    deps: Deps<ProvenanceQuery>,
    denom: String,
) -> Result<QueryResponse, StdError> {
    let denom_config = DENOM_CONFIG.load(deps.storage, denom.as_bytes())?;
    to_json_binary(&denom_config)
}

/// Query whether the address is whitelisted or not. If some country code is returned then it means the particular address
/// is whitelisted.
///
/// Returns:-
///     Country Code
fn try_get_country_code_by_address(
    deps: Deps<ProvenanceQuery>,
    denom: String,
    address: Addr,
) -> Result<QueryResponse, StdError> {
    let key = Key::new(denom, address).as_bytes_std()?;
    let code = WHITELIST.load(deps.storage, &key)?;
    to_json_binary(&code)
}

/// Query for sub_admins.
///
/// Returns:-
///     List of sub_admin addresses
fn try_get_sub_admins(deps: Deps<ProvenanceQuery>) -> Result<QueryResponse, StdError> {
    let addresses = SUB_ADMIN.load(deps.storage)?;
    to_json_binary(&addresses)
}

/// Query for admin.
///
/// Returns:-
///     Admin Address
fn try_get_admin(deps: Deps<ProvenanceQuery>) -> Result<QueryResponse, StdError> {
    let address = ADMIN.load(deps.storage)?;
    to_json_binary(&address)
}

/// Query to get balance of an address.
///
/// Returns:-
///     Balance
fn try_get_balance(
    deps: Deps<ProvenanceQuery>,
    denom: String,
    address: Addr,
) -> Result<QueryResponse, StdError> {
    let balance = deps.querier.query_balance(address, denom)?;
    to_json_binary(&balance)
}

/// Query for frozen tokens
///
/// Returns:-
///     Total Frozen Tokens
fn try_get_frozen_tokens(
    deps: Deps<ProvenanceQuery>,
    denom: String,
) -> Result<QueryResponse, StdError> {
    let tokens = FROZEN_TOKENS
        .load(deps.storage, denom.as_bytes())
        .unwrap_or(Uint128::zero());
    to_json_binary(&tokens)
}

/// Query for ciculating supply
///
/// Returns:-
///     Circulating Supply
fn try_get_circultating_tokens(
    deps: Deps<ProvenanceQuery>,
    denom: String,
) -> Result<QueryResponse, StdError> {
    let circultating_tokens = MINTED_TOKENS
        .load(deps.storage, denom.as_bytes())
        .unwrap_or_default();

    to_json_binary(&circultating_tokens)
}

/// Query for holding peiod
///
/// Returns:-
///     Timestamp for holding period in seconds
fn try_get_hold_period(
    deps: Deps<ProvenanceQuery>,
    denom: String,
) -> Result<QueryResponse, StdError> {
    let holding_period = HOLDING_PERIOD
        .load(deps.storage, denom.as_bytes())
        .unwrap_or_default();

    to_json_binary(&holding_period)
}
