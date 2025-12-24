#![cfg(not(feature = "library"))]
#[cfg(not(feature = "library"))]
use crate::{enums::*, error::ContractError, helper::*, msg::*, state::*, structs::*};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    attr, entry_point, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128,
};
#[cfg(not(feature = "library"))]
use provwasm_std::{ProvenanceMsg, ProvenanceQuerier, ProvenanceQuery};

/// Handle messages that create and interact with with native provenance markers.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    match msg {
        ExecuteMsg::Create { params } => try_create(deps, info, env.contract.address, params),
        ExecuteMsg::GrantAccess { denom, address } => {
            try_grant_access(deps, denom, env.contract.address, info.sender, address)
        }
        ExecuteMsg::Mint { amount, denom } => {
            try_mint(deps, info.sender, amount, denom, env.contract.address)
        }
        ExecuteMsg::Burn { amount, denom } => {
            try_burn(deps, info.sender, amount, denom, env.contract.address)
        }
        ExecuteMsg::Cancel { denom } => try_cancel(deps, denom, info.sender, env.contract.address),
        ExecuteMsg::Destroy { denom } => {
            try_destroy(deps, denom, info.sender, env.contract.address)
        }
        ExecuteMsg::Withdraw { amount, denom } => {
            try_withdraw(deps, info.sender, amount, denom, env.contract.address)
        }
        ExecuteMsg::Freeze { denom, update_type } => {
            try_update_freezelist(deps, info.sender, denom, update_type)
        }
        ExecuteMsg::PartialFreeze { denom, params } => {
            try_partial_freeze(deps, info.sender, denom, params)
        }
        ExecuteMsg::UpdateCountryCode { update_type, denom } => {
            try_update_country_code(deps, update_type, denom, info.sender)
        }
        ExecuteMsg::Send { amount, denom, to } => {
            try_send(deps, amount, denom, to, info.sender, env)
        }
        ExecuteMsg::UpdateTokenLimit { denom, limit } => {
            try_update_token_limit(deps, denom, limit, info.sender)
        }
        ExecuteMsg::Whitelist { lists } => try_update_whitelist(deps, lists, info.sender),
        ExecuteMsg::MintTo { mint_to_params } => {
            try_mint_to(deps, mint_to_params, info.sender, env.contract.address)
        }
        ExecuteMsg::BurnFrom { burn_from_params } => {
            try_burn_from(deps, burn_from_params, env.contract.address, info.sender)
        }
        ExecuteMsg::ManageRoles { denom, roles } => {
            try_manage_roles(deps, info, env.contract.address, denom, roles)
        }
        ExecuteMsg::ForceTransfer { denom, params } => {
            try_force_transfer(deps, denom, params, info.sender, env)
        }
        ExecuteMsg::DeliveryVsPayment { denom, mint_data } => {
            try_dvp(deps, denom, mint_data, info.sender, env.contract.address)
        }
    }
}

/// Create and dispatch a message that will create a new restricted marker w/ active status.
/// List of country codes will be stored for allowing transactions only from whitelisted countries.
/// Token limit will be set so that no user can hold tokens above this limit.
/// Issuer address will be assigned for mint, burn, force_transfer, freeze and unfreeze operations.
/// Tokenization Agent address will be assigned for mint and burn operations.
/// Transfer Agent address will be assigned for force_transfer, freeze and unfreeze operations.
///
/// Fails when:-
///     - caller is not sub_admin
///     - supply is lesser than token_limit
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.create
///     - marker_supply
///     - marker_denom
///     - denom_id
fn try_create(
    deps: DepsMut<ProvenanceQuery>,
    info: MessageInfo,
    contract_address: Addr,
    params: CreateMarkerParams,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, info.sender.clone())?;

    // storing denom configuration
    DENOM_CONFIG.save(
        deps.storage,
        params.denom.as_bytes(),
        &DenomConfig::new(params.denom_config),
    )?;

    // Storing timestamp for holding period
    if params.holding_period.u64() > 0 {
        HOLDING_PERIOD.save(
            deps.storage,
            params.denom.as_bytes(),
            &params.holding_period,
        )?;
    }

    // Giving access to Issuer, Transfer and Tokenization Agents
    try_manage_roles(
        deps,
        info,
        contract_address.clone(),
        params.denom.clone(),
        vec![
            Role::Issuer {
                update_type: UpdateType::Add(params.issuer),
            },
            Role::TransferAgent {
                update_type: UpdateType::Add(params.transfer_agent),
            },
            Role::TokenizationAgent {
                update_type: UpdateType::Add(params.tokenization_agent),
            },
        ],
    )?;

    let msgs = cm_create_marker(&params.denom, contract_address)?;

    let res = Response::<ProvenanceMsg>::new()
        .add_messages(msgs)
        .add_attribute("action", "provwasm.contracts.custom_marker.create")
        .add_attribute("marker_denom", params.denom)
        .add_attribute("denom_id", params.id);

    Ok(res)
}

// Create and dispatch a message that will grant all given permissions to a marker for an address.
fn try_grant_access(
    deps: DepsMut<ProvenanceQuery>,
    denom: String,
    contract_address: Addr,
    sender: Addr,
    address: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Only Subadmin can grant access
    is_subadmin(&deps, sender)?;

    let msg = cm_grant_marker_access(&denom, contract_address.clone(), all_access(&address))?;
    let res = Response::<ProvenanceMsg>::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.grant_access")
        .add_attribute("denom", denom)
        .add_attribute("address", address);

    Ok(res)
}

