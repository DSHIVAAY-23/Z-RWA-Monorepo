use super::*;

/// Handle messages that create and interact with with native provenance markers.
#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    match msg {
        ExecuteMsg::Create { params } => try_create(deps, info, env.contract.address, params),
        ExecuteMsg::GrantAccess { denom, address } => {
            try_grant_access(deps, denom, env.contract.address, info.sender, address)
        }
        ExecuteMsg::Freeze { denom, update_type } => {
            try_update_freezelist(deps, info.sender, denom, update_type)
        }
        ExecuteMsg::PartialFreeze { denom, params } => {
            try_partial_freeze(deps, info.sender, denom, params)
        }
        ExecuteMsg::Send { amount, denom, to } => {
            try_send(deps, amount, denom, to, info.sender, env.contract.address)
        }
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
        ExecuteMsg::SendMessageEvm {
            destination_chain,
            destination_address,
            message,
            msg_type,
        } => send_message_evm(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
            msg_type,
        ),
        ExecuteMsg::SendMessageCosmos {
            destination_chain,
            destination_address,
            message,
            msg_type,
        } => send_message_cosmos(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
            msg_type,
        ),
        ExecuteMsg::ReceiveMessageEvm {
            source_chain,
            source_address,
            payload,
        } => receive_message_evm(deps, source_chain, source_address, payload),
        ExecuteMsg::ReceiveMessageCosmos { sender, message } => {
            receive_message_cosmos(deps, sender, message)
        }
        ExecuteMsg::RequestOrder {
            order_id,
            denom,
            from,
            amount,
            request_type,
        } => try_request_from(
            deps,
            denom,
            info,
            order_id,
            from,
            amount,
            request_type,
            env.contract.address,
        ),
        ExecuteMsg::UpdateDestConfig { config } => update_dest_config(deps, config),
        ExecuteMsg::SetIbcResponse { is_required } => try_set_ibc_response(deps, is_required),
    }
}

/// Create and dispatch a message that will create a new restricted marker w/ active status.
/// List of country codes will be stored for allowing transactions only from whitelisted countries.
/// Token limit will be set so that no user can hold tokens above this limit.
/// Issuer address will be assigned for mint, burn, force_transfer, freeze and unfreeze operations.
/// Tokenization Agent address will be assigned for mint and burn operations.
/// Transfer Agent address will be assigned for force_transfer, freeze and unfreeze operations.
///
/// Parameters:-
///     - id: Uinique id for event purpose
///     - denom: Token name
///     - issuer: Issuser Address
///     - transfer_agent: Transfer Agent Address
///     - tokenization_agent: Tokenization Agent Address
///
/// Fails when:-
///     - caller is not sub_admin
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.create
///     - marker_denom
///     - denom_id
fn try_create(
    deps: DepsMut,
    info: MessageInfo,
    _contract_address: Addr,
    params: CreateMarkerParams,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, info.sender.clone())?;

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
        _contract_address.clone(),
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

    let msgs = create_marker(&params.denom, _contract_address)?;

    let res = Response::new()
        .add_messages(msgs)
        .add_attribute(
            "action",
            "provwasm.contracts.custom_marker.create.grant.finalize.activate",
        )
        .add_attribute("marker_denom", params.denom)
        .add_attribute("denom_id", params.id);

    Ok(res)
}

/// Create and dispatch a message that will grant all given permissions to a marker for an address.
///
/// Parameters:-
///     - denom: Token name
///     - address: Address to which all accesses will be provided
///
/// Fails when:-
///     - caller is not sub_admin
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.grant_access
///     - denom
///     - address
fn try_grant_access(
    deps: DepsMut,
    denom: String,
    _contract_address: Addr,
    sender: Addr,
    address: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Only Subadmin can grant access
    is_subadmin(&deps, sender)?;

    let msg = grant_marker_access(&denom, address.clone(), access())?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.custom_marker.grant_access")
        .add_attribute("denom", denom)
        .add_attribute("address", address);

    Ok(res)
}

