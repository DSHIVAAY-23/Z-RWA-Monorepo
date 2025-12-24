#[cfg(not(feature = "library"))]
use super::*;

/// Handle execute messages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Create { params } => try_create(deps, info, env, params),
        ExecuteMsg::ManageAdmins { update_type } => try_manage_admins(deps, info, update_type),
        ExecuteMsg::ManageAgent { denom, update_type } => {
            try_manage_agent(deps, info, denom, update_type)
        }
        ExecuteMsg::ManagementFees {
            denom,
            managed_users,
        } => try_user_management_fees(deps, info, denom, managed_users),
        ExecuteMsg::ShareDividend {
            denom,
            coin_type,
            shared_dividends,
        } => try_share_dividend(deps, info, denom, coin_type, shared_dividends),
        ExecuteMsg::DistributeAndBurn {
            denom,
            coin_type,
            distributions,
        } => try_distribute_and_burn(deps, info, denom, coin_type, distributions),
        ExecuteMsg::FetchPrice { denom } => try_fetch_price(deps, info, denom),
        ExecuteMsg::UpdateCurrency { denom, ccy } => try_update_ccy(deps, info, denom, ccy),
        ExecuteMsg::SendStableCoins { denom } => try_send_stable_coins(deps, info, denom),
    }
}

/// Function use to manage admins.
/// This function supports batch operations, i.e. multiple addresses can be added / removed simultaneously.
/// For addition of new addresses as admin, `update_type` will be `UpdateType::Add(Vec<Addr>)`
/// For removal of old addresses as admin, `update_type` will be `UpdateType::Remove(Vec<Addr>)`.
///
/// Fails when:-
///     - caller is not admin
///     - address is already having admin rights during addition
///     - address doesn't have admin rights during removal
///
/// Events:-
///     case 1. During addition, i.e., update_type = UpdateType::Add(Vec<Addr>)
///         then
///             provwasm.contracts.fund.add_admin
///     case 2. During removal, i.e., update_type = UpdateType::Remove(Vec<Addr>)
///         then
///             provwasm.contracts.fund.remove_admin
fn try_manage_admins(
    deps: DepsMut,
    info: MessageInfo,
    update_type: UpdateType<Vec<Addr>>,
) -> Result<Response, ContractError> {
    // Ensuring authorised sender
    is_admin(deps.storage, info.sender)?;

    let mut attrs = Vec::new();

    match update_type {
        UpdateType::Add(addrs) => {
            let updated = ADMIN.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                addresses.extend(addrs.clone());

                // Removing duplicate entries, if any
                addresses.sort();
                addresses.dedup();

                Ok(addresses)
            });
            if updated.is_err() {
                ADMIN.save(deps.storage, &addrs)?;
            };
            attrs.push(attr("action", "provwasm.contracts.fund.add_admin"));
        }
        UpdateType::Remove(addrs) => {
            ADMIN.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                addresses.retain(|addr| !addrs.contains(addr));
                Ok(addresses)
            })?;
            attrs.push(attr("action", "provwasm.contracts.fund.remove_admin"));
        }
        UpdateType::Update(_) => (),
    }

    let res = Response::new().add_attributes(attrs);

    Ok(res)
}

/// Function to create new fund contract for a particular token.
/// In this function call:-
///     - the creator is assigned as agent
///     - the contract address will be assigned mint, burn, transfer and force_transfer accesses, in token contract
///
/// Notes
/// Before calling the `create` function, the fund contract address must be assigned as sub_admin in token contract, as
/// sub_admin in token contract have all the mint, burn, transfer and force_transfer accesses.
///
/// Events:-
///     - provwasm.contracts.fund.create
///     - denom
///     - fund_name
///     - asset_type
///     - issuer_name
///     - target_aum
///     - nav_launch_price
///     - ccy
fn try_create(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    params: CreateParams,
) -> Result<Response, ContractError> {
    let global_config = GlobalConfig::new(params.clone(), env.block.time.seconds());
    GLOBAL_CONFIG.save(deps.storage, params.denom.as_bytes(), &global_config)?;

    // Adding sender as agent
    AGENT.save(deps.storage, params.denom.as_bytes(), &info.sender)?;

    // Providing mint, burn, transfer and force transfer accesses to this contract address
    let execution_msg = custom_marker::msg::ExecuteMsg::ManageRoles {
        denom: params.denom.to_string(),
        roles: vec![custom_marker::enums::Role::Agent {
            update_type: custom_marker::enums::UpdateType::Add(vec![info.sender]),
            marker_access: vec![
                custom_marker::enums::AccessControls::Mint,
                custom_marker::enums::AccessControls::Burn,
                custom_marker::enums::AccessControls::Transfer,
                custom_marker::enums::AccessControls::ForceTransfer,
            ],
        }],
    };
    let msg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: MARKER_CONTRACT_ADDRESS.to_string(),
        msg: to_json_binary(&execution_msg)?,
        funds: Vec::new(),
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.fund.create")
        .add_attribute("denom", &params.denom)
        .add_attribute("fund_name", &params.fund_name)
        .add_attribute("asset_type", params.asset_type.as_str())
        .add_attribute("issuer_name", params.issuer_name)
        .add_attribute("target_aum", params.target_aum)
        .add_attribute("nav_launch_price", params.nav_launch_price)
        .add_attribute("ccy", params.ccy))
}

