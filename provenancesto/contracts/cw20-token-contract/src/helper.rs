#![allow(unused)]
use super::*;

pub fn check_bal_avalaility(
    amount: Uint128,
    capital: Uint128,
    err: ContractError,
) -> Result<(), ContractError> {
    if amount > capital {
        return Err(err);
    }

    Ok(())
}

pub fn ensure_bal_not_frozen(deps: Deps, address: Addr) -> Result<(), ContractError> {
    let key = address.to_string();

    let bal = query_balance(deps, address.to_string())?;
    let frozen_bal = PARTIAL_FREEZE
        .load(deps.storage, key.as_bytes())
        .unwrap_or_default();

    check_bal_avalaility(
        frozen_bal,
        bal.balance,
        ContractError::BalanceFrozen { address },
    )?;

    Ok(())
}

pub fn ensure_not_freezed(
    storage: &mut dyn Storage,
    address: Vec<Addr>,
) -> Result<(), ContractError> {
    if let Ok(blacklist) = FREEZE_LIST.load(storage) {
        for addr in address {
            if blacklist.contains(&addr) {
                let err = format!("Account: `{}` Blacklisted!", addr);
                return Err(ContractError::Std(StdError::generic_err(err)));
            }
        }
    }

    Ok(())
}

pub fn has_admin_access(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = AccessControls::Admin.to_string();

    AGENTS.load(deps.storage, key.as_bytes()).map_or(
        Err(ContractError::NoAdminAccess {
            address: sender.clone(),
        }),
        |access_info| {
            if !access_info.contains(&sender) {
                Err(ContractError::NoAdminAccess { address: sender })
            } else {
                Ok(())
            }
        },
    )
}

pub fn has_mint_access(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = AccessControls::Mint.to_string();

    AGENTS.load(deps.storage, key.as_bytes()).map_or(
        Err(ContractError::NoMintAccess {
            address: sender.clone(),
        }),
        |access_info| {
            if !access_info.contains(&sender) {
                Err(ContractError::NoMintAccess { address: sender })
            } else {
                Ok(())
            }
        },
    )
}

pub fn has_burn_access(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = AccessControls::Burn.to_string();

    AGENTS.load(deps.storage, key.as_bytes()).map_or(
        Err(ContractError::NoBurnAccess {
            address: sender.clone(),
        }),
        |access_info| {
            if !access_info.contains(&sender) {
                Err(ContractError::NoBurnAccess { address: sender })
            } else {
                Ok(())
            }
        },
    )
}

pub fn has_delete_access(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = AccessControls::Delete.to_string();

    AGENTS.load(deps.storage, key.as_bytes()).map_or(
        Err(ContractError::NoDeleteAccess {
            address: sender.clone(),
        }),
        |access_info| {
            if !access_info.contains(&sender) {
                Err(ContractError::NoDeleteAccess { address: sender })
            } else {
                Ok(())
            }
        },
    )
}

pub fn has_deposit_access(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = AccessControls::Deposit.to_string();

    AGENTS.load(deps.storage, key.as_bytes()).map_or(
        Err(ContractError::NoDepositAccess {
            address: sender.clone(),
        }),
        |access_info| {
            if !access_info.contains(&sender) {
                Err(ContractError::NoDepositAccess { address: sender })
            } else {
                Ok(())
            }
        },
    )
}

pub fn has_transfer_access(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = AccessControls::Transfer.to_string();

    AGENTS.load(deps.storage, key.as_bytes()).map_or(
        Err(ContractError::NoTransferAccess {
            address: sender.clone(),
        }),
        |access_info| {
            if !access_info.contains(&sender) {
                Err(ContractError::NoTransferAccess { address: sender })
            } else {
                Ok(())
            }
        },
    )
}

pub fn has_unspecified_access(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = AccessControls::Unspecified.to_string();

    AGENTS.load(deps.storage, key.as_bytes()).map_or(
        Err(ContractError::NoUnspecifiedAccess {
            address: sender.clone(),
        }),
        |access_info| {
            if !access_info.contains(&sender) {
                Err(ContractError::NoUnspecifiedAccess { address: sender })
            } else {
                Ok(())
            }
        },
    )
}

pub fn has_withdraw_access(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = AccessControls::Withdraw.to_string();

    AGENTS.load(deps.storage, key.as_bytes()).map_or(
        Err(ContractError::NoWithdrawAccess {
            address: sender.clone(),
        }),
        |access_info| {
            if !access_info.contains(&sender) {
                Err(ContractError::NoWithdrawAccess { address: sender })
            } else {
                Ok(())
            }
        },
    )
}