/// Update Freeze List.
/// This function is used to freeze and unfreeze account.
/// If the account is freezed than it will not able to perform any transactions.
/// For freeze `update_type` will be `UpdateType::Add(Vec<Addr>)`
/// For unfreeze `update_type` will be `UpdateType::Remove(Vec<Addr>)`
/// This function supports batch operations, i.e. multiple addresses can be freezed / unfreezed simultaneously.
///
/// Parameters:-
///     - denom: Token name
///     - update_type: Can be either Add or Remove
///     - Vec<Addr>: List of addresses
///
/// Fails when:-
///     - caller is not issuer, transfer_agent, sub_admin or having freeze access for freezing
///     - caller is not issuer, transfer_agent, sub_admin or having unfreeze access for unfreezing
///     - freeze list doesn't contain the address that is going to be removed
///
/// Emits event:-
///     - when update_type = UpdateType::Add(Vec<Addr>), then
///         1. provwasm.contracts.custom_marker.freeze
///         2. addresses
///     - when update_type = UpdateType::Remove(Vec<Addr>), then
///         1. provwasm.contracts.custom_marker.unfreeze
///         2. addresses
fn try_update_freezelist(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    update_type: UpdateType<Vec<Addr>>,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // ensure not freezed
    ensure_not_freezed(deps.storage, vec![sender.clone()], denom.as_bytes())?;

    match update_type.clone() {
        UpdateType::Add(addrs) => {
            // Ensuring authorized sender
            if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
                && is_transfer_agent(&deps, denom.clone(), sender.clone()).is_err()
                && is_subadmin(&deps, sender.clone()).is_err()
            {
                let err = format!(
                    "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Freeze rights!",
                    &sender
                );
                return Err(ContractError::Unauthorized { err });
            }

            // Adding list of addresses to the freeze list, removing duplicate entries, if any
            FREEZE_LIST.update(
                deps.storage,
                denom.as_bytes(),
                |addresses_opt: Option<Vec<Addr>>| -> Result<_, ContractError> {
                    match addresses_opt {
                        Some(mut addresses) => Ok({
                            addresses.extend(addrs);
                            addresses.sort();
                            addresses.dedup();
                            addresses
                        }),
                        None => Ok(addrs),
                    }
                },
            )?;
        }
        UpdateType::Remove(addrs) => {
            // Ensuring authorized sender
            if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
                && is_transfer_agent(&deps, denom.clone(), sender.clone()).is_err()
                && is_subadmin(&deps, sender.clone()).is_err()
            {
                let err = format!(
                    "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Unfreeze rights!",
                    &sender
                );
                return Err(ContractError::Unauthorized { err });
            }

            // Removing list of addresses from freeze list
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
        }
    }

    let res = Response::new()
        .add_attribute("action", "update_blacklist")
        .add_attribute("update_type", format!("{:?}", &update_type));

    Ok(res)
}

/// Function to freeze / unfreeze partial balance of users.
/// Both partial freeze and partial unfreeze can be performed by this single entry function.
/// For partial freeze `update_type` will be `UpdateType::Add(Uint128)`
/// For partial unfreeze  `update_type` will be `UpdateType::Remove(Uint128)`
/// This function supports batch operations, i.e. multiple addresses can be partially freezed / unfreezed
/// simultaneously.
/// When the tokens are partially frozen then entire amount can't be used in any transactions, only unfreezed tokens
/// can take part in the transactions.
///  
/// Parameters:-
///     - denom: Token name
///     - List of addresses to be partially frozen
///     - update_type: can be either Add or Remove
///     - Amount to be partially frozen
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
    deps: DepsMut,
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
                    && is_subadmin(&deps, sender.clone()).is_err()
                {
                    let err = format!(
                        "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Freeze rights!",
                        &sender
                    );
                    return Err(ContractError::Unauthorized { err });
                }

                // Adding balance of the older accounts and saving new entry
                match PARTIAL_FREEZE.update(
                    deps.storage,
                    &key,
                    |bals_opt| -> Result<_, ContractError> {
                        match bals_opt {
                            Some(mut bals) => Ok({
                                bals += bal;
                                bals
                            }),
                            None => Ok(bal),
                        }
                    },
                ) {
                    Ok(_) => (),
                    Err(_) => PARTIAL_FREEZE.save(deps.storage, &key, &bal)?,
                };

                // Increasing frozen tokens value
                match FROZEN_TOKENS.update(
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
                ) {
                    Ok(_) => (),
                    Err(_) => FROZEN_TOKENS.save(deps.storage, denom.as_bytes(), &bal)?,
                };
            }
            UpdateType::Remove(bal) => {
                // Ensuring authorized sender
                if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
                    && is_transfer_agent(&deps, denom.clone(), sender.clone()).is_err()
                    && is_subadmin(&deps, sender.clone()).is_err()
                {
                    let err = format!(
                        "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Freeze rights!",
                        &sender
                    );
                    return Err(ContractError::Unauthorized { err });
                }

                // Decreasing frozen tokens value
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

                // Reducing balance
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
                // Removing enteries whose balance reaches to 0
                if rem_bal.is_zero() {
                    PARTIAL_FREEZE.remove(deps.storage, &key);
                }
            }
        }
        attrs.push(attr("address", format!("{:?}", &param.address)));
        attrs.push(attr("update_kind", format!("{:?}", &param.update_type)));
    }

    let res = Response::new()
        .add_attribute("action", "update_frozen_balance")
        .add_attributes(attrs);

    Ok(res)
}

