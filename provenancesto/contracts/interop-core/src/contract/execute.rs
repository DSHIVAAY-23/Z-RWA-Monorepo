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
    use ExecuteMsg::*;

    match msg {
        ManageRoles { roles } => try_manage_roles(deps, info, roles),
        UpdateSourceChain { chain } => try_update_source_chain(deps, info, chain),
        SendInstruction { params } => {
            try_send_instruction(deps.as_ref(), env.contract.address, info.sender, params)
        }
        ExecuteInstruction {
            source_chain,
            source_address,
            payload,
        } => try_execute_instruction(
            deps.as_ref(),
            info.sender,
            env.contract.address,
            source_chain,
            source_address,
            payload,
        ),
        MintTokens { order } => try_mint_tokens(&deps.as_ref(), info.sender, order),
        BurnTokens { order } => try_burn_tokens(&deps.as_ref(), info.sender, order),
    }
}

/// Function to manage different roles
/// This function can perform batch operations, hence multiple addresses can be added or removed simultaneously.
/// This entry point can be use to modify different roles such as:-
///     - Admins
///     - Executer
/// Based on the `update_type` field addresses can be added or removed:-
///     - for addition `update_type` will be `UpdateType::Add(Addresses)`
///     - for removal `update_type` will be `UpdateType::Remove(Addresses)`
///
/// Arguements:-
///     - List of Roles, can be either Admins or Executer
///     - UpdateType, can be either Add or Remove in case of Admins
///     - List of addresses, in case of Admin update while Address in case of Executer
///
/// Fails when:-
///     - caller is not admin,
///     - ADMINS map is empty in case of admin removals
///
/// Based on operation, any event can be emitted:-
///     - provwasm.contracts.interop_core.add_admins
///     - provwasm.contracts.interop_core.remove_admins
///     - provwasm.contracts.interop_core.update_executer
pub fn try_manage_roles(
    deps: DepsMut,
    info: MessageInfo,
    roles: Vec<Role>,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();
    // Ensuring caller has the admin rights
    is_admin(&deps.as_ref(), info.sender.clone())?;

    for role in roles {
        match role {
            Role::Admins { update_type } => match update_type {
                UpdateType::Add(addrs) => {
                    let updated =
                        ADMINS.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                            addresses.extend(addrs.clone());
                            addresses.sort();
                            addresses.dedup();
                            Ok(addresses)
                        });
                    if updated.is_err() {
                        ADMINS.save(deps.storage, &addrs)?;
                    };
                    attrs.push(attr("action", "provwasm.contracts.interop_core.add_admins"));
                }
                UpdateType::Remove(addrs) => {
                    ADMINS.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                        addresses.retain(|addr| !addrs.contains(addr));
                        Ok(addresses)
                    })?;
                    attrs.push(attr(
                        "action",
                        "provwasm.contracts.interop_core.remove_admins",
                    ));
                }
            },
            Role::Executer { addr } => {
                EXECUTER.save(deps.storage, &addr)?;
                attrs.push(attr(
                    "action",
                    "provwasm.contracts.interop_core.update_executer",
                ));
            }
        }
    }

    Ok(Response::new().add_attributes(attrs))
}

/// Function to update source chain configuration
///
/// Arguements:-
///     - Chain Name
///
/// Fails when:-
///     - caller is not admin
///
/// Event to be emitted:-
///     - provwasm.contracts.interop_core.updated_source_chain
pub fn try_update_source_chain(
    deps: DepsMut,
    info: MessageInfo,
    chain: String,
) -> Result<Response, ContractError> {
    // Ensuring caller has the admin rights
    is_admin(&deps.as_ref(), info.sender.clone())?;

    SOURCE_CHAIN.save(deps.storage, &chain)?;

    Ok(Response::new().add_attribute(
        "action",
        "provwasm.contracts.interop_core.updated_source_chain",
    ))
}