/// Create and dispatch a message that will withdraw coins from a marker to contract address.
///
/// Fails when:-
///     - caller is not sub_admin
///     - amount is zero
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.withdraw
///     - withdraw_amount
///     - withdraw_denom
///     - withdraw_recipient
fn try_withdraw(
    deps: DepsMut<ProvenanceQuery>,
    sender: Addr,
    amount: Uint128,
    denom: String,
    recipient: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let msg = cm_withdraw_coins(&denom, amount.u128(), recipient.clone(), recipient.clone())?;

    let res = Response::<ProvenanceMsg>::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.custom_marker.withdraw")
        .add_attribute("withdraw_amount", amount)
        .add_attribute("withdraw_denom", denom)
        .add_attribute("withdraw_recipient", recipient);

    Ok(res)
}

/// Create and dispatch a message that will mint coins into a marker, hence increases supply.
///
/// Fails when:-
///     - caller is not sub_admin
///     - amount is zero
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.mint
///     - mint_amount
///     - mint_denom
fn try_mint(
    deps: DepsMut<ProvenanceQuery>,
    sender: Addr,
    amount: Uint128,
    denom: String,
    contract_address: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let msg = cm_mint_marker_supply(amount.u128(), &denom, contract_address)?;

    let res = Response::<ProvenanceMsg>::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.custom_marker.mint")
        .add_attribute("mint_amount", amount)
        .add_attribute("mint_denom", denom);

    Ok(res)
}

/// Create and dispatch a message that will burn coins from a marker, hence decreases supply.
///
/// Fails when:-
///     - caller is not sub_admin
///     - amount is zero
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.burn
///     - burn_amount
///     - burn_denom
fn try_burn(
    deps: DepsMut<ProvenanceQuery>,
    sender: Addr,
    amount: Uint128,
    denom: String,
    contract_address: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let msg = cm_burn_marker_supply(amount.u128(), &denom, contract_address)?;

    let res = Response::<ProvenanceMsg>::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.custom_marker.burn")
        .add_attribute("burn_amount", amount)
        .add_attribute("burn_denom", denom);

    Ok(res)
}

/// Create and dispatch a message that will cancel a marker.
/// Move marker to a cancelled state, i.e. `MarkerStatus::Cancelled`
///
/// Fails when:-
///     - caller does not have delete access
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.cancel
///     - marker_denom
fn try_cancel(
    deps: DepsMut<ProvenanceQuery>,
    denom: String,
    sender: Addr,
    contract_address: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Checking caller has delete access
    has_delete_access(&deps, denom.clone(), sender)?;

    let msg = cm_cancel_marker(&denom, contract_address)?;

    let res = Response::<ProvenanceMsg>::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.custom_marker.cancel")
        .add_attribute("marker_denom", denom);

    Ok(res)
}

/// Create and dispatch a message that will destroy a marker.
/// Move marker to a destroyed state, i.e. `MarkerStatus::Destroyed`
///
/// Fails when:-
///     - caller does not have delete access
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.destroy
///     - marker_denom
fn try_destroy(
    deps: DepsMut<ProvenanceQuery>,
    denom: String,
    sender: Addr,
    contract_address: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Checking caller has delete access
    has_delete_access(&deps, denom.clone(), sender)?;

    let msg = cm_destroy_marker(denom.clone(), contract_address)?;

    let res = Response::<ProvenanceMsg>::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.custom_marker.destroy")
        .add_attribute("marker_denom", denom);

    Ok(res)
}

/// Update Freeze List.
/// This function is used to freeze and unfreeze account.
/// If the account is freezed than it will not able to perform any transactions.
/// For freeze `update_type` will be `UpdateType::Add(Vec<Addr>)`
/// For unfreeze `update_type` will be `UpdateType::Remove(Vec<Addr>)`
/// This function supports batch operations, i.e. multiple addresses can be freezed / unfreezed simultaneously.
///
/// Fails when:-
///     - caller is not issuer, transfer_agent, sub_admin or having freeze access for freezing
///     - caller is not issuer, transfer_agent, sub_admin or having unfreeze access for unfreezing
///     - freeze list doesn't contain the address that is going to be removed
/// Emits event:-
///     - when update_type = UpdateType::Add(Vec<Addr>), then
///         1. provwasm.contracts.custom_marker.freeze
///         2. addresses
///     - when update_type = UpdateType::Remove(Vec<Addr>), then
///         1. provwasm.contracts.custom_marker.unfreeze
///         2. addresses
fn try_update_freezelist(
    deps: DepsMut<ProvenanceQuery>,
    sender: Addr,
    denom: String,
    update_type: UpdateType<Vec<Addr>>,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // ensure not freezed
    ensure_not_freezed(deps.storage, vec![sender.clone()], denom.as_bytes())?;

    let attrs = match update_type {
        UpdateType::Add(addrs) => {
            // Ensuring authorized sender
            if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
                && is_transfer_agent(&deps, denom.clone(), sender.clone()).is_err()
                && has_freeze_access(&deps, denom.clone(), sender.clone()).is_err()
                && is_subadmin(&deps, sender.clone()).is_err()
            {
                let err = format!(
                    "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Freeze rights!",
                    &sender
                );
                return Err(ContractError::Unauthorized { err });
            }

            // Adding addresses into Freeze List
            FREEZE_LIST.update(
                deps.storage,
                denom.as_bytes(),
                |addresses_opt: Option<Vec<Addr>>| -> Result<_, ContractError> {
                    match addresses_opt {
                        Some(mut addresses) => Ok({
                            addresses.extend(addrs.clone());
                            addresses.sort();
                            addresses.dedup();
                            addresses
                        }),
                        None => Ok(addrs.clone()),
                    }
                },
            )?;
            vec![
                attr("action", "provwasm.contracts.custom_marker.freeze"),
                attr("addresses", format!("{:?}", addrs)),
            ]
        }
        UpdateType::Remove(addrs) => {
            // Ensuring authorized sender
            if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
                && is_transfer_agent(&deps, denom.clone(), sender.clone()).is_err()
                && has_unfreeze_access(&deps, denom.clone(), sender.clone()).is_err()
                && is_subadmin(&deps, sender.clone()).is_err()
            {
                let err = format!(
                    "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Unfreeze rights!",
                    &sender
                );
                return Err(ContractError::Unauthorized { err });
            }

            // Removing addresses from Freeze List
            FREEZE_LIST.update(
                deps.storage,
                denom.as_bytes(),
                |addresses_opt: Option<Vec<Addr>>| -> Result<_, ContractError> {
                    match addresses_opt {
                        Some(mut addresses) => Ok({
                            addresses.retain(|addr| !addrs.contains(addr));
                            addresses
                        }),
                        None => Err(ContractError::CanNotRemove {
                            address: addrs[0].clone(),
                        }),
                    }
                },
            )?;
            vec![
                attr("action", "provwasm.contracts.custom_marker.unfreeze"),
                attr("addresses", format!("{:?}", addrs)),
            ]
        }
    };

    let res = Response::<ProvenanceMsg>::new().add_attributes(attrs);

    Ok(res)
}