/// Create and dispatch a message that will send coins from one account to another.
/// Caller is considered as the owner of token.
///
/// Parameters:-
///     - denom: Token name
///     - to: Receipient's wallet
///     - amount: Amount of tokens going to be transferred
///
/// Fails when:-
///     - recipient is freezed
///     - amount is zero
///     - unfreezed balance is lesser than requested amount
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.send
///     - funds
///     - to
///     - from
fn try_send(
    deps: DepsMut,
    amount: Uint128,
    denom: String,
    to: Addr,
    from: Addr,
    contract_address: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // ensure not freezed
    ensure_not_freezed(deps.storage, vec![to.clone()], denom.as_bytes())?;

    // ensure balance not frozen
    ensure_bal_not_frozen(deps.as_ref(), from.clone(), denom.clone())?;

    let transfer = transfer_marker_coins(
        amount.u128(),
        &denom,
        to.clone(),
        from.clone(),
        contract_address,
    )?;

    let res = Response::new()
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
/// Parameters:-
///     - List of tokens
///     - List of addresses, where token will be minted
///     - List of amounts of tokens to be minted
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
    deps: DepsMut,
    mint_to_params: Vec<MintBurnParams>,
    sender: Addr,
    contract_address: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let mut msgs: Vec<CosmosMsg<ProvenanceMsg>> = Vec::new();

    for params in mint_to_params {
        // Ensuring authorized sender
        if is_issuer(&deps, params.denom.clone(), sender.clone()).is_err()
            && is_tokenization_agent(&deps, params.denom.clone(), sender.clone()).is_err()
            && is_subadmin(&deps, sender.clone()).is_err()
        {
            let err = format!(
                "Address `{}`: Don't have Issuer, Tokenization, Sub Admin or Mint rights!",
                &sender
            );
            return Err(ContractError::Unauthorized { err });
        }

        for mint_data in params.mint_burn_data {
            // Add mint amount to pool
            update_minted_tokens(
                deps.storage,
                params.denom.to_string(),
                UpdateType::Add(mint_data.amount),
            )?;

            msgs.extend(mint_to(
                params.denom.to_string(),
                mint_data,
                contract_address.clone(),
            )?);
        }
    }

    let res = Response::new()
        .add_messages(msgs)
        .add_attribute("action", "provwasm.contracts.custom_marker.mint_to");

    Ok(res)
}

