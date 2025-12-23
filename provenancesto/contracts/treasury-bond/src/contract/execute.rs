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
        ExecuteMsg::ManageRoles { role } => try_manage_roles(deps, info, role),
        ExecuteMsg::ShareStableCoin {
            denom,
            coin_type,
            share_params,
        } => try_share_stable_coin(deps, info, denom, coin_type, share_params),
        ExecuteMsg::UpdateCreditRating { denom, rating } => {
            try_update_credit_rating(deps, info, denom, rating)
        }
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
///             provwasm.contracts.treasury.add_admin
///     case 2. During removal, i.e., update_type = UpdateType::Remove(Vec<Addr>)
///         then
///             provwasm.contracts.treasury.remove_admin
///
/// For Managing Agent
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
///             provwasm.contracts.treasury.add_agent
///     case 2. During removal, i.e., update_type = UpdateType::Remove(Addr)
///         then
///             provwasm.contracts.treasury.remove_agent
fn try_manage_roles(
    deps: DepsMut,
    info: MessageInfo,
    role: Role,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();

    // Ensuring authorised sender
    is_admin(deps.storage, info.sender)?;

    match role {
        Role::Admin { update_type } => {
            match update_type {
                UpdateType::Add(addrs) => {
                    let updated =
                        ADMIN.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                            addresses.extend(addrs.clone());

                            // Removing duplicate entries, if any
                            addresses.sort();
                            addresses.dedup();

                            Ok(addresses)
                        });
                    if updated.is_err() {
                        ADMIN.save(deps.storage, &addrs)?;
                    };
                    attrs.push(attr("action", "provwasm.contracts.treasury.add_admin"));
                }
                UpdateType::Remove(addrs) => {
                    ADMIN.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                        addresses.retain(|addr| !addrs.contains(addr));
                        Ok(addresses)
                    })?;
                    attrs.push(attr("action", "provwasm.contracts.treasury.remove_admin"));
                }
                UpdateType::Update(_) => (),
            }
        }
        Role::Agent { denom, address } => {
            AGENT.save(deps.storage, denom.as_bytes(), &address)?;

            GLOBAL_CONFIG.update(
                deps.storage,
                denom.as_bytes(),
                |config_opt| -> Result<_, ContractError> {
                    let mut config = config_opt.ok_or(ContractError::ConfigNotFound {
                        denom: denom.clone(),
                    })?;
                    config.treasury_manager = address;

                    Ok(config)
                },
            )?;

            attrs.push(attr("action", "provwasm.contracts.treasury.update_agent"));
        }
    }

    Ok(Response::new().add_attributes(attrs))
}

/// Function to create new treasury_contract contract for a particular token.
/// In this function call:-
///     - the creator is assigned as agent
///     - the contract address will be assigned mint, burn, transfer and force_transfer accesses, in token contract
///
/// Notes
/// Before calling the `create` function, the treasury_contract contract address must be assigned as sub_admin in token contract, as
/// sub_admin in token contract have all the mint, burn, transfer and force_transfer accesses.
///
/// Events:-
///     - provwasm.contracts.treasury.create
///     - denom
///     - treasury_contract_name
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
    let global_config = GlobalConfig::new(
        params.clone(),
        env.block.time.seconds(),
        info.sender.clone(),
    );

    if !GLOBAL_CONFIG.has(deps.storage, params.denom.as_bytes()) {
        GLOBAL_CONFIG.save(deps.storage, params.denom.as_bytes(), &global_config)?;
    } else {
        return Err(ContractError::AlreadyExists {
            addr: env.contract.address,
        });
    }

    // Adding sender as agent
    AGENT.save(deps.storage, params.denom.as_bytes(), &info.sender)?;

    Ok(Response::new()
        .add_attribute("action", "provwasm.contracts.treasury.create")
        .add_attribute("denom", params.denom))
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
///             provwasm.contracts.treasury.share_dividend.token
///     case 2. When asset_type = stable_coin
///         then
///             provwasm.contracts.treasury.share_dividend.stable_coin
///     case 3. When asset_type = fiat
///         then
///             provwasm.contracts.treasury.share_dividend.fiat
fn try_share_stable_coin(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    coin_type: CoinType,
    share_params: Vec<ShareParams>,
) -> Result<Response, ContractError> {
    let mut msgs = Vec::new();

    // Ensuring authorised sender
    is_agent(deps.storage, denom.clone(), info.sender)?;

    for param in share_params {
        // Transferring stable coins from agent account
        let msg: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
            to_address: param.to.to_string(),
            amount: vec![coin(param.payment.u128(), coin_type.get_denom())],
        });
        msgs.push(msg);

        add_or_update_payments(
            deps.storage,
            denom.to_string(),
            param.to.clone(),
            param.payment,
        )?;
    }

    Ok(Response::new()
        .add_attribute("action", "provwasm.contracts.treasury.share_stable_coins")
        .add_messages(msgs))
}

/// Function to update credit rating
///
/// Notes
///     This function is not used anywhere at the moment, that will be used in future verisons.
///
/// Fails when:-
///     caller is not admin
///
/// Event:-
///     provwasm.contracts.treasury.update_ccy
fn try_update_credit_rating(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    rating: String,
) -> Result<Response, ContractError> {
    // Ensuring authorised sender
    is_agent(deps.storage, denom.to_string(), info.sender)?;

    GLOBAL_CONFIG.update(
        deps.storage,
        denom.as_bytes(),
        |global_opt: Option<GlobalConfig>| -> Result<_, ContractError> {
            match global_opt {
                Some(mut global) => Ok({
                    global.credit_rating = rating;
                    global
                }),
                None => Err(ContractError::ConfigNotFound {
                    denom: denom.clone(),
                }),
            }
        },
    )?;

    Ok(Response::new().add_attribute("action", "provwasm.contracts.treasury.update_credit_rating"))
}

/// Function to transfer stable coins to agent's account.
///
/// Events:-
///     - provwasm.contracts.treasury.send_stable_coins
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
        .add_attribute("action", "provwasm.contracts.treasury.send_stable_coins")
        .add_attribute("to", agent.to_string())
        .add_attribute("amount", format!("{:?}", info.funds)))
}