/// Update whitelist.
/// This function is used to whitelist address based on country_code.
/// Both set and unset whitelist can be performed by this single entry function.
/// For set `update_kind` will be `UpdateKind::Set(u8)`
/// For unset `update_kind` will be `UpdateKind::Unset(u8)`
/// This function supports batch operations, i.e. multiple addresses can be set / unset from whitelist as well as
/// multiple denoms simultaneously.
///
/// Fails when:-
///     - caller is not sub_admin or tokenization agent
///     - the list already contains the address for addition
///     - the list doesn't have the address for removal
///
/// Emits event:-
///     - when update_kind = UpdateKind::Set(u8), then
///         1. provwasm.contracts.custom_marker.whitelist.set
///         2. address
///     - when update_kind = UpdateKind::Unset(u8), then
///         1. provwasm.contracts.custom_marker.whitelist.unset
///         2. address
fn try_update_whitelist(
    deps: DepsMut<ProvenanceQuery>,
    lists: Vec<WhiteListParams>,
    sender: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let mut attrs = Vec::new();
    for list in lists {
        // Ensuring authorized sender
        if is_subadmin(&deps, sender.clone()).is_err()
            && is_tokenization_agent(&deps, list.denom.clone(), sender.clone()).is_err()
        {
            let err = format!(
                "Address `{}`: Don't have Tokenization and Sub Admin rights!",
                &sender
            );
            return Err(ContractError::Unauthorized { err });
        }

        for cd_data in list.data {
            let key = Key::new(list.denom.clone(), cd_data.address.clone()).as_bytes()?;

            match cd_data.country_code {
                UpdateKind::Set(code) => {
                    if WHITELIST.has(deps.storage, &key) {
                        return Err(ContractError::CountryCodeAlreadyExists { code });
                    }
                    WHITELIST.save(deps.storage, &key, &code)?;
                    attrs.push(attr(
                        "action",
                        "provwasm.contracts.custom_marker.whitelist.set",
                    ));
                    attrs.push(attr("address", &cd_data.address));
                }
                UpdateKind::Unset {} => {
                    if !WHITELIST.has(deps.storage, &key) {
                        return Err(ContractError::NotFound {
                            addr: cd_data.address,
                        });
                    }
                    WHITELIST.remove(deps.storage, &key);
                    attrs.push(attr(
                        "action",
                        "provwasm.contracts.custom_marker.whitelist.unset",
                    ));
                    attrs.push(attr("address", &cd_data.address));
                }
            }
        }
    }

    let res = Response::<ProvenanceMsg>::new().add_attributes(attrs);

    Ok(res)
}