/// Create and dispatch a message that will burn coins from address.
/// This function supports batch operations, i.e. multiple addresses can be burned simultaneously.
/// Decreases minted token value.
///  
/// Parameters:-
///     - List of tokens
///     - List of addresses, where token will be burned
///     - List of amounts of tokens to be burned
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
    deps: DepsMut,
    burn_to_params: Vec<MintBurnParams>,
    _contract_address: Addr,
    sender: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let mut msgs: Vec<CosmosMsg<ProvenanceMsg>> = Vec::new();
    let querier = provwasm_std::ProvenanceQuerier::new(&deps.querier);

    for params in burn_to_params {
        // Ensuring authorized sender
        if is_issuer(&deps, params.denom.clone(), sender.clone()).is_err()
            && is_tokenization_agent(&deps, params.denom.clone(), sender.clone()).is_err()
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

            // Remove mint amount from pool
            update_minted_tokens(
                deps.storage,
                params.denom.clone(),
                UpdateType::Remove(burn_data.amount),
            )?;

            msgs.extend(burn_from(
                params.denom.clone(),
                burn_data,
                _contract_address.clone(),
                &querier,
            )?);
        }
    }

    let res = Response::new()
        .add_messages(msgs)
        .add_attribute("action", "provwasm.contracts.custom_marker.burn_to");

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
    deps: DepsMut,
    info: MessageInfo,
    contract_address: Addr,
    denom: String,
    roles: Vec<Role>,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let mut attrs = Vec::new();
    let mut msgs = Vec::new();

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
                                msgs.push(grant_marker_access(
                                    denom.clone(),
                                    contract_address.clone(),
                                    access(),
                                )?);

                                Ok(addresses)
                            },
                        );
                        if updated.is_err() {
                            SUB_ADMIN.save(deps.storage, &addrs)?;
                        };
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.add_sub_admin",
                        ));
                    }
                    UpdateType::Remove(addrs) => {
                        SUB_ADMIN.update(
                            deps.storage,
                            |mut addresses| -> Result<_, ContractError> {
                                addresses.retain(|addr| !addrs.contains(addr));

                                // Revoke marker accesses
                                msgs.push(grant_marker_access(
                                    denom.clone(),
                                    contract_address.clone(),
                                    Vec::new(),
                                )?);

                                Ok(addresses)
                            },
                        )?;
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.remove_sub_admin",
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
            Role::Operators { update_type } => {
                // Ensuring caller has the admin rights
                is_subadmin(&deps, info.sender.clone())?;

                match update_type {
                    UpdateType::Add(addrs) => {
                        let updated = OPERATORS.update(
                            deps.storage,
                            |mut addresses| -> Result<_, ContractError> {
                                addresses.extend(addrs.clone());
                                addresses.sort();
                                addresses.dedup();
                                Ok(addresses)
                            },
                        );
                        if updated.is_err() {
                            OPERATORS.save(deps.storage, &addrs)?;
                        };
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.add_operators",
                        ));
                    }
                    UpdateType::Remove(addrs) => {
                        OPERATORS.update(
                            deps.storage,
                            |mut addresses| -> Result<_, ContractError> {
                                addresses.retain(|addr| !addrs.contains(addr));
                                Ok(addresses)
                            },
                        )?;
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.custom_marker.remove_operators",
                        ));
                    }
                }
            }
        }
    }

    Ok(Response::new().add_attributes(attrs))
}