/// Function to manage management fees.
/// For addition of new users, `update_type` will be `UpdateType::Add(Vec<Addr>)`,
/// For updation of old users, `update_type` will be `UpdateType::Update(Vec<Addr>)`,
/// For removal of old users, `update_type` will be `UpdateType::Remove(Vec<Addr>)`
/// This function supports batch operations, i.e. multiple addresses can be added / removed simultaneously.
///
/// Fails when:-
///     - caller is not admin
///     - address is already having agent rights during addition
///     - address doesn't have agent rights during removal
///
/// Events:-
///     case 1. During addition, i.e., update_type = UpdateType::Add(Vec<Addr>)
///         then
///             provwasm.contracts.fund.add_management_fees
///     case 2. During update, i.e., update_type = UpdateType::Update(Vec<Addr>)
///         then
///             provwasm.contracts.fund.update_management_fees
///     case 3. During removal, i.e., update_type = UpdateType::Remove(Vec<Addr>)
///         then
///             provwasm.contracts.fund.remove_management_fees
fn try_user_management_fees(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    managed_users: UpdateType<Vec<ManagedUser>>,
) -> Result<Response, ContractError> {
    // Ensuring authorised sender
    is_agent(deps.storage, denom.clone(), info.sender)?;

    let mut attrs = Vec::new();

    match managed_users {
        UpdateType::Add(users) => {
            for user in users {
                add_user_management_fees(deps.storage, denom.clone(), user)?;
            }
            attrs.push(attr(
                "action",
                "provwasm.contracts.fund.add_management_fees",
            ));
        }
        UpdateType::Update(users) => {
            for user in users {
                update_user_management_fees(deps.storage, denom.clone(), user)?;
            }
            attrs.push(attr(
                "action",
                "provwasm.contracts.fund.update_management_fees",
            ));
        }
        UpdateType::Remove(users) => {
            for user in users {
                remove_user_management_fees(deps.storage, denom.clone(), user)?;
            }
            attrs.push(attr(
                "action",
                "provwasm.contracts.fund.remove_management_fees",
            ));
        }
    }

    let res = Response::new().add_attributes(attrs);

    Ok(res)
}

