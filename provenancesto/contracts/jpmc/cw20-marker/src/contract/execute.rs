use super::*;

/// Handle messages that create and interact with with native provenance markers.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Transfer { amount, to } => try_transfer(deps, amount, to, info.sender),
        Freeze { update_type } => try_update_freezelist(deps, info.sender, update_type),
        PartialFreeze { params } => try_partial_freeze(deps, info.sender, params),
        ManageRoles { roles } => try_manage_roles(deps, info, roles),
        TransferFrom { amount, from, to } => {
            try_transfer_from(deps, amount, from, to, info.sender, env.contract.address)
        }
        Request {
            request_id,
            amount,
            request_type,
        } => try_request(
            deps,
            info.sender,
            env.contract.address,
            request_id,
            amount,
            request_type,
        ),
        RequestFrom {
            request_id,
            from,
            amount,
            request_type,
        } => try_request_from(
            deps,
            info.sender,
            env.contract.address,
            request_id,
            from,
            amount,
            request_type,
        ),
        ApproveRequest {
            request_id,
            request_type,
        } => try_approve_request(
            deps,
            info.sender,
            env.contract.address,
            request_id,
            request_type,
        ),
        RejectRequest {
            request_id,
            request_type,
        } => try_reject(
            deps,
            info.sender,
            env.contract.address,
            request_id,
            request_type,
        ),
        ManageRequestAllowance {
            spender,
            update_type,
            request_type,
        } => try_manage_request_allowance(deps, info.sender, spender, update_type, request_type),
        Approve { spender, amount } => try_approve(deps, spender, info.sender, amount),
        SendMessageEvm {
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
        SendMessageCosmos {
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
        ReceiveMessageEvm {
            source_chain,
            source_address,
            payload,
        } => receive_message_evm(deps, source_chain, source_address, payload),
        ReceiveMessageCosmos { sender, message } => receive_message_cosmos(deps, sender, message),
        RescueCoins {
            denom,
            to_address,
            amount,
        } => try_rescue_coins(deps, info.sender, denom, to_address, amount),
        UpdateDestConfig { config } => update_dest_config(deps, config),
        ClearBurnBalance { address } => try_clear_burn_balance(deps, info.sender, address),
    }
}

// Create and dispatch a message that will transfer coins from one account to another.
fn try_transfer(
    deps: DepsMut,
    amount: Uint128,
    to: Addr,
    from: Addr,
) -> Result<Response, ContractError> {
    // ensure not freezed
    let denom = get_denom(deps.storage)?;
    ensure_not_freezed(deps.storage, vec![to.clone()])?;

    // Ensuring amount must be greater than zero
    if amount.is_zero() {
        return Err(ContractError::AmountCannotBeZero {});
    }

    let transfer = transfer_marker_coins(
        amount.u128(),
        &denom,
        to.clone(),
        from.clone(),
        from.clone(),
    )?;

    let res = Response::new()
        .add_message(transfer)
        .add_attribute("action", "provwasm.contracts.cw20.marker.transfer")
        .add_attribute("funds", format!("{}{}", &amount, &denom))
        .add_attribute("to", to)
        .add_attribute("from", from);

    Ok(res)
}

// Update Freeze List.
fn try_update_freezelist(
    deps: DepsMut,
    sender: Addr,
    update_type: UpdateType<Vec<Addr>>,
) -> Result<Response, ContractError> {
    // Ensuring authorized sender
    is_subadmin(&deps, sender.clone())?;

    // ensure not freezed
    ensure_not_freezed(deps.storage, vec![sender])?;

    match update_type.clone() {
        UpdateType::Add(addrs) => {
            FREEZE_LIST.update(
                deps.storage,
                |mut addresses: Vec<Addr>| -> Result<_, ContractError> {
                    Ok({
                        addresses.extend(addrs);
                        addresses.sort();
                        addresses.dedup();
                        addresses
                    })
                },
            )?;
        }
        UpdateType::Remove(addrs) => {
            FREEZE_LIST.update(
                deps.storage,
                |mut addresses: Vec<Addr>| -> Result<_, ContractError> {
                    Ok({
                        addresses.retain(|addr| !addrs.contains(addr));
                        addresses
                    })
                },
            )?;
        }
    }

    let res = Response::new()
        .add_attribute("action", "provwasm.contracts.cw20.marker.update_freeze")
        .add_attribute("update_type", format!("{:?}", &update_type));

    Ok(res)
}