/// Function to send instruction
/// This function can perform batch operations, hence multiple encrypted messages can be sent simultaneously.
///
/// Arguements:-
///     - List of Potfolios containing:-
///         1. Destination Chain
///         2. Destination Address
///         3. Investor Address
///         4. Token Address
///         5. Amount of Tokens
///         6. Order Id
///         7. Action, can be either mint, burn or acknowledgement
///
/// Fails when:-
///     - source chain config is not set
///
/// Event containing following fields will be emitted:-
///     - source chain
///     - source address
///     - destination chain
///     - destination address
///     - sender address
///     - encrypted payload data
pub fn try_send_instruction(
    deps: Deps,
    contract: Addr,
    sender: Addr,
    params: SendParams,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();
    let source_chain = SOURCE_CHAIN.load(deps.storage)?;

    for portfolio in params.portfolios {
        let payload = encode(&[
            Token::Uint(portfolio.action.to_other_uint()),
            Token::Address(Address::from_str(&portfolio.investor)?),
            Token::Uint(Uint::from(portfolio.amount)),
            Token::Address(Address::from_str(&portfolio.token)?),
            Token::String(contract.to_string()),
            Token::Uint(Uint::from(portfolio.order_id)),
        ]);

        attrs.append(
            &mut (contract_call_attributes(
                source_chain.clone(),
                contract.to_string(),
                portfolio.dest_chain,
                portfolio.dest_address,
                sender.clone(),
                payload,
            ))?,
        );
        attrs.push(attr("action", portfolio.action.to_event()))
    }

    Ok(Response::new().add_attributes(attrs))
}

/// Function to execute instruction
///
/// Arguements:-
///     - Source Chain
///     - Source Address
///     - Payload Data
///
/// Fails when:-
///     - caller is not executer
///     - source chain config is not set
///     - payload bytes contains garbage values
///     - action = acknowledgement during successful message decode
///     - payload encoding fails during interop smart contract calls
///     - uint fields like action, amount and order_id are failed to parsed
///     - action type is not acknowledgement while dealing with acknowledgement
///     - executer is not set
///
/// Event emitted:-
///     - provwasm.contracts.interop_core.execute_intruction
pub fn try_execute_instruction(
    deps: Deps,
    sender: Addr,
    contract: Addr,
    source_chain: String,
    source_address: String,
    payload: String,
) -> Result<Response, ContractError> {
    // Ensuring Authorised Sender
    is_executer(&deps, sender.clone())?;

    let mut msgs = Vec::new();
    let mut attrs = Vec::new();
    let payload_bytes = hex::decode(payload)?;
    let deployed_chain = SOURCE_CHAIN.load(deps.storage)?;

    if let Ok(decoded) = decode(
        &[
            ParamType::Uint(4),
            ParamType::String,
            ParamType::Uint(4),
            ParamType::String,
            ParamType::String,
            ParamType::Uint(4),
        ],
        &payload_bytes,
    ) {
        let action_uint = decoded[0]
            .clone()
            .into_uint()
            .ok_or(ContractError::ConversionError {
                err: ErrorType::Uint.to_string(),
            })?;
        let action = Action::from_u32(action_uint.as_u32());

        let investor = Addr::unchecked(decoded[1].to_string());

        let amount_uint = decoded[2]
            .clone()
            .into_uint()
            .ok_or(ContractError::ConversionError {
                err: ErrorType::Uint.to_string(),
            })?;
        let amount = Uint128::from(amount_uint.as_u128());

        let denom = decoded[3].to_string();
        let _from = decoded[4].to_string();

        let order_id_uint =
            decoded[5]
                .clone()
                .into_uint()
                .ok_or(ContractError::ConversionError {
                    err: ErrorType::Uint.to_string(),
                })?;
        let order_id = order_id_uint.as_u128();

        match action {
            Action::Burn => {
                let exe_msg = cw20_marker::msg::ExecuteMsg::RequestOrder {
                    order_id: order_id.to_string(),
                    denom: denom.to_string(),
                    from: investor,
                    amount,
                    request_type: cw20_marker::enums::RequestType::Burn,
                };
                let msg: CosmosMsg =
                    CosmosMsg::Wasm(wasm_execute(TOKEN_CONTRACT, &exe_msg, vec![])?);
                msgs.push(msg);
            }
            Action::Mint => {
                let exe_msg = cw20_marker::msg::ExecuteMsg::RequestOrder {
                    order_id: order_id.to_string(),
                    denom: denom.to_string(),
                    from: investor,
                    amount,
                    request_type: cw20_marker::enums::RequestType::Mint,
                };
                let msg: CosmosMsg =
                    CosmosMsg::Wasm(wasm_execute(TOKEN_CONTRACT, &exe_msg, vec![])?);
                msgs.push(msg);
            }
            Action::Ack => return Err(ContractError::InvalidAction {}),
        }

        let ack = encode(&[
            Token::Uint(Action::Ack.to_other_uint()),
            Token::Uint(Uint::from(order_id)),
        ]);
        attrs.append(
            &mut (contract_call_attributes(
                deployed_chain,
                contract.to_string(),
                source_chain,
                source_address,
                sender,
                ack,
            ))?,
        );
    } else if let Ok(decoded) = decode(&[ParamType::Uint(4), ParamType::Uint(4)], &payload_bytes) {
        let action_uint = decoded[0]
            .clone()
            .into_uint()
            .ok_or(ContractError::ConversionError {
                err: ErrorType::Uint.to_string(),
            })?;
        let action = Action::from_u32(action_uint.as_u32());

        let order_id_uint =
            decoded[1]
                .clone()
                .into_uint()
                .ok_or(ContractError::ConversionError {
                    err: ErrorType::Uint.to_string(),
                })?;
        let order_id = order_id_uint.as_u128();

        // Ensuring valid action
        ensure!(action.eq(&Action::Ack), ContractError::InvalidAction {});

        let ack = encode(&[
            Token::Uint(action.to_other_uint()),
            Token::Uint(Uint::from(order_id)),
        ]);
        attrs.append(
            &mut (contract_call_attributes(
                deployed_chain,
                contract.into_string(),
                source_chain,
                source_address,
                sender,
                ack,
            ))?,
        );
    } else {
        return Err(ContractError::InvalidAction {});
    };

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute(
            "action",
            "provwasm.contracts.interop_core.execute_intruction",
        )
        .add_attributes(attrs))
}

