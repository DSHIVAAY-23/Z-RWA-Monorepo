#[cfg(not(feature = "library"))]
use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Execute,
) -> Result<Response, ContractError> {
    match msg {
        Execute::Mint { to_addr, amount } => Ok(execute_mint(deps, env, info, to_addr, amount)?),
        Execute::Transfer { recipient, amount } => transfer(deps, env, info, recipient, amount),
        Execute::Send {
            contract,
            amount,
            msg,
        } => send(deps, env, info, contract, amount, msg),
        Execute::Burn { amount } => Ok(execute_burn(deps, env, info, amount)?),
        Execute::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_increase_allowance(
            deps, env, info, spender, amount, expires,
        )?),
        Execute::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_decrease_allowance(
            deps, env, info, spender, amount, expires,
        )?),
        Execute::TransferFrom {
            owner,
            recipient,
            amount,
        } => transfer_from(deps, env, info, owner, recipient, amount),
        Execute::BurnFrom { owner, amount } => burn_from(deps, env, info, owner, amount),
        Execute::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => send_from(deps, env, info, owner, contract, amount, msg),
        Execute::UpdateMarketing {
            project,
            description,
            marketing,
        } => Ok(execute_update_marketing(
            deps,
            env,
            info,
            project,
            description,
            marketing,
        )?),
        Execute::UploadLogo(logo) => Ok(execute_upload_logo(deps, env, info, logo)?),
        Execute::ManageRoles { roles } => manage_roles(deps, info, roles),
        Execute::Freeze { update_type } => try_update_freezelist(deps, info.sender, update_type),
        Execute::PartialFreeze { params } => try_partial_freeze(deps, info.sender, params),
        Execute::UpdateMinter { new_minter } => {
            Ok(execute_update_minter(deps, env, info, new_minter)?)
        }
    }
}

fn transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // ensure not freezed
    ensure_not_freezed(deps.storage, vec![Addr::unchecked(recipient.clone())])?;

    Ok(execute_transfer(deps, env, info, recipient, amount)?)
}

fn send(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract: String,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, ContractError> {
    // ensure not freezed
    ensure_not_freezed(deps.storage, vec![info.sender.clone()])?;

    Ok(execute_send(deps, env, info, contract, amount, msg)?)
}

fn transfer_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let from = Addr::unchecked(owner.clone());
    let to = Addr::unchecked(recipient.clone());

    // ensure not blacklisted
    ensure_not_freezed(deps.storage, vec![from.clone(), to])?;

    // ensure balance not frozen
    ensure_bal_not_frozen(deps.as_ref(), from)?;

    Ok(execute_transfer_from(
        deps, env, info, owner, recipient, amount,
    )?)
}

fn burn_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // ensure token balance no frozen.
    ensure_bal_not_frozen(deps.as_ref(), Addr::unchecked(owner.clone()))?;

    Ok(execute_burn_from(deps, env, info, owner, amount)?)
}

pub fn send_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
    contract: String,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, ContractError> {
    let from = Addr::unchecked(owner.clone());
    let to = Addr::unchecked(contract.clone());

    // ensure not blacklisted
    ensure_not_freezed(deps.storage, vec![from.clone(), to])?;

    // ensure balance not frozen
    ensure_bal_not_frozen(deps.as_ref(), from)?;

    Ok(execute_send_from(
        deps, env, info, owner, contract, amount, msg,
    )?)
}