/// Create and dispatch a message that will force transfer coins from one account to another.
/// This function supports batch operations, i.e. multiple amounts can be transffered from accounts simultaneously.
///  
/// Parameters:-
///     - List of addresses from which the tokens will be transffered from
///     - List of corresponding receipients' addresses from where the tokens will be transffered to
///     - List of corresponding token amounts that will be transferred
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
    deps: DepsMut,
    denom: String,
    params: Vec<ForceTransferParams>,
    sender: Addr,
    env: Env,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Ensuring authorized sender
    if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
        && is_transfer_agent(&deps, denom.clone(), sender.clone()).is_err()
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

    for param in params {
        // ensure not freezed
        ensure_not_freezed(
            deps.storage,
            vec![param.from.clone(), param.to.clone()],
            denom.as_bytes(),
        )?;

        // ensure frozen balance
        ensure_bal_not_frozen(deps.as_ref(), param.from.clone(), denom.clone())?;

        msgs.push(transfer_marker_coins(
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

    let res = Response::new().add_messages(msgs).add_attributes(attrs);

    Ok(res)
}

/// Sends a message via Axelar GMP to the EVM {destination_chain} and {destination_address}
///
/// Parameters:-
///     - destination_chain: Name of destination chain
///     - destination_address: Destinantion chain address
///     - message: Message to be encrypted
///     - msg_type: Message type, can be either message, token or message with token
///
/// Fails when:-
///     - the transaction doesn't contains atleast one coin
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.send_message_evm
pub fn send_message_evm(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    message: String,
    msg_type: MessageType,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Message payload to be received by the destination
    let message_payload = encode(&[Token::String(message)]);

    // {info.funds} used to pay gas. Must only contain 1 token type.
    let coin: cosmwasm_std::Coin =
        cw_utils::one_coin(&info).expect("Atleast one coin should be present!");

    let gmp_message: GmpMessage = GmpMessage {
        destination_chain,
        destination_address,
        payload: message_payload.to_vec(),
        type_: msg_type.into_i64(),
        fee: Some(Fee {
            amount: coin.amount.to_string(),
            recipient: String::from("axelar1zl3rxpp70lmte2xr6c4lgske2fyuj3hupcsvcd"),
        }),
    };

    let ibc_message = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: "channel-75".to_string(),
        token: Some(coin.into()),
        sender: env.contract.address.to_string(),
        receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5".to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
        memo: to_string(&gmp_message).unwrap(),
    };

    Ok(Response::new().add_message(ibc_message).add_attribute(
        "action",
        "provwasm.contracts.custom_marker.send_message_evm",
    ))
}

/// Sends a message via Axelar GMP to the other cosmos chains
/// only difference is how the {message_payload} is constructed
///
/// Parameters:-
///     - destination_chain: Name of destination chain
///     - destination_address: Destinantion chain address
///     - message: Message to be encrypted
///     - msg_type: Message type, can be either message, token or message with token
///
/// Fails when:-
///     - the transaction doesn't contains atleast one coin
///     - serialization to JSON fails
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.send_message_evm
pub fn send_message_cosmos(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    message: String,
    msg_type: MessageType,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Construct contract call
    let contract_call = serde_json_wasm::to_string(&ExecuteMsg::ReceiveMessageCosmos {
        sender: info.sender.to_string(),
        message,
    })
    .expect("Failed to serialize struct to JSON");
    let utf8_bytes = contract_call.as_bytes();
    let utf8_vec = utf8_bytes.to_owned();
    // prepend 4 bytes to indicate the payload verison
    let mut message_payload: Vec<u8> = vec![0, 0, 0, 2];
    message_payload.extend(utf8_vec);

    // {info.funds} used to pay gas. Must only contain 1 token type.
    let coin: cosmwasm_std::Coin =
        cw_utils::one_coin(&info).expect("Atleast one coin should be present!");

    let gmp_message: GmpMessage = GmpMessage {
        destination_chain,
        destination_address,
        payload: message_payload.to_vec(),
        type_: msg_type.into_i64(), //type = 1
        fee: Some(Fee {
            amount: coin.amount.to_string(),
            recipient: String::from("axelar1zl3rxpp70lmte2xr6c4lgske2fyuj3hupcsvcd"),
        }),
    };

    let ibc_message = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: "channel-75".to_string(),
        token: Some(coin.into()),
        sender: env.contract.address.to_string(),
        receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5".to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
        memo: to_string(&gmp_message).unwrap(),
    };

    Ok(Response::new().add_message(ibc_message).add_attribute(
        "action",
        "provwasm.contracts.custom_marker.receive_message_cosmos",
    ))
}

/// Function to receive message coming from evm chain
///
/// Parameters:-
///     - source_chain: Name of source chain
///     - source_address: Source chain address
///     - payload: Encrypted message  
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.receive_message_evm
pub fn receive_message_evm(
    deps: DepsMut,
    _source_chain: String,
    _source_address: String,
    payload: Binary,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // decode the payload
    // executeMsgPayload: [sender, message]
    let decoded = decode(&[ParamType::String, ParamType::String], payload.as_slice()).unwrap();

    // store message
    STORED_MESSAGE.save(
        deps.storage,
        &Message {
            sender: decoded[0].to_string(),
            message: decoded[1].to_string(),
        },
    )?;

    Ok(Response::new().add_attribute(
        "action",
        "provwasm.contracts.custom_marker.receive_message_cosmos",
    ))
}