// Update frozen balance.
fn try_partial_freeze(
    deps: DepsMut,
    sender: Addr,
    params: Vec<PartialFreezeParams>,
) -> Result<Response, ContractError> {
    // Ensuring authorized sender
    is_subadmin(&deps, sender)?;

    let mut attrs = Vec::new();
    let mut rem_bal = Uint128::zero();

    for param in params {
        match param.update_type {
            UpdateType::Add(bal) => {
                if PARTIAL_FREEZE
                    .update(
                        deps.storage,
                        param.address.clone(),
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
                    PARTIAL_FREEZE.save(deps.storage, param.address.clone(), &bal)?;
                };

                if FROZEN_TOKENS
                    .update(deps.storage, |mut bals| -> Result<_, ContractError> {
                        Ok({
                            bals += bal;
                            bals
                        })
                    })
                    .is_err()
                {
                    FROZEN_TOKENS.save(deps.storage, &bal)?;
                }
            }
            UpdateType::Remove(bal) => {
                FROZEN_TOKENS.update(deps.storage, |mut bals| -> Result<_, ContractError> {
                    Ok({
                        bals -= bal;
                        bals
                    })
                })?;

                PARTIAL_FREEZE.update(
                    deps.storage,
                    param.address.clone(),
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
                    PARTIAL_FREEZE.remove(deps.storage, param.address.clone());
                }
            }
        }
        attrs.push(attr("address", format!("{:?}", &param.address)));
        attrs.push(attr("update_kind", format!("{:?}", &param.update_type)));
    }

    let res = Response::new()
        .add_attribute(
            "action",
            "provwasm.contracts.cw20.marker.update_frozen_balance",
        )
        .add_attributes(attrs);

    Ok(res)
}

// Function to manage different roles
pub fn try_manage_roles(
    deps: DepsMut,
    info: MessageInfo,
    roles: Vec<Role>,
) -> Result<Response, ContractError> {
    // Ensuring authorized sender
    is_subadmin(&deps, info.sender)?;

    let mut attrs = Vec::new();

    for role in roles {
        match role {
            Role::SubAdmin { update_type } => match update_type {
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
                    attrs.push(attr(
                        "action",
                        "provwasm.contracts.cw20.marker.add_sub_admin",
                    ));
                }
                UpdateType::Remove(addrs) => {
                    SUB_ADMIN.update(
                        deps.storage,
                        |mut addresses| -> Result<_, ContractError> {
                            addresses.retain(|addr| !addrs.contains(addr));
                            Ok(addresses)
                        },
                    )?;
                    attrs.push(attr(
                        "action",
                        "provwasm.contracts.cw20.marker.remove_sub_admin",
                    ));
                }
            },
            Role::TokenizationAgent { update_type } => match update_type {
                UpdateType::Add(addr) => {
                    let agent = TOKENIZATION_AGENT.load(deps.storage)?;
                    if agent == addr {
                        return Err(ContractError::AlreadyAdded { addr });
                    }
                    TOKENIZATION_AGENT.save(deps.storage, &addr)?;
                    attrs.push(attr(
                        "action",
                        "provwasm.contracts.cw20.marker.add_tokenization_agent",
                    ));
                }
                UpdateType::Remove(addr) => {
                    if !TOKENIZATION_AGENT.exists(deps.storage) {
                        return Err(ContractError::NotFound { addr });
                    }
                    TOKENIZATION_AGENT.remove(deps.storage);
                    attrs.push(attr(
                        "action",
                        "provwasm.contracts.cw20.marker.remove_tokenization_agent",
                    ));
                }
            },
        }
    }

    Ok(Response::new().add_attributes(attrs))
}