/// Function to freeze / unfreeze partial balance of users.
/// Both partial freeze and partial unfreeze can be performed by this single entry function.
/// For partial freeze `update_type` will be `UpdateType::Add(Uint128)`
/// For partial unfreeze  `update_type` will be `UpdateType::Remove(Uint128)`
/// This function supports batch operations, i.e. multiple addresses can be partially freezed / unfreezed
/// simultaneously.
/// When the tokens are partially freezed then entire amount can't be used in any transactions, only unfreezed tokens
/// can take part in the transactions.
///  
/// Fails when:-
///     - caller is not sub_admin, issuer, transfer agent or having freeze access for partial freeze
///     - caller is not sub_admin, issuer, transfer agent or having unfreeze access for partial unfreeze
///     - the partial freeze list dosen't have the address for removal
///     - the frozen token list is empty
///
/// Emits event:-
///     - when update_type = UpdateType::Add(Uint128), then
///         1. provwasm.contracts.custom_marker.partial_freeze
///         2. address
///     - when update_type = UpdateType::Remove(Uint128), then
///         1. provwasm.contracts.custom_marker.partial_unfreeze
///         2. address
fn try_partial_freeze(
    deps: DepsMut<ProvenanceQuery>,
    sender: Addr,
    denom: String,
    params: Vec<PartialFreezeParams>,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let mut attrs = Vec::new();
    let mut rem_bal = Uint128::zero();
    for param in params {
        let key = Key::new(denom.clone(), param.address.clone()).as_bytes()?;

        match param.update_type {
            UpdateType::Add(bal) => {
                // Ensuring authorized sender
                if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
                    && is_transfer_agent(&deps, denom.clone(), sender.clone()).is_err()
                    && has_freeze_access(&deps, denom.clone(), sender.clone()).is_err()
                    && is_subadmin(&deps, sender.clone()).is_err()
                {
                    let err = format!(
                        "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Freeze rights!",
                        &sender
                    );
                    return Err(ContractError::Unauthorized { err });
                }

                // Updating balance for the account that already exists and adding the address and balance pairs that
                // do not exist
                if PARTIAL_FREEZE
                    .update(deps.storage, &key, |bals_opt| -> Result<_, ContractError> {
                        match bals_opt {
                            Some(mut bals) => Ok({
                                bals += bal;
                                bals
                            }),
                            None => Ok(bal),
                        }
                    })
                    .is_err()
                {
                    // Adding address and balance pair that doesn't exist
                    PARTIAL_FREEZE.save(deps.storage, &key, &bal)?;
                }

                // Updating frozen tokens balance
                if FROZEN_TOKENS
                    .update(
                        deps.storage,
                        denom.as_bytes(),
                        |bals_opt| -> Result<_, ContractError> {
                            match bals_opt {
                                Some(mut bals) => Ok({
                                    bals += bal;
                                    bals
                                }),
                                None => Ok(bal),
                            }
                        },
                    )
                    .is_err()
                {
                    // Adding balance as new entry
                    FROZEN_TOKENS.save(deps.storage, denom.as_bytes(), &bal)?;
                }
                attrs.push(attr(
                    "action",
                    "provwasm.contracts.custom_marker.partial_freeze",
                ));
                attrs.push(attr("address", &param.address));
            }
            UpdateType::Remove(bal) => {
                // Ensuring authorized sender
                if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
                    && is_transfer_agent(&deps, denom.clone(), sender.clone()).is_err()
                    && has_unfreeze_access(&deps, denom.clone(), sender.clone()).is_err()
                    && is_subadmin(&deps, sender.clone()).is_err()
                {
                    let err = format!(
                        "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Freeze rights!",
                        &sender
                    );
                    return Err(ContractError::Unauthorized { err });
                }

                // Removed from frozen tokens that are unfreezed
                FROZEN_TOKENS.update(
                    deps.storage,
                    denom.as_bytes(),
                    |bals_opt| -> Result<_, ContractError> {
                        match bals_opt {
                            Some(mut bals) => Ok({
                                bals -= bal;
                                bals
                            }),
                            None => Err(ContractError::NotFound {
                                addr: param.address.clone(),
                            }),
                        }
                    },
                )?;

                // Reducing partial freeze balance
                PARTIAL_FREEZE.update(
                    deps.storage,
                    &key,
                    |bals_opt| -> Result<_, ContractError> {
                        match bals_opt {
                            Some(mut bals) => Ok({
                                bals -= bal;
                                rem_bal = bals;
                                bals
                            }),
                            None => Err(ContractError::NotFound {
                                addr: param.address.clone(),
                            }),
                        }
                    },
                )?;

                // Removing the address and balance pair from the partial freeze list, if amount becomes zero
                if rem_bal.is_zero() {
                    PARTIAL_FREEZE.remove(deps.storage, &key);
                }
                attrs.push(attr(
                    "action",
                    "provwasm.contracts.custom_marker.partial_unfreeze",
                ));
                attrs.push(attr("address", &param.address));
            }
        }
    }

    let res = Response::<ProvenanceMsg>::new().add_attributes(attrs);

    Ok(res)
}

/// Update country code.
/// This list contains set of country codes that are authorised to transact on this platform.
/// Both add and remove can be performed by this single entry function.
/// For addition `update_type` will be `UpdateType::Add(u8)`
/// For removal `update_type` will be `UpdateType::Remove(u8)`
///  
/// Fails when:-
///     - caller is not sub_admin
///     - denom config (also known as token config) is not available for particular denom
///
/// Emits event:-
///     - when update_type = UpdateType::Add(u8), then
///         1. provwasm.contracts.custom_marker.add_country_code
///         2. country_code
///     - when update_type = UpdateType::Remove(u8), then
///         1. provwasm.contracts.custom_marker.remove_country_code
///         2. country_code
fn try_update_country_code(
    deps: DepsMut<ProvenanceQuery>,
    update_type: UpdateType<u8>,
    denom: String,
    sender: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Only Subadmin update country code
    is_subadmin(&deps, sender)?;

    let attrs = match update_type {
        UpdateType::Add(code) => {
            // Adding new country code to the list
            DENOM_CONFIG.update(
                deps.storage,
                denom.clone().as_bytes(),
                |denom_config_opt: Option<DenomConfig>| -> Result<_, ContractError> {
                    match denom_config_opt {
                        Some(mut denom_config) => {
                            add_country_codes(&mut denom_config.country_codes, code)?;
                            Ok(denom_config)
                        }
                        None => Err(ContractError::MissingDenomConfig { denom }),
                    }
                },
            )?;

            vec![
                attr(
                    "action",
                    "provwasm.contracts.custom_marker.add_country_code",
                ),
                attr("country_code", code.to_string()),
            ]
        }
        UpdateType::Remove(code) => {
            // Removing country code from the list
            DENOM_CONFIG.update(
                deps.storage,
                denom.clone().as_bytes(),
                |denom_config_opt: Option<DenomConfig>| -> Result<_, ContractError> {
                    match denom_config_opt {
                        Some(mut denom_config) => {
                            remove_country_codes(&mut denom_config.country_codes, code)?;
                            Ok(denom_config)
                        }
                        None => Err(ContractError::MissingDenomConfig { denom }),
                    }
                },
            )?;

            vec![
                attr(
                    "action",
                    "provwasm.contracts.custom_marker.remove_country_code",
                ),
                attr("country_code", code.to_string()),
            ]
        }
    };

    let res = Response::<ProvenanceMsg>::new().add_attributes(attrs);

    Ok(res)
}

