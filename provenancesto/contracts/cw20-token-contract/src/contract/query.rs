#[cfg(not(feature = "library"))]
use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: Query) -> StdResult<Binary> {
    match msg {
        // inherited from cw20-base
        Query::TokenInfo {} => to_json_binary(&query_token_info(deps)?),
        Query::Balance { address } => to_json_binary(&query_balance(deps, address)?),
        Query::Allowance { owner, spender } => {
            to_json_binary(&query_allowance(deps, owner, spender)?)
        }
        Query::Minter {} => to_json_binary(&query_minter(deps)?),
        Query::GetFreezedAccounts {} => try_get_freezed_accounts(deps),
        Query::GetFrozenBalance { address } => try_get_frozen_balance(deps, address),
        Query::GetSubAdmins {} => try_get_sub_admins(deps),
        Query::GetAdmin {} => try_get_admin(deps),
        Query::GetFrozenTokens {} => try_get_frozen_tokens(deps),
        Query::GetCiculatingSupply {} => try_get_circultating_tokens(deps),
    }
}

// Query freezed accounts.
fn try_get_freezed_accounts(deps: Deps) -> Result<QueryResponse, StdError> {
    let accounts = FREEZE_LIST.load(deps.storage)?;
    to_json_binary(&accounts)
}

// Query frozen balances by address.
fn try_get_frozen_balance(deps: Deps, address: Addr) -> Result<QueryResponse, StdError> {
    let bal = PARTIAL_FREEZE
        .load(deps.storage, address.as_bytes())
        .unwrap_or(Uint128::zero());
    to_json_binary(&bal)
}

// Query for sub_admins.
fn try_get_sub_admins(deps: Deps) -> Result<QueryResponse, StdError> {
    let addresses = SUB_ADMIN.load(deps.storage)?;
    to_json_binary(&addresses)
}

// Query for admin.
fn try_get_admin(deps: Deps) -> Result<QueryResponse, StdError> {
    let address = ADMIN.load(deps.storage)?;
    to_json_binary(&address)
}

// Query for frozen tokens
fn try_get_frozen_tokens(deps: Deps) -> Result<QueryResponse, StdError> {
    let tokens = FROZEN_TOKENS.load(deps.storage).unwrap_or(Uint128::zero());
    to_json_binary(&tokens)
}

// Query for ciculating supply
fn try_get_circultating_tokens(deps: Deps) -> Result<QueryResponse, StdError> {
    let token_info = query_token_info(deps)?;
    let supply = token_info.total_supply;
    to_json_binary(&supply)
}