// Create and dispatch a message that will force transfer coins from one account to another.
fn try_transfer_from(
    deps: DepsMut,
    amount: Uint128,
    from: Addr,
    to: Addr,
    sender: Addr,
    contract_address: Addr,
) -> Result<Response, ContractError> {
    let denom = get_denom(deps.storage)?;
    let key = Key::new(from.clone(), sender.clone()).as_bytes()?;

    // Ensuring authorized sender
    is_subadmin(&deps, sender)?;

    let mut msgs = Vec::new();

    // ensure not freezed
    ensure_not_freezed(deps.storage, vec![from.clone(), to.clone()])?;

    // ensure frozen balance
    ensure_bal_not_frozen(deps.as_ref(), from.clone())?;
    let mut rem_allowance = Uint128::zero();

    if ALLOWANCE
        .update(
            deps.storage,
            &key,
            |allowance_opt: Option<Uint128>| -> Result<_, ContractError> {
                match allowance_opt {
                    Some(mut allowance) => Ok({
                        if allowance < amount {
                            return Err(ContractError::AllowanceTooLow { allowance, amount });
                        }

                        msgs.push(transfer_marker_coins(
                            amount.u128(),
                            &denom,
                            to.clone(),
                            from.clone(),
                            contract_address.clone(),
                        )?);

                        allowance -= amount;
                        rem_allowance = allowance;
                        allowance
                    }),
                    None => Err(ContractError::Std(StdError::not_found(
                        "Allowance not found!",
                    ))),
                }
            },
        )
        .is_err()
    {
        let err = format!("Spender: `{}` is not authorized!", from);
        return Err(ContractError::Unauthorized { err });
    }

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "provwasm.contracts.cw20.marker.transfer_from")
        .add_attribute("funds", format!("{}{}", &amount, &denom))
        .add_attribute("to", to)
        .add_attribute("from", from)
        .add_attribute("allowance_remaining", rem_allowance))
}

// Function for request
fn try_request(
    deps: DepsMut,
    sender: Addr,
    contract_address: Addr,
    request_id: String,
    amount: Uint128,
    request_type: RequestType,
) -> Result<Response, ContractError> {
    let mut msgs = Vec::new();

    // Ensuring amount must be greater than zero
    if amount.is_zero() {
        return Err(ContractError::AmountCannotBeZero {});
    }

    // Ensuring request doesn't exists
    if REQUESTS.load(deps.storage, request_id.as_bytes()).is_ok() {
        return Err(ContractError::RequestExists { request_id });
    }

    let attribute = match request_type {
        RequestType::Mint => attr("action", "provwasm.contracts.cw20.marker.mint_requested"),
        RequestType::Burn => {
            let bal = get_consolidated_balance(deps.as_ref(), sender.clone())?;
            // Ensuring balance is sufficient
            check_bal_avalaility(amount, bal, ContractError::BalanceLow { bal, cap: amount })?;

            let denom = get_denom(deps.storage)?;
            let querier = MarkerQuerier::new(&deps.querier);
            let marker_addr =
                get_marker_address(validate_string(denom.clone(), "denom")?, &querier)?;
            msgs.push(transfer_marker_coins(
                amount.u128(),
                denom,
                marker_addr,
                sender.clone(),
                contract_address,
            )?);
            update_burn_balances(deps.storage, sender.clone(), UpdateType::Add(amount))?;

            attr("action", "provwasm.contracts.cw20.marker.burn_requested")
        }
    };

    REQUESTS.save(
        deps.storage,
        request_id.as_bytes(),
        &Request {
            request_type,
            status: Status::Pending,
            requester: sender.clone(),
            responder: None,
            amount,
        },
    )?;

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attribute])
        .add_attribute("request_id", request_id)
        .add_attribute("address", sender.into_string())
        .add_attribute("amount", amount))
}