/// Function to receive message coming from cosmos chain
///
/// Parameters:-
///     - message: Message
///
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.receive_message_cosmos
pub fn receive_message_cosmos(
    deps: DepsMut,
    sender: String,
    message: String,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // store message
    STORED_MESSAGE.save(
        deps.storage,
        &Message {
            sender,
            message: message.clone(),
        },
    )?;

    Ok(Response::new().add_attribute(
        "action",
        "provwasm.contracts.custom_marker.receive_message_cosmos",
    ))
}

/// Function for request
///
/// Parameters:-
///     - denom: Token name
///     - order_id: Order id
///     - from: Adress from which burn / mint will be performed
///     - amount: Amount of tokens to be minted / burned
///     - request_typre: Can be either mint or burn
///     
/// Fails when:-
///     - Amount is zero
///     - Mint balance failed to be updated
///     - Inter smart contract communication fails
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.request_from
///     - request_type
///     - order_id
///     - from
///     - amount
#[allow(clippy::too_many_arguments)]
fn try_request_from(
    deps: DepsMut,
    denom: String,
    info: MessageInfo,
    order_id: String,
    from: Addr,
    amount: Uint128,
    request_type: RequestType,
    contract_address: Addr,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Ensuring authorized sender
    is_subadmin(&deps, contract_address.clone())?;

    // Ensuring amount
    if amount.is_zero() {
        return Err(ContractError::AmountCannotBeZero {});
    }

    let mut msgs;
    let dest_config = DEST_CONFIG.load(deps.storage)?;
    let is_ibc_response_required = IS_IBC_RESPONSE_REQUIRED.load(deps.storage)?;

    match request_type {
        RequestType::Mint => {
            // Ensuring request doesn't exists
            if REQUESTS.load(deps.storage, order_id.to_string()).is_ok() {
                return Err(ContractError::RequestExists {});
            }

            update_mint_balances(deps.storage, info.sender.clone(), UpdateType::Add(amount))?;

            msgs = mint_to(
                denom,
                MintBurnData {
                    address: from.clone(),
                    amount,
                },
                contract_address.clone(),
            )?;

            if is_ibc_response_required {
                msgs.push(create_response(
                    dest_config,
                    contract_address,
                    order_id.to_string(),
                    info.funds,
                )?);
            }
        }
        RequestType::Burn => {
            // Ensuring request doesn't exists
            if REQUESTS.load(deps.storage, order_id.to_string()).is_ok() {
                return Err(ContractError::RequestExists {});
            }

            update_burn_balances(deps.storage, info.sender.clone(), UpdateType::Add(amount))?;

            let querier = provwasm_std::ProvenanceQuerier::new(&deps.querier);

            msgs = burn_from(
                denom,
                MintBurnData {
                    address: from.clone(),
                    amount,
                },
                contract_address.clone(),
                &querier,
            )?;

            if is_ibc_response_required {
                msgs.push(create_response(
                    dest_config,
                    contract_address,
                    order_id.to_string(),
                    info.funds,
                )?);
            }
        }
    }

    REQUESTS.save(
        deps.storage,
        order_id.to_string(),
        &Request {
            requester: from.clone(),
            responder: info.sender.clone(),
            amount,
            request_type: request_type.clone(),
        },
    )?;

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "provwasm.contracts.custom_marker.request_from")
        .add_attribute("request_type", request_type.to_string())
        .add_attribute("order_id", order_id)
        .add_attribute("from", from.clone())
        .add_attribute("amount", amount))
}

/// Update Destination Config
///
/// Parameters:-
///     - chain: chain name
///     - address: chain address
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.update_destination_config
pub fn update_dest_config(
    deps: DepsMut,
    config: DestConfig,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // store destination configuration
    DEST_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute(
        "action",
        "provwasm.contracts.custom_marker.update_destination_config",
    ))
}

/// Set IBC Response flag
///
/// Parameters:-
///     - is_required_bool: if true then ibc response otherwise no ibc response
///
/// Emits event:-
///     - provwasm.contracts.custom_marker.set_ibc_response
pub fn try_set_ibc_response(
    deps: DepsMut,
    is_required: bool,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    IS_IBC_RESPONSE_REQUIRED.save(deps.storage, &is_required)?;

    Ok(Response::new().add_attribute(
        "action",
        "provwasm.contracts.custom_marker.set_ibc_response",
    ))
}
