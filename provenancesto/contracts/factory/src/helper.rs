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

fn get_consolidated_balance(deps: Deps, address: Addr, denom: String) -> StdResult<Uint128> {
    let bal = deps.querier.query_balance(address, denom)?;

    Ok(bal.amount)
}

pub fn ensure_token_limit(
    deps: Deps,
    address: Addr,
    denom: String,
    amount: Uint128,
) -> Result<(), ContractError> {
    let denom_config = DENOM_CONFIG.load(deps.storage, denom.as_bytes())?;
    let bal = get_consolidated_balance(deps, address.clone(), denom)?;

    check_bal_avalaility(
        bal + amount,
        denom_config.token_limit,
        ContractError::TokenLimitExceeded { address },
    )?;

    Ok(())
}

pub fn ensure_bal_not_frozen(
    deps: Deps,
    address: Addr,
    denom: String,
) -> Result<(), ContractError> {
    let key = Key::new(denom.to_string(), address.clone()).as_bytes()?;
    let bal = get_consolidated_balance(deps, address.clone(), denom.clone())?;
    let frozen_bal = PARTIAL_FREEZE
        .load(deps.storage, &key)
        .unwrap_or(Uint128::default());

    check_bal_avalaility(
        frozen_bal,
        bal,
        ContractError::BalanceFrozen { denom, address },
    )?;

    Ok(())
}

pub fn ensure_bal_maintained(
    deps: Deps,
    to: Addr,
    from: Addr,
    denom: String,
    amount: Uint128,
) -> Result<(), ContractError> {
    ensure_bal_not_frozen(deps, from, denom.clone())?;
    ensure_token_limit(deps, to, denom, amount)?;

    Ok(())
}

pub fn ensure_authorized_country(
    storage: &mut dyn Storage,
    denom: String,
    address: Addr,
) -> Result<(), ContractError> {
    let denom_config = DENOM_CONFIG.load(storage, denom.as_bytes())?;
    let country_code = get_country_code_from_whitelist(storage, denom.clone(), address.clone())?;

    if !denom_config
        .country_codes
        .iter()
        .any(|code| country_code.eq(code))
    {
        return Err(ContractError::CountryCodeAuthorizationFailed { denom, address });
    }

    Ok(())
}

pub fn ensure_not_freezed(
    storage: &mut dyn Storage,
    address: Vec<Addr>,
    denom: &[u8],
) -> Result<(), ContractError> {
    if let Ok(blacklist) = FREEZE_LIST.load(storage, denom) {
        for addr in address {
            if blacklist.contains(&addr) {
                let err = format!("Account: `{}` Blacklisted!", addr);
                return Err(ContractError::Unauthorized { err });
            }
        }
    }

    Ok(())
}

pub fn add_country_codes(country_codes: &mut Vec<u8>, code: u8) -> Result<(), ContractError> {
    if country_codes.contains(&code) {
        return Err(ContractError::CountryCodeAlreadyExists { code });
    } else {
        country_codes.push(code)
    }

    Ok(())
}

pub fn remove_country_codes(country_codes: &mut Vec<u8>, code: u8) -> Result<(), ContractError> {
    if !country_codes.contains(&code) {
        return Err(ContractError::CountryCodeNotExists { code });
    } else {
        country_codes.retain(|cd| cd.ne(&code));
    }

    Ok(())
}

pub fn get_country_code_from_whitelist(
    storage: &mut dyn Storage,
    denom: String,
    address: Addr,
) -> Result<u8, ContractError> {
    let key = Key::new(denom.to_string(), address.clone()).as_bytes()?;

    match WHITELIST.load(storage, &key) {
        Ok(code) => Ok(code),
        Err(_) => Err(ContractError::CountryCodeAuthorizationFailed { denom, address }),
    }
}