// Function to manage different roles
pub fn manage_roles(
    deps: DepsMut,
    info: MessageInfo,
    roles: Vec<Role>,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();

    for role in roles {
        match role {
            Role::Issuer { update_type } => {
                // Only Subadmin can add/remove Issuer
                is_subadmin(&deps, info.sender.clone())?;

                match update_type {
                    UpdateType::Add(addr) => {
                        let key = addr.as_bytes();
                        if ISSUER.has(deps.storage, key) {
                            return Err(ContractError::AlreadyAdded { addr });
                        }
                        ISSUER.save(deps.storage, key, &AccessControls::issuer_rights())?;
                        attrs.push(attr("action", "provwasm.contracts.marker.add_issuer"));
                    }
                    UpdateType::Remove(addr) => {
                        let key = addr.as_bytes();
                        if !ISSUER.has(deps.storage, key) {
                            return Err(ContractError::NotFound { addr });
                        }
                        ISSUER.remove(deps.storage, key);
                        attrs.push(attr("action", "provwasm.contracts.marker.remove_issuer"));
                    }
                }
            }
            Role::TransferAgent { update_type } => {
                // Only Subadmin can add/remove Transfer Agent
                is_subadmin(&deps, info.sender.clone())?;

                match update_type {
                    UpdateType::Add(addr) => {
                        let key = addr.as_bytes();
                        if TRANSFER_AGENT.has(deps.storage, key) {
                            return Err(ContractError::AlreadyAdded { addr });
                        }
                        TRANSFER_AGENT.save(
                            deps.storage,
                            key,
                            &AccessControls::transfer_agent_rights(),
                        )?;
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.marker.add_transfer_agent",
                        ));
                    }
                    UpdateType::Remove(addr) => {
                        let key = addr.as_bytes();
                        if !TRANSFER_AGENT.has(deps.storage, key) {
                            return Err(ContractError::NotFound { addr });
                        }
                        TRANSFER_AGENT.remove(deps.storage, key);
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.marker.remove_transfer_agent",
                        ));
                    }
                }
            }
            Role::TokenizationAgent { update_type } => {
                // Only Subadmin can add/remove Tokenization Agent
                is_subadmin(&deps, info.sender.clone())?;

                match update_type {
                    UpdateType::Add(addr) => {
                        let key = addr.as_bytes();
                        if TOKENIZATION_AGENT.has(deps.storage, key) {
                            return Err(ContractError::AlreadyAdded { addr });
                        }
                        TOKENIZATION_AGENT.save(
                            deps.storage,
                            key,
                            &AccessControls::tokenization_agent_rights(),
                        )?;
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.marker.add_tokenization_agent",
                        ));
                    }
                    UpdateType::Remove(addr) => {
                        let key = addr.as_bytes();
                        if !TOKENIZATION_AGENT.has(deps.storage, key) {
                            return Err(ContractError::NotFound { addr });
                        }
                        TOKENIZATION_AGENT.remove(deps.storage, key);
                        attrs.push(attr(
                            "action",
                            "provwasm.contracts.marker.remove_tokenization_agent",
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
                                        AccessControls::Admin,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Mint => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Mint,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Burn => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Burn,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Delete => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Delete,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Deposit => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Deposit,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Transfer => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Transfer,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Unspecified => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Unspecified,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Withdraw => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Withdraw,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }

                                AccessControls::Freeze => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Freeze,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::Unfreeze => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Unfreeze,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                                AccessControls::ForceTransfer => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::ForceTransfer,
                                        to.clone(),
                                        UpdateType::Add(()),
                                    )?;
                                }
                            }
                        }
                        attrs.push(attr("action", "provwasm.contracts.marker.grant_access"));
                    }
                    UpdateType::Remove(to) => {
                        for access in marker_access {
                            match access {
                                AccessControls::Admin => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Admin,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Mint => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Mint,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Burn => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Burn,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Delete => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Delete,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Deposit => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Deposit,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Transfer => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Transfer,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Unspecified => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Unspecified,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Withdraw => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Withdraw,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Freeze => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Freeze,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::Unfreeze => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::Unfreeze,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                                AccessControls::ForceTransfer => {
                                    manage_agent_access(
                                        deps.storage,
                                        AccessControls::ForceTransfer,
                                        to.clone(),
                                        UpdateType::Remove(()),
                                    )?;
                                }
                            }
                        }
                        attrs.push(attr("action", "provwasm.contracts.marker.ungrant_access"));
                    }
                }
            }
        }
    }

    Ok(Response::new().add_attributes(attrs))
}

