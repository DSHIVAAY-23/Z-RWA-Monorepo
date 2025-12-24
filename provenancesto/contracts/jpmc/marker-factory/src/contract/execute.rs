use super::*;

/// Handle messages that create and interact with cw20 marker contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        DeployToken { msg } => try_deploy_token_contract(deps, info.sender, msg),
        TransferFrom {
            denom,
            amount,
            from,
            to,
        } => try_transfer_from(deps, info.sender, denom, amount, from, to),
        Freeze { denom, update_type } => {
            try_update_freezelist(deps, info.sender, denom, update_type)
        }
        PartialFreeze { denom, params } => {
            try_update_partial_freeze(deps, info.sender, denom, params)
        }
        ManageRoles { roles } => try_manage_roles(deps, info.sender, roles),
        Approve {
            denom,
            spender,
            amount,
        } => try_approve(deps, info.sender, denom, spender, amount),
        Request {
            denom,
            request_id,
            amount,
            request_type,
        } => try_request(deps, info.sender, denom, request_id, amount, request_type),
        RequestFrom {
            denom,
            request_id,
            from,
            amount,
            request_type,
        } => try_request_from(
            deps,
            info.sender,
            denom,
            request_id,
            from,
            amount,
            request_type,
        ),
        ApproveRequest {
            denom,
            request_id,
            request_type,
        } => try_approve_request(deps, info.sender, denom, request_id, request_type),
        RejectRequest {
            denom,
            request_id,
            request_type,
        } => try_reject_request(deps, info.sender, denom, request_id, request_type),
        ManageRequestAllowance {
            denom,
            spender,
            update_type,
            request_type,
        } => try_manage_request_allowance(
            deps,
            info.sender,
            denom,
            spender,
            update_type,
            request_type,
        ),
        UpdateCode { code_id } => try_update_code(deps, info.sender, code_id),
        UpgradeContract { contract_address } => {
            try_migrate_contracts(deps, info.sender, contract_address)
        }
        RescueCoins {
            denom,
            target_denom,
            to_address,
            amount,
        } => try_rescue_coins(deps, info.sender, denom, target_denom, to_address, amount),
    }
}

// Deploy new token contract
fn try_deploy_token_contract(
    deps: DepsMut,
    sender: Addr,
    msg: cw20_marker::msg::InitMsg,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender.clone())?;

    let code_id = get_code_id(deps.storage)?;

    DENOM.save(deps.storage, &msg.denom)?;

    let msg = WasmMsg::Instantiate {
        admin: Some(sender.to_string()),
        code_id,
        msg: to_binary(&msg)?,
        funds: Vec::default(),
        label: LABEL.to_string(),
    };

    // Creating a submessage that wraps the message above
    let sub_msg = SubMsg::reply_on_success(msg, INSTANTIATE_REPLY_ID);

    Ok(Response::new()
        .add_submessage(sub_msg)
        .add_attribute("action", "marker.factory.deploy.token"))
}

// Function to transfer tokens from an address
fn try_transfer_from(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    amount: Uint128,
    from: Addr,
    to: Addr,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let contract = get_contract_by_denom(deps.storage, denom.clone())?;

    let exe_msg = cw20_marker::msg::ExecuteMsg::TransferFrom { amount, from, to };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(contract.clone(), &exe_msg, Vec::default())?);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "marker.factory.transfer_from")
        .add_attribute("denom", denom)
        .add_attribute("contract", contract))
}

// Update Freeze List.
fn try_update_freezelist(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    update_type: UpdateType<Vec<Addr>>,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let contract = get_contract_by_denom(deps.storage, denom.clone())?;

    let exe_msg = cw20_marker::msg::ExecuteMsg::Freeze { update_type };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(contract.clone(), &exe_msg, Vec::default())?);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "marker.factory.update_freezelist")
        .add_attribute("denom", denom)
        .add_attribute("contract", contract))
}

// Update Partial Freeze.
fn try_update_partial_freeze(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    params: Vec<PartialFreezeParams>,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let contract = get_contract_by_denom(deps.storage, denom.clone())?;

    let exe_msg = cw20_marker::msg::ExecuteMsg::PartialFreeze { params };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(contract.clone(), &exe_msg, Vec::default())?);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "marker.factory.update_partial_freeze")
        .add_attribute("denom", denom)
        .add_attribute("contract", contract))
}

// Function to manage different roles
pub fn try_manage_roles(
    deps: DepsMut,
    sender: Addr,
    roles: Vec<Role>,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();
    let mut msgs = Vec::new();

    for role in roles {
        match role {
            Role::SubAdmin { update_type } => {
                // Ensuring caller has the admin rights
                is_admin(&deps, sender.clone())?;

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
                        attrs.push(attr("action", "marker.factory.add_sub_admin"));
                    }
                    UpdateType::Remove(addrs) => {
                        SUB_ADMIN.update(
                            deps.storage,
                            |mut addresses| -> Result<_, ContractError> {
                                addresses.retain(|addr| !addrs.contains(addr));
                                Ok(addresses)
                            },
                        )?;
                        attrs.push(attr("action", "marker.factory.remove_sub_admin"));
                    }
                }
            }
            Role::Admin { address } => {
                // Ensuring caller has the admin rights
                is_admin(&deps, sender.clone())?;

                ADMIN.save(deps.storage, &address)?;
                attrs.push(attr("action", "marker.factory.update_admin"));
            }
            Role::TokenSubAdmin { denom, update_type } => {
                let contract = get_contract_by_denom(deps.storage, denom.clone())?;
                let exe_msg = cw20_marker::msg::ExecuteMsg::ManageRoles {
                    roles: vec![cw20_marker::enums::Role::SubAdmin { update_type }],
                };
                msgs.push(CosmosMsg::Wasm(wasm_execute(
                    contract.clone(),
                    &exe_msg,
                    Vec::default(),
                )?));
            }
        }
    }

    Ok(Response::new().add_messages(msgs).add_attributes(attrs))
}