/// Update token limit.
/// This list contains token limit, i.e., the maximum permissible token that an account can hold.
///  
/// Fails when:-
///     - caller is not sub_admin
///     - denom config (also known as token config) is not available for particular denom
///
/// Emits event:-
///     - update_token_limit
///     - new_limit
fn try_update_token_limit(
    deps: DepsMut<ProvenanceQuery>,
    denom: String,
    limit: Uint128,
    sender: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Only Subadmin update token limit
    is_subadmin(&deps, sender)?;

    // Setting new token_limit
    DENOM_CONFIG.update(
        deps.storage,
        denom.clone().as_bytes(),
        |denom_config_opt: Option<DenomConfig>| -> Result<_, ContractError> {
            match denom_config_opt {
                Some(mut denom_config) => {
                    denom_config.token_limit = limit;
                    Ok(denom_config)
                }
                None => Err(ContractError::MissingDenomConfig { denom }),
            }
        },
    )?;

    let res = Response::<ProvenanceMsg>::new()
        .add_attribute(
            "action",
            "provwasm.contracts.custom_marker.update_token_limit",
        )
        .add_attribute("new_limit", limit);

    Ok(res)
}

/// Create and dispatch a message that will send coins from one account to another.
/// Caller is considered as the owner of token.
///
/// Fails when:-
///     - sender doesn't have transfer access
///     - recipient is not whitelisted
///     - recipient is freezed
///     - amount is zero
///     - amount exceeds the alloted token_limit
///     - unfreezed balance is lesser than requested amount
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.send
///     - funds
///     - to
///     - from
fn try_send(
    deps: DepsMut<ProvenanceQuery>,
    amount: Uint128,
    denom: String,
    to: Addr,
    from: Addr,
    env: Env,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Ensuring authorized sender
    has_transfer_access(&deps, denom.clone(), from.clone())?;

    // Ensuring holding period is passed
    ensure_holding_period_passed(deps.storage, denom.as_bytes(), env.block.time.seconds())?;

    if env.contract.address.ne(&to) {
        // ensuring country is authorized
        ensure_authorized_country(deps.storage, denom.clone(), to.clone())?;
    }

    // ensure not freezed
    ensure_not_freezed(deps.storage, vec![to.clone()], denom.as_bytes())?;

    // ensure frozen balance and balance capital maintained
    ensure_bal_maintained(
        deps.as_ref(),
        to.clone(),
        from.clone(),
        denom.clone(),
        amount,
    )?;

    let transfer = cm_transfer_marker_coins(
        amount.u128(),
        &denom,
        to.clone(),
        from.clone(),
        env.contract.address,
    )?;

    let res = Response::<ProvenanceMsg>::new()
        .add_message(transfer)
        .add_attribute("action", "provwasm.contracts.custom_marker.send")
        .add_attribute("funds", format!("{}{}", &amount, &denom))
        .add_attribute("to", to)
        .add_attribute("from", from);

    Ok(res)
}

/// Create and dispatch a message that will mint coins into address.
/// This function supports batch operations, i.e. multiple addresses can be minted simultaneously.
///  
/// Fails when:-
///     - caller is not sub_admin, issuer, tokenization agent or having mint access
///     - recipient is not whitelisted
///     - recipient is freezed
///     - amount is zero
///     - amount exceeds the alloted token_limit
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.mint_to
fn try_mint_to(
    deps: DepsMut<ProvenanceQuery>,
    mint_to_params: Vec<MintBurnParams>,
    sender: Addr,
    contract_address: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let mut msgs: Vec<CosmosMsg<ProvenanceMsg>> = Vec::new();

    for params in mint_to_params {
        // Ensuring authorized sender
        if is_issuer(&deps, params.denom.clone(), sender.clone()).is_err()
            && is_tokenization_agent(&deps, params.denom.clone(), sender.clone()).is_err()
            && has_mint_access(&deps, params.denom.clone(), sender.clone()).is_err()
            && is_subadmin(&deps, sender.clone()).is_err()
        {
            let err = format!(
                "Address `{}`: Don't have Issuer, Tokenization, Sub Admin or Mint rights!",
                &sender
            );
            return Err(ContractError::Unauthorized { err });
        }

        for mint_data in params.mint_burn_data {
            // ensure token limit maintained.
            ensure_token_limit(
                deps.as_ref(),
                mint_data.address.clone(),
                params.denom.clone(),
                mint_data.amount,
            )?;

            // ensure not freezed
            ensure_not_freezed(
                deps.storage,
                vec![mint_data.address.clone()],
                params.denom.as_bytes(),
            )?;

            // ensuring country is authorized
            ensure_authorized_country(
                deps.storage,
                params.denom.clone(),
                mint_data.address.clone(),
            )?;

            // Add mint amount to pool
            update_minted_tokens(
                deps.storage,
                params.denom.clone(),
                UpdateType::Add(mint_data.amount),
            )?;

            msgs.extend(mint_to(
                params.denom.to_string(),
                mint_data,
                contract_address.clone(),
            )?);
        }
    }

    let res = Response::<ProvenanceMsg>::new()
        .add_messages(msgs)
        .add_attribute("action", "provwasm.contracts.custom_marker.mint_to");

    Ok(res)
}