// Function for request
fn try_request_from(
    deps: DepsMut,
    sender: Addr,
    contract_address: Addr,
    request_id: String,
    from: Addr,
    amount: Uint128,
    request_type: RequestType,
) -> Result<Response, ContractError> {
    let key = Key::new(from.clone(), sender.clone()).as_bytes()?;
    let mut msgs = Vec::new();
    let dest_config = DEST_CONFIG.load(deps.storage)?;

    // Ensuring amount must be greater than zero
    if amount.is_zero() {
        return Ok(Response::new().add_message(create_response(
            dest_config,
            contract_address,
            ContractError::AmountCannotBeZero {}.to_string(),
        )?));
    }

    // Ensuring request doesn't exists
    if REQUESTS.load(deps.storage, request_id.as_bytes()).is_ok() {
        return Ok(Response::new().add_message(create_response(
            dest_config,
            contract_address,
            ContractError::RequestExists { request_id }.to_string(),
        )?));
    }

    let denom = get_denom(deps.storage)?;
    let querier = MarkerQuerier::new(&deps.querier);
    let marker_addr = get_marker_address(validate_string(denom.clone(), "denom")?, &querier)?;

    let attribute = match request_type {
        RequestType::Mint => {
            // Reduce mint allowance
            if MINT_ALLOWANCES
                .update(
                    deps.storage,
                    &key,
                    |allowance_opt: Option<Uint128>| -> Result<_, ContractError> {
                        match allowance_opt {
                            Some(mut allowance) => Ok({
                                if allowance < amount {
                                    return Err(ContractError::AllowanceTooLow {
                                        allowance,
                                        amount,
                                    });
                                }
                                allowance -= amount;
                                allowance
                            }),
                            None => Err(ContractError::AllowanceNotFound {
                                owner: from.clone(),
                                spender: sender.clone(),
                            }),
                        }
                    },
                )
                .is_err()
            {
                return Ok(Response::new().add_message(create_response(
                    dest_config,
                    contract_address,
                    ContractError::AllowanceNotFound {
                        owner: from,
                        spender: sender,
                    }
                    .to_string(),
                )?));
            }

            attr(
                "action",
                "provwasm.contracts.cw20.marker.mint_requested_from",
            )
        }
        RequestType::Burn => {
            // Reduce burn allowance
            if BURN_ALLOWANCES
                .update(
                    deps.storage,
                    &key,
                    |allowance_opt: Option<Uint128>| -> Result<_, ContractError> {
                        match allowance_opt {
                            Some(mut allowance) => Ok({
                                if allowance < amount {
                                    return Err(ContractError::AllowanceTooLow {
                                        allowance,
                                        amount,
                                    });
                                }
                                allowance -= amount;
                                allowance
                            }),
                            None => Err(ContractError::AllowanceNotFound {
                                owner: from.clone(),
                                spender: sender.clone(),
                            }),
                        }
                    },
                )
                .is_err()
            {
                return Ok(Response::new().add_message(create_response(
                    dest_config,
                    contract_address,
                    ContractError::AllowanceNotFound {
                        owner: from,
                        spender: sender,
                    }
                    .to_string(),
                )?));
            }

            let bal = get_consolidated_balance(deps.as_ref(), from.clone())?;
            // Ensuring balance is sufficient
            if let Err(err) =
                check_bal_avalaility(amount, bal, ContractError::BalanceLow { bal, cap: amount })
            {
                return Ok(Response::new().add_message(create_response(
                    dest_config,
                    contract_address,
                    err.to_string(),
                )?));
            };

            msgs.push(transfer_marker_coins(
                amount.u128(),
                denom,
                marker_addr,
                from.clone(),
                contract_address.clone(),
            )?);

            match update_burn_balances(deps.storage, from.clone(), UpdateType::Add(amount)) {
                Ok(_) => (),
                Err(err) => {
                    return Ok(Response::new().add_message(create_response(
                        dest_config,
                        contract_address,
                        err.to_string(),
                    )?))
                }
            };

            attr(
                "action",
                "provwasm.contracts.cw20.marker.burn_requested_from",
            )
        }
    };

    REQUESTS.save(
        deps.storage,
        request_id.as_bytes(),
        &Request {
            request_type,
            status: Status::Pending,
            requester: from.clone(),
            responder: None,
            amount,
        },
    )?;

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attribute])
        .add_attribute("request_id", request_id)
        .add_attribute("from", from)
        .add_attribute("amount", amount))
}