// Function to approve allowance
pub fn try_approve(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    spender: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let contract = get_contract_by_denom(deps.storage, denom.clone())?;

    let exe_msg = cw20_marker::msg::ExecuteMsg::Approve { spender, amount };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(contract.clone(), &exe_msg, Vec::default())?);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "marker.factory.approve")
        .add_attribute("denom", denom)
        .add_attribute("contract", contract))
}

// Function for request
pub fn try_request(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    request_id: String,
    amount: Uint128,
    request_type: cw20_marker::enums::RequestType,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let contract = get_contract_by_denom(deps.storage, denom.clone())?;

    let exe_msg = cw20_marker::msg::ExecuteMsg::Request {
        request_id,
        amount,
        request_type,
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(contract.clone(), &exe_msg, Vec::default())?);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "marker.factory.request")
        .add_attribute("denom", denom)
        .add_attribute("contract", contract))
}

// Function to request from an address
pub fn try_request_from(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    request_id: String,
    from: Addr,
    amount: Uint128,
    request_type: cw20_marker::enums::RequestType,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let contract = get_contract_by_denom(deps.storage, denom.clone())?;

    let exe_msg = cw20_marker::msg::ExecuteMsg::RequestFrom {
        request_id,
        from,
        amount,
        request_type,
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(contract.clone(), &exe_msg, Vec::default())?);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "marker.factory.request_from")
        .add_attribute("denom", denom)
        .add_attribute("contract", contract))
}

// Function to approve request
pub fn try_approve_request(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    request_id: String,
    request_type: cw20_marker::enums::RequestType,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let contract = get_contract_by_denom(deps.storage, denom.clone())?;

    let exe_msg = cw20_marker::msg::ExecuteMsg::ApproveRequest {
        request_id,
        request_type,
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(contract.clone(), &exe_msg, Vec::default())?);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "marker.factory.approve_request")
        .add_attribute("denom", denom)
        .add_attribute("contract", contract))
}

// Function to reject request
pub fn try_reject_request(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    request_id: String,
    request_type: cw20_marker::enums::RequestType,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let contract = get_contract_by_denom(deps.storage, denom.clone())?;

    let exe_msg = cw20_marker::msg::ExecuteMsg::RejectRequest {
        request_id,
        request_type,
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(contract.clone(), &exe_msg, Vec::default())?);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "marker.factory.reject_request")
        .add_attribute("denom", denom)
        .add_attribute("contract", contract))
}

// Function to manage request allowance
pub fn try_manage_request_allowance(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    spender: Addr,
    update_type: UpdateType<Uint128>,
    request_type: cw20_marker::enums::RequestType,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let contract = get_contract_by_denom(deps.storage, denom.clone())?;

    let exe_msg = cw20_marker::msg::ExecuteMsg::ManageRequestAllowance {
        spender,
        update_type,
        request_type,
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(contract.clone(), &exe_msg, Vec::default())?);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "marker.factory.manage_request_allowance")
        .add_attribute("denom", denom)
        .add_attribute("contract", contract))
}

// Update token contract code
fn try_update_code(deps: DepsMut, sender: Addr, code_id: u64) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    CODE_ID.save(deps.storage, &code_id)?;

    Ok(Response::new()
        .add_attribute("action", "marker.factory.update.code")
        .add_attribute("code_id", code_id.to_string()))
}

// Migrating token contracts
fn try_migrate_contracts(
    deps: DepsMut,
    sender: Addr,
    contract_addr: String,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let new_code_id = get_code_id(deps.storage)?;
    let mig_msg = cw20_marker::msg::MigrateMsg {};

    let msg = WasmMsg::Migrate {
        contract_addr,
        new_code_id,
        msg: to_binary(&mig_msg)?,
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "marker.factory.migrate.contracts"))
}

// Function to manage request allowance
pub fn try_rescue_coins(
    deps: DepsMut,
    sender: Addr,
    denom: String,
    target_denom: String,
    to_address: String,
    amount: u128,
) -> Result<Response, ContractError> {
    // checking caller is sub_admin
    is_subadmin(&deps, sender)?;

    let contract = get_contract_by_denom(deps.storage, denom.clone())?;

    let exe_msg = cw20_marker::msg::ExecuteMsg::RescueCoins {
        denom: target_denom,
        to_address,
        amount,
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(contract.clone(), &exe_msg, Vec::default())?);

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "marker.factory.rescue_coin")
        .add_attribute("denom", denom)
        .add_attribute("contract", contract))
}