/// Create and dispatch a message that will mint coins into address and freeze that amount.
/// This function supports batch operations, i.e. multiple addresses can be minted simultaneously.
///  
/// Fails when:-
///     - caller is not sub_admin, issuer, tokenization agent or having mint access
///     - recipient is not whitelisted
///     - recipient is freezed
///     - amount is zero
///     - amount exceeds the alloted token_limit
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.delivery_vs_payment
fn try_dvp(
    deps: DepsMut<ProvenanceQuery>,
    denom: String,
    mint_data: Vec<MintBurnData>,
    sender: Addr,
    contract_address: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let mut msgs: Vec<CosmosMsg<ProvenanceMsg>> = Vec::new();

    // Ensuring authorized sender
    if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
        && (has_mint_access(&deps, denom.clone(), sender.clone()).is_err()
            || has_freeze_access(&deps, denom.clone(), sender.clone()).is_err())
        && is_subadmin(&deps, sender.clone()).is_err()
    {
        let err = format!(
            "Address `{}`: Don't have Issuer, Sub Admin, Freeze or Mint rights!",
            &sender
        );
        return Err(ContractError::Unauthorized { err });
    }

    for data in mint_data {
        // ensure token limit maintained.
        ensure_token_limit(
            deps.as_ref(),
            data.address.clone(),
            denom.clone(),
            data.amount,
        )?;

        // ensure not freezed
        ensure_not_freezed(deps.storage, vec![data.address.clone()], denom.as_bytes())?;

        // ensuring country is authorized
        ensure_authorized_country(deps.storage, denom.clone(), data.address.clone())?;

        // Add mint amount to pool
        update_minted_tokens(deps.storage, denom.clone(), UpdateType::Add(data.amount))?;

        msgs.extend(mint_to(
            denom.to_string(),
            data.clone(),
            contract_address.clone(),
        )?);

        let key = Key::new(denom.clone(), data.address.clone()).as_bytes()?;

        // Updating balance for the account that already exists and adding the address and balance pairs that
        // do not exist
        if PARTIAL_FREEZE
            .update(deps.storage, &key, |bals_opt| -> Result<_, ContractError> {
                match bals_opt {
                    Some(mut bals) => Ok({
                        bals += data.amount;
                        bals
                    }),
                    None => Ok(data.amount),
                }
            })
            .is_err()
        {
            // Adding address and balance pair that doesn't exist
            PARTIAL_FREEZE.save(deps.storage, &key, &data.amount)?;
        }

        // Updating frozen tokens balance
        if FROZEN_TOKENS
            .update(
                deps.storage,
                denom.as_bytes(),
                |bals_opt| -> Result<_, ContractError> {
                    match bals_opt {
                        Some(mut bals) => Ok({
                            bals += data.amount;
                            bals
                        }),
                        None => Ok(data.amount),
                    }
                },
            )
            .is_err()
        {
            // Adding balance as new entry
            FROZEN_TOKENS.save(deps.storage, denom.as_bytes(), &data.amount)?;
        }
    }

    let res = Response::<ProvenanceMsg>::new()
        .add_messages(msgs)
        .add_attribute(
            "action",
            "provwasm.contracts.custom_marker.delivery_vs_payment",
        );

    Ok(res)
}

/// Create and dispatch a message that will burn coins from address.
/// This function supports batch operations, i.e. multiple addresses can be burned simultaneously.
/// Decreases minted token value.
///  
/// Fails when:-
///     - caller is not sub_admin, issuer, tokenization agent or having burn access
///     - recipient is not whitelisted
///     - recipient is freezed
///     - amount is zero
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.burn_from
fn try_burn_from(
    deps: DepsMut<ProvenanceQuery>,
    burn_from_params: Vec<MintBurnParams>,
    contract_address: Addr,
    sender: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let mut msgs: Vec<CosmosMsg<ProvenanceMsg>> = Vec::new();
    let querier = ProvenanceQuerier::new(&deps.querier);

    for params in burn_from_params {
        // Ensuring authorized sender
        if is_issuer(&deps, params.denom.clone(), sender.clone()).is_err()
            && is_tokenization_agent(&deps, params.denom.clone(), sender.clone()).is_err()
            && has_burn_access(&deps, params.denom.clone(), sender.clone()).is_err()
            && is_subadmin(&deps, sender.clone()).is_err()
        {
            let err = format!(
                "Address `{}`: Don't have Issuer, Tokenization, Sub Admin or Burn rights!",
                &sender
            );
            return Err(ContractError::Unauthorized { err });
        }

        for burn_data in params.mint_burn_data {
            // ensure token balance no frozen.
            ensure_bal_not_frozen(
                deps.as_ref(),
                burn_data.address.clone(),
                params.denom.clone(),
            )?;

            // ensuring country is authorized
            ensure_authorized_country(
                deps.storage,
                params.denom.clone(),
                burn_data.address.clone(),
            )?;

            // Remove mint amount to pool
            update_minted_tokens(
                deps.storage,
                params.denom.clone(),
                UpdateType::Remove(burn_data.amount),
            )?;

            msgs.extend(burn_from(
                params.denom.clone(),
                burn_data,
                contract_address.clone(),
                &querier,
            )?);
        }
    }

    let res = Response::<ProvenanceMsg>::new()
        .add_messages(msgs)
        .add_attribute("action", "provwasm.contracts.custom_marker.burn_from");

    Ok(res)
}

