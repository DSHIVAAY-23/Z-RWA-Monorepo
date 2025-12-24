#[cfg(not(feature = "library"))]
use super::*;

/// Handle messages that create and interact with with native provenance markers.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateTokenContract { code_id } => {
            try_update_token_contract(deps, info, code_id)
        }
        ExecuteMsg::DeployToken { params } => {
            try_deploy_token(deps, info, env.contract.address, params)
        }
        ExecuteMsg::MintTo { denom, params } => try_mint_to(deps, info.sender, denom, params),
        ExecuteMsg::Transfer { amount, denom, to } => {
            let to = deps.api.addr_validate(&to)?;
            try_transfer(deps, amount, denom, to, env.contract.address)
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
        ExecuteMsg::BurnFrom {
            denom,
            burn_from_params,
        } => try_burn_from(
            deps,
            denom,
            burn_from_params,
            env.contract.address,
            info.sender,
        ),
        ExecuteMsg::ManageRoles { denom, roles } => try_manage_roles(deps, info, denom, roles),
    }
}

// Update Token Contract Id.
fn try_update_token_contract(
    deps: DepsMut,
    info: MessageInfo,
    code_id: u64,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, info.sender.clone())?;

    CODE_ID.update(deps.storage, |mut id| -> Result<_, ContractError> {
        id = code_id;
        Ok(id)
    })?;

    Ok(Response::new()
        .add_attribute("action", "code_id_updated")
        .add_attribute("id", code_id.to_string()))
}

// Deploying new token.
fn try_deploy_token(
    deps: DepsMut,
    info: MessageInfo,
    contract_address: Addr,
    params: token_contract::msg::Instantiate,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, info.sender.clone())?;

    let code_id = CODE_ID.load(deps.storage)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_instantiate(
        code_id,
        &params,
        Vec::default(),
        String::from("token-contract"),
    )?);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "token_deployed"))
}

// Create and dispatch a message that will mint coins into a marker.
fn try_mint_to(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    params: Vec<MintBurnData>,
) -> Result<Response, ContractError> {
    // Fetching contract address
    let token_contract_address = get_contract_address(deps.as_ref(), denom)?;

    let mut msgs: Vec<CosmosMsg> = Vec::new();

    // Ensuring authorized sender
    if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
        && is_tokenization_agent(&deps, denom.clone(), sender.clone()).is_err()
        && has_mint_access(&deps, denom.clone(), sender.clone()).is_err()
        && is_subadmin(&deps, sender.clone()).is_err()
    {
        let err = format!(
            "Address `{}`: Don't have Issuer, Tokenization, Sub Admin or Mint rights!",
            &sender
        );
        return Err(ContractError::Unauthorized { err });
    }

    for mint_data in params {
        let exe_msg = token_contract::msg::Execute::Mint {
            to_addr: mint_data.address,
            amount: mint_data.amount,
        };
        let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
            token_contract_address,
            &exe_msg,
            Vec::default(),
        )?);
        msgs.push(msg);
    }

    Ok(Response::new().add_messages(msgs))
}

// Create and dispatch a message that will burn coins from address.
fn try_burn_from(
    deps: DepsMut,
    denom: String,
    params: Vec<MintBurnData>,
    contract_address: Addr,
    sender: Addr,
) -> Result<Response, ContractError> {
    // Fetching contract address
    let token_contract_address = get_contract_address(deps.as_ref(), denom)?;

    let mut msgs: Vec<CosmosMsg> = Vec::new();

    // Ensuring authorized sender
    if is_issuer(&deps, denom.clone(), sender.clone()).is_err()
        && is_tokenization_agent(&deps, denom.clone(), sender.clone()).is_err()
        && has_burn_access(&deps, denom.clone(), sender.clone()).is_err()
        && is_subadmin(&deps, sender.clone()).is_err()
    {
        let err = format!(
            "Address `{}`: Don't have Issuer, Tokenization, Sub Admin or Burn rights!",
            &sender
        );
        return Err(ContractError::Unauthorized { err });
    }

    for burn_data in params {
        // ensure token balance no frozen.
        ensure_bal_not_frozen(
            deps.as_ref(),
            Addr::unchecked(burn_data.address.clone()),
            denom.clone(),
        )?;

        let exe_msg = token_contract::msg::Execute::BurnFrom {
            owner: burn_data.address,
            amount: burn_data.amount,
        };
        let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
            token_contract_address,
            &exe_msg,
            Vec::default(),
        )?);

        msgs.push(msg);
    }

    Ok(Response::new().add_messages(msgs))
}