/// Function for share dividend
/// Stable coins must be transferred from `from` account to agent account before this function call.
/// For dividend share in token, `asset_type` will be `token`, for dividend share in stable coins, `asset_type` will be
/// `stable_coin` and for dividend share in fiat, `asset_type` will be `fiat`.
/// This function supports batch operations, i.e. multiple dividends can be shared simultaneously.
///
/// Notes
///     - Dividend stored during this function call is not used anywhere at the moment, that will be used in future
///       verisons.
///     - Logic for Fiat is not implemented yet, which is subjected to be added on future versions.
///
/// Fails when:-
///     - caller is not agent
///     - agent doesn't have the tokens for the token transactions
///     - agent doesn't have the stable coins for the stable coins
///
/// Events:-
///     case 1. When asset_type = token
///         then
///             provwasm.contracts.fund.share_dividend.token
///     case 2. When asset_type = stable_coin
///         then
///             provwasm.contracts.fund.share_dividend.stable_coin
///     case 3. When asset_type = fiat
///         then
///             provwasm.contracts.fund.share_dividend.fiat
fn try_share_dividend(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    coin_type: CoinType,
    shared_dividends: Vec<SharedDividend>,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();
    let mut msgs = Vec::new();
    let mut key;

    // Ensuring authorised sender
    is_agent(deps.storage, denom.clone(), info.sender)?;

    for shared_dividend in shared_dividends {
        key = Key::new(denom.to_string(), shared_dividend.to.clone()).as_bytes()?;
        match shared_dividend.asset_type {
            AssetType::Token => {
                // Minting tokens in case of asset_type = token
                let execution_msg = custom_marker::msg::ExecuteMsg::MintTo {
                    mint_to_params: vec![custom_marker::structs::MintBurnParams {
                        denom: denom.clone(),
                        mint_burn_data: vec![custom_marker::structs::MintBurnData {
                            address: shared_dividend.to.clone(),
                            amount: shared_dividend.dividend,
                        }],
                    }],
                };

                let msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                    contract_addr: MARKER_CONTRACT_ADDRESS.to_string(),
                    msg: to_json_binary(&execution_msg)?,
                    funds: Vec::new(),
                });

                // Storing dividend share, not using at the moment
                DIVIDEND.save(
                    deps.storage,
                    &key,
                    &Dividend::Token(shared_dividend.dividend),
                )?;
                msgs.push(msg);
                attrs.push(attr(
                    "action",
                    "provwasm.contracts.fund.share_dividend.token",
                ))
            }
            AssetType::StableCoin => {
                // Transferring stable coins from agent account
                let msg: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
                    to_address: shared_dividend.to.to_string(),
                    amount: vec![coin(shared_dividend.dividend.u128(), coin_type.get_denom())],
                });
                msgs.push(msg);

                // Storing dividend share, not using at the moment
                DIVIDEND.save(
                    deps.storage,
                    &key,
                    &Dividend::StableCoin(shared_dividend.dividend),
                )?;
                attrs.push(attr(
                    "action",
                    "provwasm.contracts.fund.share_dividend.stable_coin",
                ));
            }
            AssetType::Fiat => {
                // Storing dividend share, not using at the moment
                DIVIDEND.save(
                    deps.storage,
                    &key,
                    &Dividend::Fiat(shared_dividend.dividend),
                )?;
                attrs.push(attr(
                    "action",
                    "provwasm.contracts.fund.share_dividend.fiat",
                ))
            }
        };
    }

    let res = Response::new().add_attributes(attrs).add_messages(msgs);

    Ok(res)
}

/// Function for Distribute and Burn
/// Stable coins must be transferred from `from` account to agent account before this function call
/// This function is used to exchange stable coins with the a particular token holding by the investor. The stable
/// coins are transferred from agent account to the investor's account and tokens are burnt from the investor's
/// accounts.
/// This function supports batch operations, i.e. multiple distributions can happen simultaneously.
///
/// Fails when:-
///     - caller is not agent
///     - investor doesn't have the tokens
///     - agent doesn't have the stable coins
///
/// Event:-
///     provwasm.contracts.fund.distribute_and_burn
fn try_distribute_and_burn(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    coin_type: CoinType,
    distributions: Vec<Distribution>,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();
    let mut msgs = Vec::new();

    // Ensuring authorised sender
    is_agent(deps.storage, denom.clone(), info.sender)?;

    for distribution in distributions {
        // Calling Burn function from marker contract
        let execution_msg = custom_marker::msg::ExecuteMsg::BurnFrom {
            burn_from_params: vec![custom_marker::structs::MintBurnParams {
                denom: denom.clone(),
                mint_burn_data: vec![custom_marker::structs::MintBurnData {
                    address: distribution.investor.clone(),
                    amount: distribution.token,
                }],
            }],
        };

        let msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: MARKER_CONTRACT_ADDRESS.to_string(),
            msg: to_json_binary(&execution_msg)?,
            funds: Vec::new(),
        });
        msgs.push(msg);

        // Transfer stable coins from agent account
        let msg: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
            to_address: distribution.investor.to_string(),
            amount: vec![coin(distribution.amount.u128(), coin_type.get_denom())],
        });
        msgs.push(msg);

        attrs.push(attr(
            "action",
            "provwasm.contracts.fund.distribute_and_burn",
        ))
    }

    let res = Response::new().add_attributes(attrs).add_messages(msgs);

    Ok(res)
}

