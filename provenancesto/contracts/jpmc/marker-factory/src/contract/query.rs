use super::*;

/// Handle query requests for the provenance marker module.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    use QueryMsg::*;

    match msg {
        GetContractByDenom { denom } => to_binary(&get_contract_by_denom(deps.storage, denom)?),
        GetDenomByContract { addr } => to_binary(&get_denom_by_contract(deps.storage, addr)?),
        GetDecimals { denom } => try_get_decimals(deps, denom),
        GetTotalSupply { denom } => try_get_total_supply(deps, denom),
        GetBalanceOf { denom, address } => try_get_balance_of(deps, denom, address),
        GetAllowance {
            denom,
            owner,
            spender,
        } => try_get_allowance(deps, denom, owner, spender),
        GetFreezedAccounts { denom } => try_get_freezed_accounts(deps, denom),
        GetFrozenBalance { denom, address } => try_get_frozen_balance(deps, denom, address),
        GetSubAdmins {} => try_get_sub_admins(deps),
        GetAdmin {} => try_get_admin(deps),
        GetFrozenTokens { denom } => try_get_frozen_tokens(deps, denom),
        GetRequestOf { denom, request_id } => try_get_request_of(deps, denom, request_id),
        GetRequestAllowances {
            denom,
            owner,
            spender,
            request_type,
        } => try_get_request_allowance(deps, denom, owner, spender, request_type),
        GetBurnBalanceOf { denom, owner } => try_get_burn_balance_of(deps, denom, owner),
        GetCodeId {} => try_get_code_id(deps),
    }
}

// Query to get decimals
pub fn try_get_decimals(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let contract = get_contract_by_denom(deps.storage, denom)?;
    let query_msg = cw20_marker::msg::QueryMsg::GetName {};
    let decimal: u8 = deps.querier.query_wasm_smart(contract, &query_msg)?;

    to_binary(&decimal)
}

// Query to get total supply
pub fn try_get_total_supply(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let contract = get_contract_by_denom(deps.storage, denom)?;
    let query_msg = cw20_marker::msg::QueryMsg::GetTotalSupply {};
    let coin: Coin = deps.querier.query_wasm_smart(contract, &query_msg)?;

    to_binary(&coin)
}

// Query to get balance of
pub fn try_get_balance_of(
    deps: Deps,
    denom: String,
    address: Addr,
) -> Result<QueryResponse, StdError> {
    let contract = get_contract_by_denom(deps.storage, denom)?;
    let query_msg = cw20_marker::msg::QueryMsg::GetBalanceOf { address };
    let coin: Coin = deps.querier.query_wasm_smart(contract, &query_msg)?;

    to_binary(&coin)
}

// Query to get allowance
pub fn try_get_allowance(
    deps: Deps,
    denom: String,
    owner: Addr,
    spender: Addr,
) -> Result<QueryResponse, StdError> {
    let contract = get_contract_by_denom(deps.storage, denom)?;
    let query_msg = cw20_marker::msg::QueryMsg::GetAllowance { owner, spender };
    let allowance: Uint128 = deps.querier.query_wasm_smart(contract, &query_msg)?;

    to_binary(&allowance)
}

// Query to get freezed accounts
pub fn try_get_freezed_accounts(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let contract = get_contract_by_denom(deps.storage, denom)?;
    let query_msg = cw20_marker::msg::QueryMsg::GetFreezedAccounts {};
    let accounts: Option<Vec<Addr>> = deps.querier.query_wasm_smart(contract, &query_msg)?;

    to_binary(&accounts)
}

// Query to get frozen balance
pub fn try_get_frozen_balance(
    deps: Deps,
    denom: String,
    address: Addr,
) -> Result<QueryResponse, StdError> {
    let contract = get_contract_by_denom(deps.storage, denom)?;
    let query_msg = cw20_marker::msg::QueryMsg::GetFrozenBalance { address };
    let bal: Option<Uint128> = deps.querier.query_wasm_smart(contract, &query_msg)?;

    to_binary(&bal)
}

// Query for sub_admins.
fn try_get_sub_admins(deps: Deps) -> Result<QueryResponse, StdError> {
    let addresses = SUB_ADMIN.load(deps.storage)?;
    to_binary(&addresses)
}

// Query for admin.
fn try_get_admin(deps: Deps) -> Result<QueryResponse, StdError> {
    let address = ADMIN.load(deps.storage)?;
    to_binary(&address)
}

// Query to get frozen tokens
pub fn try_get_frozen_tokens(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let contract = get_contract_by_denom(deps.storage, denom)?;
    let query_msg = cw20_marker::msg::QueryMsg::GetFrozenTokens {};
    let token: Uint128 = deps.querier.query_wasm_smart(contract, &query_msg)?;

    to_binary(&token)
}

// Query to get request of
pub fn try_get_request_of(
    deps: Deps,
    denom: String,
    request_id: String,
) -> Result<QueryResponse, StdError> {
    let contract = get_contract_by_denom(deps.storage, denom)?;
    let query_msg = cw20_marker::msg::QueryMsg::GetRequestOf { request_id };
    let req: Request = deps.querier.query_wasm_smart(contract, &query_msg)?;

    to_binary(&req)
}

// Query to get request allowance
pub fn try_get_request_allowance(
    deps: Deps,
    denom: String,
    owner: Addr,
    spender: Addr,
    request_type: RequestType,
) -> Result<QueryResponse, StdError> {
    let contract = get_contract_by_denom(deps.storage, denom)?;
    let query_msg = cw20_marker::msg::QueryMsg::GetRequestAllowances {
        owner,
        spender,
        request_type,
    };
    let allowance: Uint128 = deps
        .querier
        .query_wasm_smart(contract, &query_msg)
        .unwrap_or_default();

    to_binary(&allowance)
}

// Query to get burn balance of
pub fn try_get_burn_balance_of(
    deps: Deps,
    denom: String,
    owner: Addr,
) -> Result<QueryResponse, StdError> {
    let contract = get_contract_by_denom(deps.storage, denom)?;
    let query_msg = cw20_marker::msg::QueryMsg::GetBurnBalanceOf { owner };
    let bal: Uint128 = deps.querier.query_wasm_smart(contract, &query_msg)?;

    to_binary(&bal)
}

// Query to get code id of token contract
pub fn try_get_code_id(deps: Deps) -> Result<QueryResponse, StdError> {
    let code_id = get_code_id(deps.storage)?;

    to_binary(&code_id)
}