// Create and dispatch a message that will transfer coins from one account to another.
fn try_transfer(
    deps: DepsMut,
    amount: Uint128,
    denom: String,
    to: Addr,
    from: Addr,
) -> Result<Response, ContractError> {
    // ensuring country is authorized
    ensure_authorized_country(deps.storage, denom.clone(), to.clone())?;

    // ensure not freezed
    ensure_not_freezed(deps.storage, vec![to.clone()], denom.as_bytes())?;

    // ensure token limit maintained.
    ensure_token_limit(deps.as_ref(), to.clone(), denom.clone(), amount)?;

    let transfer = transfer_marker_coins(amount.u128(), &denom, to.clone(), from.clone())?;

    let res = Response::new()
        .add_message(transfer)
        .add_attribute("action", "provwasm.contracts.marker.transfer")
        .add_attribute("funds", format!("{}{}", &amount, &denom))
        .add_attribute("to", to)
        .add_attribute("from", from);

    Ok(res)
}

// Update Freeze List.
fn try_update_freezelist(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    update_type: UpdateType<Vec<Addr>>,
) -> Result<Response, ContractError> {
    // ensure not freezed
    ensure_not_freezed(deps.storage, vec![sender.clone()], denom.as_bytes())?;

    match update_type.clone() {
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
                && has_unfreeze_access(&deps, denom.clone(), sender.clone()).is_err()
                && is_subadmin(&deps, sender.clone()).is_err()
            {
                let err = format!(
                    "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Unfreeze rights!",
                    &sender
                );
                return Err(ContractError::Unauthorized { err });
            }

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

// Update frozen balance.
fn try_partial_freeze(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    params: Vec<PartialFreezeParams>,
) -> Result<Response, ContractError> {
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
                    && has_unfreeze_access(&deps, denom.clone(), sender.clone()).is_err()
                    && is_subadmin(&deps, sender.clone()).is_err()
                {
                    let err = format!(
                        "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Freeze rights!",
                        &sender
                    );
                    return Err(ContractError::Unauthorized { err });
                }

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

// Create and dispatch a message that will transfer coins from one account to another.
fn try_send(
    deps: DepsMut,
    amount: Uint128,
    denom: String,
    to: Addr,
    from: Addr,
    contract_address: Addr,
) -> Result<Response, ContractError> {
    // Ensuring authorized sender
    has_transfer_access(&deps, denom.clone(), from.clone())?;

    if contract_address.ne(&to) {
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

    let transfer = transfer_marker_coins(amount.u128(), &denom, to.clone(), from.clone())?;

    let res = Response::new()
        .add_message(transfer)
        .add_attribute("action", "provwasm.contracts.marker.send")
        .add_attribute("funds", format!("{}{}", &amount, &denom))
        .add_attribute("to", to)
        .add_attribute("from", from);

    Ok(res)
}

// Function to manage different roles
pub fn try_manage_roles(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    roles: Vec<Role>,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();

    for role in roles {
        match role {
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
                                Ok(addresses)
                            },
                        );
                        if updated.is_err() {
                            SUB_ADMIN.save(deps.storage, &addrs)?;
                        };
                        attrs.push(attr("action", "provwasm.contracts.marker.add_sub_admin"));
                    }
                    UpdateType::Remove(addrs) => {
                        SUB_ADMIN.update(
                            deps.storage,
                            |mut addresses| -> Result<_, ContractError> {
                                addresses.retain(|addr| !addrs.contains(addr));
                                Ok(addresses)
                            },
                        )?;
                        attrs.push(attr("action", "provwasm.contracts.marker.remove_sub_admin"));
                    }
                }
            }
            Role::Admin { address } => {
                // Ensuring caller has the admin rights
                is_admin(&deps, info.sender.clone())?;

                ADMIN.save(deps.storage, &address)?;
                attrs.push(attr("action", "provwasm.contracts.marker.update_admin"));
            }
        }
    }

    Ok(Response::new().add_attributes(attrs))
}

// Create and dispatch a message that will force transfer coins from one account to another.
fn try_force_transfer(
    deps: DepsMut,
    denom: String,
    params: Vec<ForceTransferParams>,
    sender: Addr,
) -> Result<Response, ContractError> {
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

    let mut msgs = Vec::new();
    let mut attrs = Vec::new();

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

        msgs.push(transfer_marker_coins(
            param.amount.u128(),
            &denom,
            param.to.clone(),
            param.from.clone(),
        )?);
        attrs.push(attr("funds", format!("{}{}", &param.amount, &denom)));
        attrs.push(attr("to", param.to));
        attrs.push(attr("from", param.from));
    }

    let res = Response::new().add_messages(msgs).add_attributes(attrs);

    Ok(res)
}