// Function to approve request
fn try_approve_request(
    deps: DepsMut,
    sender: Addr,
    contract_address: Addr,
    request_id: String,
    request_type: RequestType,
) -> Result<Response, ContractError> {
    // Ensuring authorized caller
    is_tokenization_agent(&deps, sender.clone())?;

    let mut requester = Addr::unchecked("");
    let mut amount = Uint128::default();
    let mut msgs = Vec::new();
    let denom = get_denom(deps.storage)?;
    let dest_config = DEST_CONFIG.load(deps.storage)?;

    match request_type {
        RequestType::Mint => {
            // Ensuring valid request exists
            if let Ok(request) = REQUESTS.load(deps.storage, request_id.as_bytes()) {
                if request.status.ne(&Status::Pending) {
                    return Err(ContractError::InvalidRequestStatus {
                        req: request.status.into_string(),
                    });
                }
                if request.request_type.ne(&RequestType::Mint) {
                    return Err(ContractError::InvalidRequestType {
                        typ: request.request_type.into_string(),
                    });
                }
            } else {
                return Err(ContractError::RequestNotExists {
                    request_id: request_id.to_string(),
                });
            }

            REQUESTS.update(
                deps.storage,
                request_id.as_bytes(),
                |request_opt: Option<Request>| -> Result<_, ContractError> {
                    match request_opt {
                        Some(mut request) => Ok({
                            request.status = Status::Approved;
                            request.responder = Some(sender.clone());

                            amount = request.amount;
                            requester = request.requester.clone();

                            msgs = mint_to(
                                denom,
                                Data {
                                    address: request.requester.clone(),
                                    amount,
                                },
                                contract_address.clone(),
                            )?;

                            request
                        }),
                        None => Err(ContractError::RequestNotExists {
                            request_id: request_id.to_string(),
                        }),
                    }
                },
            )?;
        }
        RequestType::Burn => {
            // Ensuring valid request exists
            if let Ok(request) = REQUESTS.load(deps.storage, request_id.as_bytes()) {
                if request.status.ne(&Status::Pending) {
                    return Err(ContractError::InvalidRequestStatus {
                        req: request.status.into_string(),
                    });
                }
                if request.request_type.ne(&RequestType::Burn) {
                    return Err(ContractError::InvalidRequestType {
                        typ: request.request_type.into_string(),
                    });
                }
            } else {
                return Err(ContractError::RequestNotExists {
                    request_id: request_id.to_string(),
                });
            }

            REQUESTS.update(
                deps.storage,
                request_id.as_bytes(),
                |request_opt: Option<Request>| -> Result<_, ContractError> {
                    match request_opt {
                        Some(mut request) => Ok({
                            request.status = Status::Approved;
                            request.responder = Some(sender.clone());

                            amount = request.amount;
                            requester = request.requester.clone();

                            msgs.push(burn_marker_supply(
                                amount.u128(),
                                denom,
                                contract_address.clone(),
                            )?);

                            request
                        }),
                        None => Err(ContractError::RequestNotExists {
                            request_id: request_id.to_string(),
                        }),
                    }
                },
            )?;
        }
    }

    msgs.push(create_response(
        dest_config,
        contract_address,
        request_id.to_string(),
    )?);

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute(
            "action",
            format!(
                "provwasm.contracts.cw20.marker.{}.approved",
                request_type.into_string()
            ),
        )
        .add_attribute("request_id", request_id)
        .add_attribute("requester", requester)
        .add_attribute("responder", sender.into_string())
        .add_attribute("amount", amount))
}

// Function to reject
fn try_reject(
    deps: DepsMut,
    sender: Addr,
    contract_address: Addr,
    request_id: String,
    request_type: RequestType,
) -> Result<Response, ContractError> {
    // Ensuring authorized caller
    is_tokenization_agent(&deps, sender.clone())?;

    let mut requester = Addr::unchecked("");
    let mut amount = Uint128::default();
    let mut msgs = Vec::new();
    let denom = get_denom(deps.storage)?;
    let dest_config = DEST_CONFIG.load(deps.storage)?;

    match request_type {
        RequestType::Mint => {
            // Ensuring valid request exists
            if let Ok(request) = REQUESTS.load(deps.storage, request_id.as_bytes()) {
                if request.status.ne(&Status::Pending) {
                    return Err(ContractError::InvalidRequestStatus {
                        req: request.status.into_string(),
                    });
                }
                if request.request_type.ne(&RequestType::Mint) {
                    return Err(ContractError::InvalidRequestType {
                        typ: request.request_type.into_string(),
                    });
                }
            } else {
                return Err(ContractError::RequestNotExists {
                    request_id: request_id.to_string(),
                });
            }

            REQUESTS.update(
                deps.storage,
                request_id.as_bytes(),
                |request_opt: Option<Request>| -> Result<_, ContractError> {
                    match request_opt {
                        Some(mut request) => Ok({
                            request.status = Status::Rejected;
                            request.responder = Some(sender.clone());

                            amount = request.amount;
                            requester = request.requester.clone();

                            request
                        }),
                        None => Err(ContractError::RequestNotExists {
                            request_id: request_id.to_string(),
                        }),
                    }
                },
            )?;
        }
        RequestType::Burn => {
            // Ensuring valid request exists
            if let Ok(request) = REQUESTS.load(deps.storage, request_id.as_bytes()) {
                if request.status.ne(&Status::Pending) {
                    return Err(ContractError::InvalidRequestStatus {
                        req: request.status.into_string(),
                    });
                }
                if request.request_type.ne(&RequestType::Burn) {
                    return Err(ContractError::InvalidRequestType {
                        typ: request.request_type.into_string(),
                    });
                }
            } else {
                return Err(ContractError::RequestNotExists {
                    request_id: request_id.to_string(),
                });
            }

            REQUESTS.update(
                deps.storage,
                request_id.as_bytes(),
                |request_opt: Option<Request>| -> Result<_, ContractError> {
                    match request_opt {
                        Some(mut request) => Ok({
                            request.status = Status::Rejected;
                            request.responder = Some(sender.clone());

                            amount = request.amount;
                            requester = request.requester.clone();

                            request
                        }),
                        None => Err(ContractError::RequestNotExists {
                            request_id: request_id.to_string(),
                        }),
                    }
                },
            )?;

            update_burn_balances(deps.storage, requester.clone(), UpdateType::Remove(amount))?;

            msgs.push(withdraw_coins(
                denom,
                amount.u128(),
                requester.clone(),
                contract_address.clone(),
            )?);
        }
    }

    msgs.push(create_response(
        dest_config,
        contract_address,
        format!("Request: {} Rejected!", request_id),
    )?);

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute(
            "action",
            format!(
                "provwasm.contracts.cw20.marker.{}.rejected",
                request_type.into_string()
            ),
        )
        .add_attribute("request_id", request_id)
        .add_attribute("requester", requester)
        .add_attribute("responder", sender.into_string())
        .add_attribute("amount", amount))
}