/// Function to manage different roles
/// This function can perform batch operations, hence multiple addresses can be added or removed simultaneously.
/// This entry point can be use to modify different roles such as:-
///     - Issuer
///     - Transfer Agent
///     - Tokenization Agent
///     - Sub Admin
///     - Admin
///     - Agents
/// Based on the `update_type` field addresses can be added or removed:-
///     - for addition `update_type` will be `UpdateType::Add(Addresses)`
///     - for removal `update_type` will be `UpdateType::Remove(Addresses)`
///
/// Fails when:-
///     - caller is not sub_admin, in case of Issuer, Transfer Agent, Tokenization Agents and Agents
///     - caller is not admin, in case of Admin and Sub Admin
///     - any map is empty during remove call
///
/// Based on operation, any event can be emitted:-
///     - provwasm.contracts.custom_marker.add_issuer
///     - provwasm.contracts.custom_marker.remove_issuer
///     - provwasm.contracts.custom_marker.add_transfer_agent
///     - provwasm.contracts.custom_marker.remove_transfer_agent
///     - provwasm.contracts.custom_marker.add_tokenization_agent
///     - provwasm.contracts.custom_marker.remove_tokenization_agent
///     - provwasm.contracts.custom_marker.add_sub_admin
///     - provwasm.contracts.custom_marker.remove_sub_admin
///     - provwasm.contracts.custom_marker.update_admin
///     - provwasm.contracts.custom_marker.grant_access
///     - provwasm.contracts.custom_marker.ungrant_access
pub fn try_manage_roles(
    deps: DepsMut<ProvenanceQuery>,
    info: MessageInfo,
    contract_address: Addr,
    denom: String,
    roles: Vec<Role>,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let mut attrs = Vec::new();
    let mut msgs: Vec<CosmosMsg<ProvenanceMsg>> = Vec::new();

    for role in roles {
        match role {
            Role::Issuer { update_type } => {
                // Only Subadmin can add/remove Issuer
                is_subadmin(&deps, info.sender.clone())?;

                match update_type {
                    UpdateType::Add(addr) => {
                        let key = Key::new(denom.clone(), addr.clone()).as_bytes()?;
                        if ISSUER.has(deps.storage, &key) {
                            return Err(ContractError::AlreadyAdded { addr });
                        }
                        ISSUER.save(deps.storage, &key, &AccessControls::issuer_rights())?;
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.add_issuer",
                        ));
                    }
                    UpdateType::Remove(addr) => {
                        let key = Key::new(denom.clone(), addr.clone()).as_bytes()?;
                        if !ISSUER.has(deps.storage, &key) {
                            return Err(ContractError::NotFound { addr });
                        }
                        ISSUER.remove(deps.storage, &key);
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.remove_issuer",
                        ));
                    }
                }
            }
            Role::TransferAgent { update_type } => {
                // Only Subadmin can add/remove Transfer Agent
                is_subadmin(&deps, info.sender.clone())?;

                match update_type {
                    UpdateType::Add(addr) => {
                        let key = Key::new(denom.clone(), addr.clone()).as_bytes()?;
                        if TRANSFER_AGENT.has(deps.storage, &key) {
                            return Err(ContractError::AlreadyAdded { addr });
                        }
                        TRANSFER_AGENT.save(
                            deps.storage,
                            &key,
                            &AccessControls::transfer_agent_rights(),
                        )?;
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.add_transfer_agent",
                        ));
                    }
                    UpdateType::Remove(addr) => {
                        let key = Key::new(denom.clone(), addr.clone()).as_bytes()?;
                        if !TRANSFER_AGENT.has(deps.storage, &key) {
                            return Err(ContractError::NotFound { addr });
                        }
                        TRANSFER_AGENT.remove(deps.storage, &key);
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.remove_transfer_agent",
                        ));
                    }
                }
            }
            Role::TokenizationAgent { update_type } => {
                // Only Subadmin can add/remove Tokenization Agent
                is_subadmin(&deps, info.sender.clone())?;

                match update_type {
                    UpdateType::Add(addr) => {
                        let key = Key::new(denom.clone(), addr.clone()).as_bytes()?;
                        if TOKENIZATION_AGENT.has(deps.storage, &key) {
                            return Err(ContractError::AlreadyAdded { addr });
                        }
                        TOKENIZATION_AGENT.save(
                            deps.storage,
                            &key,
                            &AccessControls::tokenization_agent_rights(),
                        )?;
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.add_tokenization_agent",
                        ));
                    }
                    UpdateType::Remove(addr) => {
                        let key = Key::new(denom.clone(), addr.clone()).as_bytes()?;
                        if !TOKENIZATION_AGENT.has(deps.storage, &key) {
                            return Err(ContractError::NotFound { addr });
                        }
                        TOKENIZATION_AGENT.remove(deps.storage, &key);
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.remove_tokenization_agent",
                        ));
                    }
                }
            }
            Role::SubAdmin { update_type } => {
                // Ensuring caller has the admin rights
                is_admin(&deps, info.sender.clone())?;

                match update_type {
                    UpdateType::Add(addrs) => {
                        let updated = SUB_ADMIN.update(
                            deps.storage,
                            |mut addresses| -> Result<_, ContractError> {
                                addresses.extend(addrs.clone());
                                addresses.sort();
                                addresses.dedup();

                                // Grant marker accesses
                                msgs.push(cm_grant_marker_access(
                                    denom.clone(),
                                    contract_address.clone(),
                                    all_access_to_addresses(&addresses, true),
                                )?);

                                Ok(addresses)
                            },
                        );
                        if updated.is_err() {
                            SUB_ADMIN.save(deps.storage, &addrs)?;
                        };
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.add_sub_admins",
                        ));
                    }
                    UpdateType::Remove(addrs) => {
                        SUB_ADMIN.update(
                            deps.storage,
                            |mut addresses| -> Result<_, ContractError> {
                                addresses.retain(|addr| !addrs.contains(addr));

                                // Revoke marker accesses
                                msgs.push(cm_grant_marker_access(
                                    denom.clone(),
                                    contract_address.clone(),
                                    all_access_to_addresses(&addresses, false),
                                )?);

                                Ok(addresses)
                            },
                        )?;
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.remove_sub_admins",
                        ));
                    }
                }
            }
            Role::Admin { address } => {
                // Ensuring caller has the admin rights
                is_admin(&deps, info.sender.clone())?;

                ADMIN.save(deps.storage, &address)?;
                attrs.push(attr(
                    "action",
                    "provwasm.contracts.custom_marker.update_admin",
                ));
            }
            Role::Agent {
                update_type,
                marker_access,
            } => {
                // Only Subadmin can grant/ungrant Agent Access
                is_subadmin(&deps, info.sender.clone())?;

                match update_type {
                    UpdateType::Add(to) => {
                        for access in marker_access {
                            match access {
                                AccessControls::Admin => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Admin,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Mint => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Mint,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Burn => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Burn,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Delete => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Delete,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Deposit => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Deposit,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Transfer => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Transfer,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Unspecified => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Unspecified,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Withdraw => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Withdraw,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }

                                AccessControls::Freeze => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Freeze,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Unfreeze => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Unfreeze,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::ForceTransfer => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::ForceTransfer,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                            }
                        }
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.grant_access",
                        ));
                    }
                    UpdateType::Remove(to) => {
                        for access in marker_access {
                            match access {
                                AccessControls::Admin => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Admin,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Mint => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Mint,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Burn => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Burn,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Delete => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Delete,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Deposit => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Deposit,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Transfer => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Transfer,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Unspecified => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Unspecified,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Withdraw => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Withdraw,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Freeze => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Freeze,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Unfreeze => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::Unfreeze,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::ForceTransfer => {
                                    manage_agent_access(
                                        deps.storage,
                                        denom.clone(),
                                        AccessControls::ForceTransfer,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                            }
                        }
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.ungrant_access",
                        ));
                    }
                }
            }
        }
    }

    Ok(Response::<ProvenanceMsg>::new()
        .add_messages(msgs)
        .add_attributes(attrs))
}

