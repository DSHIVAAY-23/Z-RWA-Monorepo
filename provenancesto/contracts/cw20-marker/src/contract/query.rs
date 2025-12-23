use super::*;

/// Handle query requests for the provenance marker module.
#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetByAddress { address } => try_get_marker_by_address(deps, address),
        QueryMsg::GetByDenom { denom } => try_get_marker_by_denom(deps, denom),
        QueryMsg::GetFreezedAccounts { denom } => try_get_freezed_accounts(deps, denom),
        QueryMsg::GetFrozenBalance { denom, address } => {
            try_get_frozen_balance(deps, denom, address)
        }
        QueryMsg::GetSubAdmins {} => try_get_sub_admins(deps),
        QueryMsg::GetAdmin {} => try_get_admin(deps),
        QueryMsg::GetBalance { denom, address } => try_get_balance(deps, denom, address),
        QueryMsg::GetFrozenTokens { denom } => try_get_frozen_tokens(deps, denom),
        QueryMsg::GetCiculatingSupply { denom } => try_get_circultating_tokens(deps, denom),
        QueryMsg::GetIBCResponse {} => try_get_ibc_response(deps),
        QueryMsg::GetOperators {} => try_get_operators(deps),
        QueryMsg::GetRequestOf { order_id } => try_get_request_of(deps, order_id),
        QueryMsg::GetRequestBalanceOf {
            owner,
            request_type,
        } => try_get_request_balance_of(deps, owner, request_type),
        QueryMsg::GetDestConfig {} => try_get_dest_config(deps),
        QueryMsg::IsIbcResponseRequired {} => try_is_ibc_response_required(deps),
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
fn try_get_marker_by_address(deps: Deps, address: String) -> Result<QueryResponse, StdError> {
    let address = deps.api.addr_validate(&address)?;
    let querier = provwasm_std::ProvenanceQuerier::new(&deps.querier);
    let marker = get_marker_by_address(address, &querier)?;
    to_json_binary(&marker)
}

/// Query a marker by denom.
///
/// Fails when:-
///    denom is missing
///
/// Returns:-
///     Marker Struct, fields can be found here https://docs.rs/provwasm-std/1.2.0/provwasm_std/struct.Marker.html#fields
fn try_get_marker_by_denom(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let querier = provwasm_std::ProvenanceQuerier::new(&deps.querier);
    let marker = get_marker_by_denom(denom, &querier)?;
    to_json_binary(&marker)
}

/// Query freezed accounts.
///
/// Returns:-
///     List of freezed addresses
fn try_get_freezed_accounts(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let accounts = FREEZE_LIST.load(deps.storage, denom.as_bytes()).ok();
    to_json_binary(&accounts)
}

/// Query frozen balances by address.
///
/// Returns:-
///     Frozen Balance
fn try_get_frozen_balance(
    deps: Deps,
    denom: String,
    address: Addr,
) -> Result<QueryResponse, StdError> {
    let key = Key::new(denom, address).as_bytes_std()?;
    let bal = PARTIAL_FREEZE
        .load(deps.storage, &key)
        .unwrap_or(Uint128::zero());
    to_json_binary(&bal)
}

/// Query for sub_admins.
///
/// Returns:-
///     List of sub_admin addresses
fn try_get_sub_admins(deps: Deps) -> Result<QueryResponse, StdError> {
    let addresses = SUB_ADMIN.load(deps.storage)?;
    to_json_binary(&addresses)
}

/// Query for admin.
///
/// Returns:-
///     Admin Address
fn try_get_admin(deps: Deps) -> Result<QueryResponse, StdError> {
    let address = ADMIN.load(deps.storage)?;
    to_json_binary(&address)
}

/// Query to get operator list.
///
/// Returns:-
///     - List of Operators
fn try_get_operators(deps: Deps) -> Result<QueryResponse, StdError> {
    let addresses = OPERATORS.load(deps.storage)?;
    to_json_binary(&addresses)
}

/// Query to get balance of an address.
///
/// Returns:-
///     Balance
fn try_get_balance(deps: Deps, denom: String, address: Addr) -> Result<QueryResponse, StdError> {
    let balance = deps.querier.query_balance(address, denom)?;
    to_json_binary(&balance)
}

/// Query for frozen tokens
///
/// Returns:-
///     Total Frozen Tokens
fn try_get_frozen_tokens(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let tokens = FROZEN_TOKENS
        .load(deps.storage, denom.as_bytes())
        .unwrap_or(Uint128::zero());
    to_json_binary(&tokens)
}

/// Query for ciculating supply
///
/// Returns:-
///     Circulating Supply
fn try_get_circultating_tokens(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let circultating_tokens = MINTED_TOKENS
        .load(deps.storage, denom.as_bytes())
        .unwrap_or(Uint128::zero());

    to_json_binary(&circultating_tokens)
}

/// Query to get IBC Response
///
/// Returns:-
///     - List of IBC Responses
fn try_get_ibc_response(deps: Deps) -> Result<QueryResponse, StdError> {
    let res = IBC_RESPONSE.load(deps.storage)?;

    to_json_binary(&res)
}

/// Query to get requests
///
/// returns:-
///     - Request
pub fn try_get_request_of(deps: Deps, order_id: String) -> Result<QueryResponse, StdError> {
    let res = REQUESTS.load(deps.storage, order_id)?;

    to_json_binary(&res)
}

/// Query to get request balance of
///
/// Returns:-
///     - Balance (Uint128)
pub fn try_get_request_balance_of(
    deps: Deps,
    owner: Addr,
    request_type: RequestType,
) -> Result<QueryResponse, StdError> {
    let res = match request_type {
        RequestType::Mint => MINT_BALANCES.load(deps.storage, owner)?,
        RequestType::Burn => BURN_BALANCES.load(deps.storage, owner)?,
    };

    to_json_binary(&res)
}

/// Query to get destination configuration
///
/// Returns:-
///     - DestConfig
fn try_get_dest_config(deps: Deps) -> Result<QueryResponse, StdError> {
    let conig = DEST_CONFIG.load(deps.storage)?;

    to_json_binary(&conig)
}

/// Query for ibc reponse
///
/// Returns:-
///     either true or false
fn try_is_ibc_response_required(deps: Deps) -> Result<QueryResponse, StdError> {
    let response = IS_IBC_RESPONSE_REQUIRED.load(deps.storage)?;

    to_json_binary(&response)
}

/// Query for holding peiod
///
/// Returns:-
///     Timestamp for holding period in seconds
fn try_get_hold_period(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let holding_period = HOLDING_PERIOD
        .load(deps.storage, denom.as_bytes())
        .unwrap_or_default();

    to_json_binary(&holding_period)
}
