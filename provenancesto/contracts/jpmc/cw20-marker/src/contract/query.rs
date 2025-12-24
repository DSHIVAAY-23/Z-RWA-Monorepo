use super::*;

/// Handle query requests for the provenance marker module.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    use QueryMsg::*;

    match msg {
        GetByAddress { address } => try_get_marker_by_address(deps, address),
        GetByDenom {} => try_get_marker_by_denom(deps),
        GetFreezedAccounts {} => try_get_freezed_accounts(deps),
        GetFrozenBalance { address } => try_get_frozen_balance(deps, address),
        GetSubAdmins {} => try_get_sub_admins(deps),
        GetBalanceOf { address } => try_get_balance(deps, address),
        GetFrozenTokens {} => try_get_frozen_tokens(deps),
        GetRequestOf { request_id } => try_get_request_of(deps, request_id),
        GetRequestAllowances {
            owner,
            spender,
            request_type,
        } => try_get_request_allowances(deps, owner, spender, request_type),
        GetBurnBalanceOf { owner } => try_get_burn_balance_of(deps, owner),
        GetName {} => try_get_denom(deps),
        GetSymbol {} => try_get_denom(deps),
        GetDecimals {} => to_binary(&0),
        GetTotalSupply {} => try_get_supply(deps),
        GetAllowance { owner, spender } => try_get_allowance(deps, owner, spender),
        GetTokenizationAgent {} => try_get_tokenization_agent(deps),
        GetDestConfig {} => try_get_dest_config(deps),
    }
}

// Query a marker by address.
fn try_get_marker_by_address(deps: Deps, address: String) -> Result<QueryResponse, StdError> {
    let address = deps.api.addr_validate(&address)?;
    let querier = MarkerQuerier::new(&deps.querier);
    let marker = get_marker_by_address(address, &querier)?;
    to_binary(&marker)
}

// Query a marker by denom.
fn try_get_marker_by_denom(deps: Deps) -> Result<QueryResponse, StdError> {
    let querier = MarkerQuerier::new(&deps.querier);
    let denom = get_denom(deps.storage)?;
    let marker = get_marker_by_denom(denom, &querier)?;
    to_binary(&marker)
}

// Query to get denom name
fn try_get_denom(deps: Deps) -> Result<QueryResponse, StdError> {
    let denom = get_denom(deps.storage)?;
    to_binary(&denom)
}

// Query freezed accounts.
fn try_get_freezed_accounts(deps: Deps) -> Result<QueryResponse, StdError> {
    let accounts = FREEZE_LIST.load(deps.storage).ok();
    to_binary(&accounts)
}

// Query frozen balances by address.
fn try_get_frozen_balance(deps: Deps, address: Addr) -> Result<QueryResponse, StdError> {
    let bal = PARTIAL_FREEZE
        .load(deps.storage, address)
        .unwrap_or(Uint128::zero());
    to_binary(&bal)
}

// Query for sub_admins.
fn try_get_sub_admins(deps: Deps) -> Result<QueryResponse, StdError> {
    let addresses = SUB_ADMIN.load(deps.storage)?;
    to_binary(&addresses)
}

// Query to get supply.
fn try_get_supply(deps: Deps) -> Result<QueryResponse, StdError> {
    let denom = get_denom(deps.storage)?;
    let supply = deps.querier.query_supply(denom)?;
    to_binary(&supply)
}

// Query to get balance of an address.
fn try_get_balance(deps: Deps, address: Addr) -> Result<QueryResponse, StdError> {
    let denom = get_denom(deps.storage)?;
    let balance = deps.querier.query_balance(address, denom)?;
    to_binary(&balance)
}

// Query for frozen tokens
fn try_get_frozen_tokens(deps: Deps) -> Result<QueryResponse, StdError> {
    let tokens = FROZEN_TOKENS.load(deps.storage).unwrap_or(Uint128::zero());
    to_binary(&tokens)
}

// Query to get request of
fn try_get_request_of(deps: Deps, request_id: String) -> Result<QueryResponse, StdError> {
    let res = REQUESTS.load(deps.storage, request_id.as_bytes())?;

    to_binary(&res)
}

// Query to get request allowances
fn try_get_request_allowances(
    deps: Deps,
    owner: Addr,
    spender: Addr,
    request_type: RequestType,
) -> Result<QueryResponse, StdError> {
    let key = Key::new(owner, spender).as_bytes()?;

    let res = match request_type {
        RequestType::Mint => MINT_ALLOWANCES.load(deps.storage, &key).unwrap_or_default(),
        RequestType::Burn => BURN_ALLOWANCES.load(deps.storage, &key).unwrap_or_default(),
    };

    to_binary(&res)
}

// Query to get request balance of
fn try_get_burn_balance_of(deps: Deps, owner: Addr) -> Result<QueryResponse, StdError> {
    let res = BURN_BALANCES.load(deps.storage, owner).unwrap_or_default();

    to_binary(&res)
}

// Query to get remaining balance of allowance
fn try_get_allowance(deps: Deps, owner: Addr, spender: Addr) -> Result<QueryResponse, StdError> {
    let rem_bal = get_allowance(deps.storage, owner, spender).unwrap_or_default();

    to_binary(&rem_bal)
}

// Query to get tokenization agent
fn try_get_tokenization_agent(deps: Deps) -> Result<QueryResponse, StdError> {
    let agent = TOKENIZATION_AGENT.load(deps.storage)?;

    to_binary(&agent)
}

fn try_get_dest_config(deps: Deps) -> Result<QueryResponse, StdError> {
    let conig = DEST_CONFIG.load(deps.storage)?;

    to_binary(&conig)
}