// Function to manage request allowance
fn try_manage_request_allowance(
    deps: DepsMut,
    sender: Addr,
    spender: Addr,
    update_type: UpdateType<Uint128>,
    request_type: RequestType,
) -> Result<Response, ContractError> {
    let key = Key::new(sender, spender.clone()).as_bytes()?;

    let res = match request_type {
        RequestType::Mint => match update_type {
            UpdateType::Add(amount) => {
                if MINT_ALLOWANCES
                    .update(
                        deps.storage,
                        &key,
                        |allowance_opt: Option<Uint128>| -> Result<_, ContractError> {
                            match allowance_opt {
                                Some(mut allowance) => Ok({
                                    allowance += amount;
                                    allowance
                                }),
                                None => Ok(amount),
                            }
                        },
                    )
                    .is_err()
                {
                    MINT_ALLOWANCES.save(deps.storage, &key, &amount)?;
                };
                Response::new()
                    .add_attribute(
                        "action",
                        "provwasm.contracts.cw20.marker.increased_mint_allowance",
                    )
                    .add_attribute("spender", spender)
                    .add_attribute("amount", amount)
            }
            UpdateType::Remove(amount) => {
                MINT_ALLOWANCES.update(
                    deps.storage,
                    &key,
                    |allowance_opt: Option<Uint128>| -> Result<_, ContractError> {
                        match allowance_opt {
                            Some(mut allowance) => Ok({
                                if allowance < amount {
                                    return Err(ContractError::AllowanceTooLow {
                                        allowance,
                                        amount,
                                    });
                                }
                                allowance -= amount;
                                allowance
                            }),
                            None => Ok(amount),
                        }
                    },
                )?;
                Response::new()
                    .add_attribute(
                        "action",
                        "provwasm.contracts.cw20.marker.decreased_mint_allowance",
                    )
                    .add_attribute("spender", spender)
                    .add_attribute("amount", amount)
            }
        },
        RequestType::Burn => match update_type {
            UpdateType::Add(amount) => {
                if BURN_ALLOWANCES
                    .update(
                        deps.storage,
                        &key,
                        |allowance_opt: Option<Uint128>| -> Result<_, ContractError> {
                            match allowance_opt {
                                Some(mut allowance) => Ok({
                                    allowance += amount;
                                    allowance
                                }),
                                None => Ok(amount),
                            }
                        },
                    )
                    .is_err()
                {
                    BURN_ALLOWANCES.save(deps.storage, &key, &amount)?;
                };
                Response::new()
                    .add_attribute(
                        "action",
                        "provwasm.contracts.cw20.marker.increased_burn_allowance",
                    )
                    .add_attribute("spender", spender)
                    .add_attribute("amount", amount)
            }
            UpdateType::Remove(amount) => {
                BURN_ALLOWANCES.update(
                    deps.storage,
                    &key,
                    |allowance_opt: Option<Uint128>| -> Result<_, ContractError> {
                        match allowance_opt {
                            Some(mut allowance) => Ok({
                                if allowance < amount {
                                    return Err(ContractError::AllowanceTooLow {
                                        allowance,
                                        amount,
                                    });
                                }
                                allowance -= amount;
                                allowance
                            }),
                            None => Ok(amount),
                        }
                    },
                )?;
                Response::new()
                    .add_attribute(
                        "action",
                        "provwasm.contracts.cw20.marker.decreased_burn_allowance",
                    )
                    .add_attribute("spender", spender)
                    .add_attribute("amount", amount)
            }
        },
    };

    Ok(res)
}