/// Create and dispatch a message that will force transfer coins from one account to another.
/// This function supports batch operations, i.e. multiple amounts can be transffered from accounts simultaneously.
///  
/// Fails when:-
///     - caller is not sub_admin, issuer, transfer agent or having force_transfer access
///     - recipient is not whitelisted
///     - caller is not whitelisted
///     - recipient is freezed
///     - caller is freezed
///     - amount is zero
///     - unfreezed balance is lesser than requested amount
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.force_transfer
///     - funds
///     - to
///     - from
fn try_force_transfer(
    deps: DepsMut<ProvenanceQuery>,
    denom: String,
    params: Vec<ForceTransferParams>,
    sender: Addr,
    env: Env,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Ensuring authorized sender
    if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
        && is_transfer_agent(&deps, denom.clone(), sender.clone()).is_err()
        && has_force_transfer_access(&deps, denom.clone(), sender.clone()).is_err()
        && is_subadmin(&deps, sender.clone()).is_err()
    {
        let err = format!(
            "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Force Transfer rights!",
            &sender
        );
        return Err(ContractError::Unauthorized { err });
    }

    // Ensuring holding period is passed
    ensure_holding_period_passed(deps.storage, denom.as_bytes(), env.block.time.seconds())?;

    let mut msgs = Vec::new();
    let mut attrs = Vec::new();
    attrs.push(attr(
        "action",
        "provwasm.contracts.custom_marker.force_transfer",
    ));

    for param in params {
        // ensuring country is authorized
        ensure_authorized_country(deps.storage, denom.clone(), param.from.clone())?;
        ensure_authorized_country(deps.storage, denom.clone(), param.to.clone())?;

        // ensure not freezed
        ensure_not_freezed(
            deps.storage,
            vec![param.from.clone(), param.to.clone()],
            denom.as_bytes(),
        )?;

        // ensure frozen balance and balance capital maintained
        ensure_bal_maintained(
            deps.as_ref(),
            param.to.clone(),
            param.from.clone(),
            denom.clone(),
            param.amount,
        )?;

        msgs.push(cm_transfer_marker_coins(
            param.amount.u128(),
            &denom,
            param.to.clone(),
            param.from.clone(),
            env.contract.address.clone(),
        )?);
        attrs.push(attr("funds", format!("{}{}", &param.amount, &denom)));
        attrs.push(attr("to", param.to));
        attrs.push(attr("from", param.from));
    }

    let res = Response::<ProvenanceMsg>::new()
        .add_messages(msgs)
        .add_attributes(attrs);

    Ok(res)
}