pub fn has_admin_access(deps: &DepsMut, denom: String, sender: Addr) -> Result<(), ContractError> {
    let key = Key::new(denom, AccessControls::Admin).as_bytes()?;

    AGENTS.load(deps.storage, &key).map_or(
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

pub fn has_mint_access(deps: &DepsMut, denom: String, sender: Addr) -> Result<(), ContractError> {
    let key = Key::new(denom, AccessControls::Mint).as_bytes()?;

    AGENTS.load(deps.storage, &key).map_or(
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

pub fn has_burn_access(deps: &DepsMut, denom: String, sender: Addr) -> Result<(), ContractError> {
    let key = Key::new(denom, AccessControls::Burn).as_bytes()?;

    AGENTS.load(deps.storage, &key).map_or(
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

pub fn has_delete_access(deps: &DepsMut, denom: String, sender: Addr) -> Result<(), ContractError> {
    let key = Key::new(denom, AccessControls::Delete).as_bytes()?;

    AGENTS.load(deps.storage, &key).map_or(
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

pub fn has_deposit_access(
    deps: &DepsMut,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
    let key = Key::new(denom, AccessControls::Deposit).as_bytes()?;

    AGENTS.load(deps.storage, &key).map_or(
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

pub fn has_transfer_access(
    deps: &DepsMut,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
    let key = Key::new(denom, AccessControls::Transfer).as_bytes()?;

    AGENTS.load(deps.storage, &key).map_or(
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

pub fn has_unspecified_access(
    deps: &DepsMut,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
    let key = Key::new(denom, AccessControls::Unspecified).as_bytes()?;

    AGENTS.load(deps.storage, &key).map_or(
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

pub fn has_withdraw_access(
    deps: &DepsMut,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
    let key = Key::new(denom, AccessControls::Withdraw).as_bytes()?;

    AGENTS.load(deps.storage, &key).map_or(
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

pub fn has_freeze_access(deps: &DepsMut, denom: String, sender: Addr) -> Result<(), ContractError> {
    let key = Key::new(denom, AccessControls::Freeze).as_bytes()?;

    AGENTS.load(deps.storage, &key).map_or(
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

pub fn has_unfreeze_access(
    deps: &DepsMut,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
    let key = Key::new(denom, AccessControls::Unfreeze).as_bytes()?;

    AGENTS.load(deps.storage, &key).map_or(
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

pub fn has_force_transfer_access(
    deps: &DepsMut,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
    let key = Key::new(denom, AccessControls::ForceTransfer).as_bytes()?;

    AGENTS.load(deps.storage, &key).map_or(
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

pub fn is_issuer(deps: &DepsMut, denom: String, sender: Addr) -> Result<(), ContractError> {
    let key = Key::new(denom, sender.clone()).as_bytes()?;
    let issuer_accesses = ISSUER.load(deps.storage, &key)?;

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

pub fn is_transfer_agent(deps: &DepsMut, denom: String, sender: Addr) -> Result<(), ContractError> {
    let key = Key::new(denom, sender.clone()).as_bytes()?;
    let transfer_agent_access = TRANSFER_AGENT.load(deps.storage, &key)?;

    if !(transfer_agent_access.contains(&AccessControls::Freeze)
        && transfer_agent_access.contains(&AccessControls::Unfreeze)
        && transfer_agent_access.contains(&AccessControls::ForceTransfer))
    {
        return Err(ContractError::NotATransferAgent { address: sender });
    }

    Ok(())
}

pub fn is_tokenization_agent(
    deps: &DepsMut,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
    let key = Key::new(denom, sender.clone()).as_bytes()?;
    let tokenization_access = TOKENIZATION_AGENT.load(deps.storage, &key)?;

    if !(tokenization_access.contains(&AccessControls::Mint)
        && tokenization_access.contains(&AccessControls::Burn))
    {
        return Err(ContractError::NotATokenizationAgent { address: sender });
    }

    Ok(())
}

pub fn update_minted_tokens(
    storage: &mut dyn Storage,
    denom: String,
    update_type: UpdateType<Uint128>,
) -> Result<(), ContractError> {
    match update_type {
        UpdateType::Add(amount) => {
            match MINTED_TOKENS.update(
                storage,
                denom.as_bytes(),
                |bals_opt| -> Result<_, ContractError> {
                    match bals_opt {
                        Some(mut bals) => Ok({
                            bals += amount;
                            bals
                        }),
                        None => Ok(amount),
                    }
                },
            ) {
                Ok(_) => (),
                Err(_) => MINTED_TOKENS.save(storage, denom.as_bytes(), &amount)?,
            };
        }
        UpdateType::Remove(amount) => {
            MINTED_TOKENS.update(
                storage,
                denom.as_bytes(),
                |bals_opt| -> Result<_, ContractError> {
                    match bals_opt {
                        Some(mut bals) => Ok({
                            bals -= amount;
                            bals
                        }),
                        None => Ok(amount),
                    }
                },
            )?;
        }
    }

    Ok(())
}

pub fn manage_agent_access(
    storage: &mut dyn Storage,
    denom: String,
    access_control: AccessControls,
    addrs: Vec<Addr>,
    update_type: UpdateType<()>,
) -> Result<(), ContractError> {
    let key = Key::new(denom, access_control).as_bytes()?;

    match update_type {
        UpdateType::Add(_) => {
            AGENTS.update(storage, &key, |addresses_opt| -> Result<_, ContractError> {
                match addresses_opt {
                    Some(mut addresses) => Ok({
                        addresses.extend(addrs);
                        addresses.sort();
                        addresses.dedup();
                        addresses
                    }),
                    None => Ok(addrs),
                }
            })?;
        }
        UpdateType::Remove(_) => {
            AGENTS.update(storage, &key, |addresses_opt| -> Result<_, ContractError> {
                match addresses_opt {
                    Some(mut addresses) => Ok({
                        addresses.retain(|addr| !addrs.contains(addr));
                        addresses
                    }),
                    None => Ok(Vec::default()),
                }
            })?;
        }
    }

    Ok(())
}

pub fn get_contract_address(deps: Deps, denom: String) -> Result<Addr, StdError> {
    TOKEN_CONTRACT_ADDRESS.load(deps.storage, denom.as_bytes())
}