// Update Freeze List.
fn try_update_freezelist(
    deps: DepsMut,
    sender: Addr,
    update_type: UpdateType<Vec<Addr>>,
) -> Result<Response, ContractError> {
    // ensure not freezed
    ensure_not_freezed(deps.storage, vec![sender.clone()])?;

    match update_type.clone() {
        UpdateType::Add(addrs) => {
            // Ensuring authorized sender
            if is_issuer(&deps, sender.clone()).is_err()
                && is_transfer_agent(&deps, sender.clone()).is_err()
                && has_freeze_access(&deps, sender.clone()).is_err()
                && is_subadmin(&deps, sender.clone()).is_err()
            {
                let err = format!(
                    "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Freeze rights!",
                    &sender
                );
                return Err(ContractError::Std(StdError::generic_err(err)));
            }

            FREEZE_LIST.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                Ok({
                    addresses.extend(addrs);
                    addresses.sort();
                    addresses.dedup();
                    addresses
                })
            })?;
        }
        UpdateType::Remove(addrs) => {
            // Ensuring authorized sender
            if is_issuer(&deps, sender.clone()).is_err()
                && is_transfer_agent(&deps, sender.clone()).is_err()
                && has_unfreeze_access(&deps, sender.clone()).is_err()
                && is_subadmin(&deps, sender.clone()).is_err()
            {
                let err = format!(
                    "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Unfreeze rights!",
                    &sender
                );
                return Err(ContractError::Std(StdError::generic_err(err)));
            }

            FREEZE_LIST.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                Ok({
                    addresses.retain(|addr| !addrs.contains(addr));
                    addresses
                })
            })?;
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
    params: Vec<PartialFreezeParams>,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();
    let mut rem_bal = Uint128::zero();

    for param in params {
        let key = param.address.as_bytes();

        match param.update_type {
            UpdateType::Add(bal) => {
                // Ensuring authorized sender
                if is_issuer(&deps, sender.clone()).is_err()
                    && is_transfer_agent(&deps, sender.clone()).is_err()
                    && has_freeze_access(&deps, sender.clone()).is_err()
                    && is_subadmin(&deps, sender.clone()).is_err()
                {
                    let err = format!(
                        "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Freeze rights!",
                        &sender
                    );
                    return Err(ContractError::Std(StdError::generic_err(err)));
                }

                match PARTIAL_FREEZE.update(
                    deps.storage,
                    key,
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
                    Err(_) => PARTIAL_FREEZE.save(deps.storage, key, &bal)?,
                };

                match FROZEN_TOKENS.update(deps.storage, |mut bals| -> Result<_, ContractError> {
                    Ok({
                        bals += bal;
                        bals
                    })
                }) {
                    Ok(_) => (),
                    Err(_) => FROZEN_TOKENS.save(deps.storage, &bal)?,
                };
            }
            UpdateType::Remove(bal) => {
                // Ensuring authorized sender
                if is_issuer(&deps, sender.clone()).is_err()
                    && is_transfer_agent(&deps, sender.clone()).is_err()
                    && has_unfreeze_access(&deps, sender.clone()).is_err()
                    && is_subadmin(&deps, sender.clone()).is_err()
                {
                    let err = format!(
                        "Address `{}`: Don't have Issuer, Transfer, Sub Admin or Freeze rights!",
                        &sender
                    );
                    return Err(ContractError::Std(StdError::generic_err(err)));
                }

                FROZEN_TOKENS.update(deps.storage, |mut bals| -> Result<_, ContractError> {
                    Ok({
                        bals -= bal;
                        bals
                    })
                })?;

                PARTIAL_FREEZE.update(
                    deps.storage,
                    key,
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
                    PARTIAL_FREEZE.remove(deps.storage, key);
                }
            }
        }
        attrs.push(attr("address", format!("{:?}", &param.address)));
        attrs.push(attr("update_type", format!("{:?}", &param.update_type)));
    }

    let res = Response::new()
        .add_attribute("action", "update_frozen_balance")
        .add_attributes(attrs);

    Ok(res)
}