/// Function to manage Agent
/// For addition of new address as agent, `update_type` will be `UpdateType::Add(Addr)`,
/// For removal of old address as admin, `update_type` will be `UpdateType::Remove(Addr)`.
///
/// Fails when:-
///     - caller is not admin
///     - address is already having agent rights during addition
///     - address doesn't have agent rights during removal
///
/// Events:-
///     case 1. During addition, i.e., update_type = UpdateType::Add(Addr)
///         then
///             provwasm.contracts.fund.add_agent
///     case 2. During removal, i.e., update_type = UpdateType::Remove(Addr)
///         then
///             provwasm.contracts.fund.remove_agent
fn try_manage_agent(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    update_type: UpdateType<Addr>,
) -> Result<Response, ContractError> {
    // Ensuring authorised sender
    is_admin(deps.storage, info.sender)?;

    let mut attrs = Vec::new();

    match update_type {
        UpdateType::Add(addr) => {
            if !AGENT.has(deps.storage, denom.as_bytes()) {
                AGENT.save(deps.storage, denom.as_bytes(), &addr)?;
            } else {
                return Err(ContractError::AlreadyExists { addr });
            }
            attrs.push(attr("action", "provwasm.contracts.fund.add_agent"));
        }
        UpdateType::Remove(addr) => {
            if let Ok(agent) = AGENT.load(deps.storage, denom.as_bytes()) {
                if agent.eq(&addr) {
                    AGENT.remove(deps.storage, denom.as_bytes());
                } else {
                    return Err(ContractError::NotFound { addr });
                }
            } else {
                return Err(ContractError::NotFound { addr });
            }
            attrs.push(attr("action", "provwasm.contracts.fund.remove_agent"));
        }
        UpdateType::Update(_) => (),
    }
    let res = Response::new().add_attributes(attrs);

    Ok(res)
}

/// Function to fetch price
///
/// Notes
/// This function is not used anywhere at the moment, that will be used in future verisons.
///
/// Fails when:-
///     - caller is not admin
///
/// Event:-
///     provwasm.contracts.fund.fetch_price
fn try_fetch_price(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
) -> Result<Response, ContractError> {
    // Ensuring authorised sender
    is_admin(deps.storage, info.sender)?;

    let global_config: GlobalConfig = GLOBAL_CONFIG.load(deps.storage, denom.as_bytes())?;
    let mut msgs = Vec::new();

    // Calling request price from price feed contract
    let execution_msg = price_feed::msg::ExecuteMsg::Request {
        symbols: vec![global_config.ccy.to_string()],
    };

    let msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: ORACLE_CONTRACT_ADDRESS.to_string(),
        msg: to_json_binary(&execution_msg)?,
        funds: info.funds.clone(),
    });
    msgs.push(msg);

    // Calling get rate from price feed contract
    let query_msg = price_feed::msg::QueryMsg::GetRate {
        symbol: global_config.ccy,
    };
    let rate: price_feed::state::Rate = deps
        .querier
        .query_wasm_smart(ORACLE_CONTRACT_ADDRESS, &query_msg)?;

    GLOBAL_CONFIG.update(
        deps.storage,
        denom.as_bytes(),
        |global_opt: Option<GlobalConfig>| -> Result<_, ContractError> {
            match global_opt {
                Some(mut global) => Ok({
                    global.nav_latest_price = Uint128::from(rate.rate);
                    global
                }),
                None => Err(ContractError::DeserializationFailed {}),
            }
        },
    )?;

    let msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: ORACLE_CONTRACT_ADDRESS.to_string(),
        msg: to_json_binary(&execution_msg)?,
        funds: info.funds,
    });
    msgs.push(msg);

    let res = Response::new()
        .add_messages(msgs)
        .add_attribute("action", "provwasm.contracts.fund.fetch_price");

    Ok(res)
}

/// Function to update currency
///
/// Notes
///     This function is not used anywhere at the moment, that will be used in future verisons.
///
/// Fails when:-
///     caller is not admin
///
/// Event:-
///     provwasm.contracts.fund.update_ccy
fn try_update_ccy(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    ccy: String,
) -> Result<Response, ContractError> {
    // Ensuring authorised sender
    is_agent(deps.storage, denom.to_string(), info.sender)?;

    GLOBAL_CONFIG.update(
        deps.storage,
        denom.as_bytes(),
        |global_opt: Option<GlobalConfig>| -> Result<_, ContractError> {
            match global_opt {
                Some(mut global) => Ok({
                    global.ccy = ccy;
                    global
                }),
                None => Err(ContractError::DeserializationFailed {}),
            }
        },
    )?;

    let res = Response::new().add_attribute("action", "provwasm.contracts.fund.update_ccy");

    Ok(res)
}

/// Function to transfer stable coins to agent's account.
///
/// Events:-
///     - provwasm.contracts.fund.send_stable_coins
///     - to
///     - amount
fn try_send_stable_coins(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
) -> Result<Response, ContractError> {
    let agent = get_agent_by_id(deps.storage, denom)?;

    // Transfer stable coins from agent account
    let msg: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
        to_address: agent.to_string(),
        amount: info.funds.clone(),
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.fund.send_stable_coins")
        .add_attribute("to", agent.to_string())
        .add_attribute("amount", format!("{:?}", info.funds)))
}