/// Approve some address to move some tokens on sender's behalf
///
/// The amount approved will replace the existing amount.
fn try_approve(
    deps: DepsMut,
    spender: Addr,
    sender: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let key = Key::new(sender.clone(), spender.clone()).as_bytes()?;

    if ALLOWANCE
        .update(
            deps.storage,
            &key,
            |allowance_opt: Option<Uint128>| -> Result<_, ContractError> {
                match allowance_opt {
                    Some(mut allowance) => Ok({
                        allowance = amount;
                        allowance
                    }),
                    None => Ok(amount),
                }
            },
        )
        .is_err()
    {
        ALLOWANCE.save(deps.storage, &key, &amount)?;
    };

    Ok(Response::new()
        .add_attribute("action", "provwasm.contracts.cw20.marker.approve")
        .add_attribute("owner", sender)
        .add_attribute("spender", spender)
        .add_attribute("amount", amount))
}

// Sends a message via Axelar GMP to the EVM {destination_chain} and {destination_address}
pub fn send_message_evm(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    message: String,
    msg_type: MessageType,
) -> Result<Response, ContractError> {
    // Message payload to be received by the destination
    let message_payload = encode(&[
        Token::String(info.sender.to_string()),
        Token::String(message),
    ]);

    // {info.funds} used to pay gas. Must only contain 1 token type.
    let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

    let gmp_message: GmpMessage = GmpMessage {
        destination_chain,
        destination_address,
        payload: message_payload.to_vec(),
        type_: msg_type.into_i64(),
        fee: None,
    };

    let ibc_message = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: "channel-1".to_string(),
        token: Some(coin.into()),
        sender: env.contract.address.to_string(),
        receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5".to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
        memo: to_string(&gmp_message).unwrap(),
    };

    Ok(Response::new().add_message(ibc_message))
}

// Sends a message via Axelar GMP to the other cosmos chains
// only difference is how the {message_payload} is constructed
pub fn send_message_cosmos(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    message: String,
    msg_type: MessageType,
) -> Result<Response, ContractError> {
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

    let gmp_message: GmpMessage = GmpMessage {
        destination_chain,
        destination_address,
        payload: message_payload.to_vec(),
        type_: msg_type.into_i64(), //type = 1
        fee: None,
    };

    // info.funds used to pay gas. Must only contain 1 token type.
    let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

    let ibc_message = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: "channel-1".to_string(),
        token: Some(coin.into()),
        sender: env.contract.address.to_string(),
        receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5".to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
        memo: to_string(&gmp_message).unwrap(),
    };

    Ok(Response::new().add_message(ibc_message))
}

pub fn receive_message_evm(
    deps: DepsMut,
    _source_chain: String,
    _source_address: String,
    payload: Binary,
) -> Result<Response, ContractError> {
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

    Ok(Response::new())
}

pub fn receive_message_cosmos(
    deps: DepsMut,
    sender: String,
    message: String,
) -> Result<Response, ContractError> {
    // store message
    STORED_MESSAGE.save(deps.storage, &Message { sender, message })?;

    Ok(Response::new())
}

// Rescue unused coins
pub fn try_rescue_coins(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    to_address: String,
    amount: u128,
) -> Result<Response, ContractError> {
    // Ensuring authorized sender
    is_subadmin(&deps, sender)?;

    let msg: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
        to_address,
        amount: vec![coin(amount, denom)],
    });

    Ok(Response::new().add_message(msg))
}

// Update Destination Config
pub fn update_dest_config(deps: DepsMut, config: DestConfig) -> Result<Response, ContractError> {
    // store destination configuration
    DEST_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

// Remove burn balance on an address
fn try_clear_burn_balance(
    deps: DepsMut,
    sender: Addr,
    address: Addr,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    BURN_BALANCES.remove(deps.storage, address.clone());

    let res = Response::new()
        .add_attribute(
            "action",
            "provwasm.contracts.cw20.marker.clear_burn_balance",
        )
        .add_attribute("address", address);

    Ok(res)
}
