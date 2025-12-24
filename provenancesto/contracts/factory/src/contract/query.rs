#[cfg(not(feature = "library"))]
use super::*;

/// Handle query requests for the provenance marker module.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
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
        QueryMsg::GetAllProposals {} => try_get_all_proposals(deps),
        QueryMsg::GetMintProposalInfo { proposal_id } => try_get_mint_proposal(deps, proposal_id),
        QueryMsg::GetBurnProposalInfo { proposal_id } => try_get_burn_proposal(deps, proposal_id),
        QueryMsg::GetFrozenTokens { denom } => try_get_frozen_tokens(deps, denom),
        QueryMsg::GetCiculatingSupply { denom } => try_get_circultating_tokens(deps, denom),
        QueryMsg::GetContractAddressByDenom { denom } => {
            to_binary(&get_contract_address(deps, denom)?)
        }
    }
}

// Query a marker by address.
fn try_get_marker_by_address(deps: Deps, address: String) -> Result<QueryResponse, StdError> {
    let address = deps.api.addr_validate(&address)?;
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker = querier.get_marker_by_address(address)?;
    to_binary(&marker)
}

// Query a marker by denom.
fn try_get_marker_by_denom(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker = querier.get_marker_by_denom(denom)?;
    to_binary(&marker)
}

// Query authorized countries.
fn try_get_auth_countries(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let denom_config = DENOM_CONFIG.load(deps.storage, denom.as_bytes())?;
    to_binary(&denom_config.country_codes)
}

// Query freezed accounts.
fn try_get_freezed_accounts(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let accounts = FREEZE_LIST.load(deps.storage, denom.as_bytes()).ok();
    to_binary(&accounts)
}

// Query frozen balances by address.
fn try_get_frozen_balance(
    deps: Deps,
    denom: String,
    address: Addr,
) -> Result<QueryResponse, StdError> {
    let key = Key::new(denom, address).as_bytes_std()?;
    let bal = PARTIAL_FREEZE
        .load(deps.storage, &key)
        .unwrap_or(Uint128::zero());
    to_binary(&bal)
}

// Query denom config by denom.
fn try_get_denom_config(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let denom_config = DENOM_CONFIG.load(deps.storage, denom.as_bytes())?;
    to_binary(&denom_config)
}

// Query authorized countries.
fn try_get_country_code_by_address(
    deps: Deps,
    denom: String,
    address: Addr,
) -> Result<QueryResponse, StdError> {
    let key = Key::new(denom, address).as_bytes_std()?;
    let code = WHITELIST.load(deps.storage, &key)?;
    to_binary(&code)
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

// Query authorized countries.
fn try_get_balance(deps: Deps, denom: String, address: Addr) -> Result<QueryResponse, StdError> {
    let balance = deps.querier.query_balance(address, denom)?;
    to_binary(&balance)
}

// Query Mint Proposal Info
fn try_get_mint_proposal(deps: Deps, proposal_id: u128) -> Result<QueryResponse, StdError> {
    let key = Key::new(RequestType::Mint, proposal_id).as_bytes_std()?;
    let proposals = REQUEST_INFO.load(deps.storage, &key)?;
    to_binary(&proposals)
}

// Query Burn Proposal Info
fn try_get_burn_proposal(deps: Deps, proposal_id: u128) -> Result<QueryResponse, StdError> {
    let key = Key::new(RequestType::Burn, proposal_id).as_bytes_std()?;
    let proposals = REQUEST_INFO.load(deps.storage, &key)?;
    to_binary(&proposals)
}

// Query All Proposals
fn try_get_all_proposals(deps: Deps) -> Result<QueryResponse, StdError> {
    let mut result = Vec::new();
    for data in REQUEST_INFO.range(deps.storage, None, None, Order::Ascending) {
        result.push(data.unwrap());
    }
    to_binary(&result)
}

// Query for frozen tokens
fn try_get_frozen_tokens(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let tokens = FROZEN_TOKENS
        .load(deps.storage, denom.as_bytes())
        .unwrap_or(Uint128::zero());
    to_binary(&tokens)
}

// Query for ciculating supply
fn try_get_circultating_tokens(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let circultating_tokens = MINTED_TOKENS
        .load(deps.storage, denom.as_bytes())
        .unwrap_or(Uint128::zero());

    to_binary(&circultating_tokens)
}