/// Function to mint tokens
///
/// Arguements:-
///     - Order Id
///     - Denom
///     - User Address
///     - Amount of tokens
///
/// Fails when:-
///     - payload encoding fails during interop smart contract calls
///
/// Event emitted:-
///     - provwasm.contracts.interop_core.token_minted
pub fn try_mint_tokens(deps: &Deps, sender: Addr, order: Order) -> Result<Response, ContractError> {
    // Ensuring caller has the admin rights
    is_admin(deps, sender.clone())?;

    let mut msgs = Vec::new();

    let exe_msg = cw20_marker::msg::ExecuteMsg::RequestOrder {
        order_id: order.order_id.to_string(),
        denom: order.denom.to_string(),
        from: order.user,
        amount: order.amount,
        request_type: cw20_marker::enums::RequestType::Mint,
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(TOKEN_CONTRACT, &exe_msg, vec![])?);
    msgs.push(msg);

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "provwasm.contracts.interop_core.token_minted"))
}

/// Function to burn tokens
///
/// Arguements:-
///     - Order Id
///     - Denom
///     - User Address
///     - Amount of tokens
///
/// Fails when:-
///     - payload encoding fails during interop smart contract calls
///
/// Event emitted:-
///     - provwasm.contracts.interop_core.token_burned
pub fn try_burn_tokens(deps: &Deps, sender: Addr, order: Order) -> Result<Response, ContractError> {
    // Ensuring caller has the admin rights
    is_admin(deps, sender.clone())?;

    let mut msgs = Vec::new();

    let exe_msg = cw20_marker::msg::ExecuteMsg::RequestOrder {
        order_id: order.order_id.to_string(),
        denom: order.denom.to_string(),
        from: order.user,
        amount: order.amount,
        request_type: cw20_marker::enums::RequestType::Burn,
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(TOKEN_CONTRACT, &exe_msg, vec![])?);
    msgs.push(msg);

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "provwasm.contracts.interop_core.token_burned"))
}