pub fn has_freeze_access(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = AccessControls::Freeze.to_string();

    AGENTS.load(deps.storage, key.as_bytes()).map_or(
        Err(ContractError::NoFreezeAccess {
            address: sender.clone(),
        }),
        |access_info| {
            if !access_info.contains(&sender) {
                Err(ContractError::NoFreezeAccess { address: sender })
            } else {
                Ok(())
            }
        },
    )
}

pub fn has_unfreeze_access(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = AccessControls::Unfreeze.to_string();

    AGENTS.load(deps.storage, key.as_bytes()).map_or(
        Err(ContractError::NoUnfreezeAccess {
            address: sender.clone(),
        }),
        |access_info| {
            if !access_info.contains(&sender) {
                Err(ContractError::NoUnfreezeAccess { address: sender })
            } else {
                Ok(())
            }
        },
    )
}

pub fn has_force_transfer_access(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = AccessControls::ForceTransfer.to_string();

    AGENTS.load(deps.storage, key.as_bytes()).map_or(
        Err(ContractError::NoForceTransferAccess {
            address: sender.clone(),
        }),
        |access_info| {
            if !access_info.contains(&sender) {
                Err(ContractError::NoForceTransferAccess { address: sender })
            } else {
                Ok(())
            }
        },
    )
}

pub fn is_admin(deps: &DepsMut, addr: Addr) -> Result<(), ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    if admin != addr {
        return Err(ContractError::NotAdmin { address: addr });
    }

    Ok(())
}

pub fn is_subadmin(deps: &DepsMut, address: Addr) -> Result<(), ContractError> {
    SUB_ADMIN.load(deps.storage).map_or(
        Err(ContractError::NotSubAdmin {
            address: address.clone(),
        }),
        |sub_admins| {
            if !sub_admins.contains(&address) {
                Err(ContractError::NotSubAdmin { address })
            } else {
                Ok(())
            }
        },
    )
}

pub fn is_issuer(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = sender.as_bytes();
    let issuer_accesses = ISSUER.load(deps.storage, key)?;

    if !(issuer_accesses.contains(&AccessControls::Mint)
        && issuer_accesses.contains(&AccessControls::Burn)
        && issuer_accesses.contains(&AccessControls::Freeze)
        && issuer_accesses.contains(&AccessControls::Unfreeze)
        && issuer_accesses.contains(&AccessControls::ForceTransfer))
    {
        return Err(ContractError::NotAnIssuer { address: sender });
    }

    Ok(())
}

pub fn is_transfer_agent(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = sender.as_bytes();
    let transfer_agent_access = TRANSFER_AGENT.load(deps.storage, key)?;

    if !(transfer_agent_access.contains(&AccessControls::Freeze)
        && transfer_agent_access.contains(&AccessControls::Unfreeze)
        && transfer_agent_access.contains(&AccessControls::ForceTransfer))
    {
        return Err(ContractError::NotATransferAgent { address: sender });
    }

    Ok(())
}

pub fn is_tokenization_agent(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let key = sender.as_bytes();
    let tokenization_access = TOKENIZATION_AGENT.load(deps.storage, key)?;

    if !(tokenization_access.contains(&AccessControls::Mint)
        && tokenization_access.contains(&AccessControls::Burn))
    {
        return Err(ContractError::NotATokenizationAgent { address: sender });
    }

    Ok(())
}

pub fn manage_agent_access(
    storage: &mut dyn Storage,
    access_control: AccessControls,
    addrs: Vec<Addr>,
    update_type: UpdateType<()>,
) -> Result<(), ContractError> {
    let key = access_control.to_string();

    match update_type {
        UpdateType::Add(_) => {
            AGENTS.update(
                storage,
                key.as_bytes(),
                |addresses_opt| -> Result<_, ContractError> {
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
        UpdateType::Remove(_) => {
            AGENTS.update(
                storage,
                key.as_bytes(),
                |addresses_opt| -> Result<_, ContractError> {
                    match addresses_opt {
                        Some(mut addresses) => Ok({
                            addresses.retain(|addr| !addrs.contains(addr));
                            addresses
                        }),
                        None => Ok(Vec::default()),
                    }
                },
            )?;
        }
    }

    Ok(())
}
