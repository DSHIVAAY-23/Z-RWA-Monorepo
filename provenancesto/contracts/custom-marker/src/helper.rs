use super::*;
use cosmwasm_std::CosmosMsg;
use provwasm_std::ProvenanceQuery;
use provwasm_std::{ProvenanceMsg, ProvenanceQuerier};

/// Function to check balance availability
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

/// Function to get consolidated balance
fn get_consolidated_balance(
    deps: Deps<ProvenanceQuery>,
    address: Addr,
    denom: String,
) -> StdResult<Uint128> {
    let bal = deps.querier.query_balance(address, denom)?;

    Ok(bal.amount)
}

/// Function to ensure holding period is passed
pub fn ensure_holding_period_passed(
    storage: &mut dyn Storage,
    denom: &[u8],
    current_timestamp: u64,
) -> Result<(), ContractError> {
    if let Ok(holding_period) = HOLDING_PERIOD.load(storage, denom) {
        ensure!(
            holding_period.u64() < current_timestamp,
            ContractError::TokenHeld {
                timestamp: holding_period.u64()
            }
        );
    }

    Ok(())
}

/// Function to ensure token limit is not exceeded
pub fn ensure_token_limit(
    deps: Deps<ProvenanceQuery>,
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

/// Function to check balance is not frozen
pub fn ensure_bal_not_frozen(
    deps: Deps<ProvenanceQuery>,
    address: Addr,
    denom: String,
) -> Result<(), ContractError> {
    let key = Key::new(denom.to_string(), address.clone()).as_bytes()?;
    let bal = get_consolidated_balance(deps, address.clone(), denom.clone())?;
    let frozen_bal = PARTIAL_FREEZE.load(deps.storage, &key).unwrap_or_default();

    check_bal_avalaility(
        frozen_bal,
        bal,
        ContractError::BalanceFrozen { denom, address },
    )?;

    Ok(())
}

/// Function to check balance is maintained
pub fn ensure_bal_maintained(
    deps: Deps<ProvenanceQuery>,
    to: Addr,
    from: Addr,
    denom: String,
    amount: Uint128,
) -> Result<(), ContractError> {
    ensure_bal_not_frozen(deps, from, denom.clone())?;
    ensure_token_limit(deps, to, denom, amount)?;

    Ok(())
}

/// Function to the user is whitelisted
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

/// Function to ensure account is not freezed
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

/// Helper function to add country codes
pub fn add_country_codes(country_codes: &mut Vec<u8>, code: u8) -> Result<(), ContractError> {
    if country_codes.contains(&code) {
        return Err(ContractError::CountryCodeAlreadyExists { code });
    } else {
        country_codes.push(code)
    }

    Ok(())
}

/// Helper function to remove country codes
pub fn remove_country_codes(country_codes: &mut Vec<u8>, code: u8) -> Result<(), ContractError> {
    if !country_codes.contains(&code) {
        return Err(ContractError::CountryCodeNotExists { code });
    } else {
        country_codes.retain(|cd| cd.ne(&code));
    }

    Ok(())
}

/// Function to get country code from an address
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

/// Function to check the user has admin rights or not
pub fn has_admin_access(
    deps: &DepsMut<ProvenanceQuery>,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
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

/// Function to check the user has mint rights or not
pub fn has_mint_access(
    deps: &DepsMut<ProvenanceQuery>,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
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

/// Function to check the user has burn rights or not
pub fn has_burn_access(
    deps: &DepsMut<ProvenanceQuery>,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
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

/// Function to check the user has delete rights or not
pub fn has_delete_access(
    deps: &DepsMut<ProvenanceQuery>,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
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

/// Function to check the user has deposit rights or not
pub fn has_deposit_access(
    deps: &DepsMut<ProvenanceQuery>,
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

/// Function to check the user has transfer rights or not
pub fn has_transfer_access(
    deps: &DepsMut<ProvenanceQuery>,
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

/// Function to check the user has unspecified rights or not
pub fn has_unspecified_access(
    deps: &DepsMut<ProvenanceQuery>,
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

/// Function to check the user has withdraw rights or not
pub fn has_withdraw_access(
    deps: &DepsMut<ProvenanceQuery>,
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

/// Function to check the user has freeze rights or not
pub fn has_freeze_access(
    deps: &DepsMut<ProvenanceQuery>,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
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

/// Function to check the user has unfreeze rights or not
pub fn has_unfreeze_access(
    deps: &DepsMut<ProvenanceQuery>,
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

/// Function to check the user has force transfer rights or not
pub fn has_force_transfer_access(
    deps: &DepsMut<ProvenanceQuery>,
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

/// Function to check wheather the user has admin access or not
pub fn is_admin(deps: &DepsMut<ProvenanceQuery>, addr: Addr) -> Result<(), ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    if admin != addr {
        return Err(ContractError::NotAdmin { address: addr });
    }

    Ok(())
}

/// Function to check wheather the user has sub_admin access or not
pub fn is_subadmin(deps: &DepsMut<ProvenanceQuery>, address: Addr) -> Result<(), ContractError> {
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

/// Function to check wheather the user has issuer access or not
pub fn is_issuer(
    deps: &DepsMut<ProvenanceQuery>,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
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

/// Function to check wheather the user has transfer agent access or not
pub fn is_transfer_agent(
    deps: &DepsMut<ProvenanceQuery>,
    denom: String,
    sender: Addr,
) -> Result<(), ContractError> {
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

/// Function to check wheather the user has tokenization agent access or not
pub fn is_tokenization_agent(
    deps: &DepsMut<ProvenanceQuery>,
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

/// Function to manage minted tokens
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

/// Function to manage agent access
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

/// Helper function to create new marker with Finalized state
pub fn create_marker<S: Into<String> + Clone>(
    denom: S,
    _contract_address: Addr,
) -> StdResult<Vec<CosmosMsg<ProvenanceMsg>>> {
    let msgs: Vec<CosmosMsg<ProvenanceMsg>> = vec![
        create_forced_transfer_marker(0u128, denom.clone())?,
        finalize_marker(denom.clone())?,
        activate_marker(denom)?,
    ];
    Ok(msgs)
}

pub fn cm_create_marker<S: Into<String> + Clone>(
    denom: S,
    contract_address: Addr,
) -> StdResult<Vec<CosmosMsg<ProvenanceMsg>>> {
    create_marker(denom, contract_address)
}

// Function to provide marker accesses to an address
pub fn cm_grant_marker_access<S: Into<String>, H: Into<Addr>>(
    denom: S,
    address: H,
    permissions: Vec<Access>,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    grant_marker_access(denom, address, permissions)
}

/// Helper function for coin withdrawal
pub fn cm_withdraw_coins<S: Into<String> + Clone, H: Into<Addr>>(
    denom: S,
    amount: u128,
    recipient: H,
    _contract_address: Addr,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    withdraw_coins(denom.clone(), amount, denom, recipient)
}

/// Helper function for increasing marker supply
pub fn cm_mint_marker_supply<S: Into<String>>(
    amount: u128,
    denom: S,
    _contract_address: Addr,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    mint_marker_supply(amount, denom)
}

/// Helper function for decreasing marker supply
pub fn cm_burn_marker_supply<S: Into<String>>(
    amount: u128,
    denom: S,
    _contract_address: Addr,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    burn_marker_supply(amount, denom)
}

/// Helper function for cancelling marker
pub fn cm_cancel_marker<S: Into<String>>(
    denom: S,
    _contract_address: Addr,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    cancel_marker(denom)
}

/// Helper function for destroying marker
pub fn cm_destroy_marker<S: Into<String>>(
    denom: S,
    _contract_address: Addr,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    destroy_marker(denom)
}

/// Helper function for transferring marker coins
pub fn cm_transfer_marker_coins<S: Into<String>, H: Into<Addr>>(
    amount: u128,
    denom: S,
    to: H,
    from: H,
    _contract_address: H,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    transfer_marker_coins(amount, denom, to, from)
}

/// Helper function to get marker by an address
pub fn get_marker_by_address<H: Into<Addr>>(
    address: H,
    querier: &ProvenanceQuerier,
) -> StdResult<Marker> {
    let m = querier.get_marker_by_address(address)?;
    Ok(Marker {
        marker_account: m.clone(),
        coins: m.coins.clone(),
    })
}

/// Helper function to get marker by denom
pub fn get_marker_by_denom<H: Into<String>>(
    denom: H,
    querier: &ProvenanceQuerier,
) -> StdResult<Marker> {
    let m = querier.get_marker_by_denom(denom)?;
    Ok(Marker {
        marker_account: m.clone(),
        coins: m.coins.clone(),
    })
}

/// Helper function to get marker
pub fn get_marker(_id: String, _querier: &ProvenanceQuerier) -> StdResult<Marker> {
    unimplemented!()
}

/// Helper function to get marker address
pub fn get_marker_address(id: String, querier: &ProvenanceQuerier) -> StdResult<Addr> {
    let m = querier.get_marker_by_denom(id)?;
    Ok(m.address)
}

pub fn access() -> Vec<Access> {
    vec![
        Access::Admin,
        Access::Burn,
        Access::Deposit,
        Access::Delete,
        Access::Mint,
        Access::Transfer,
        Access::Withdraw,
    ]
}

/// Helper function to provide all the accesses
pub fn all_access(_address: &Addr) -> Vec<Access> {
    access()
}

/// Helper function to provide all access to addresses
pub fn all_access_to_addresses(addresses: &[Addr], is_all_access: bool) -> Vec<Access> {
    let mut access_grant = Vec::new();

    let permissions = if is_all_access {
        access()
    } else {
        Vec::default()
    };

    for _ in addresses.iter() {
        access_grant.extend(permissions.clone());
    }

    access_grant
}

/// A helper that ensures string params are non-empty.
pub fn validate_string<S: Into<String>>(input: S, param_name: &str) -> StdResult<String> {
    let s: String = input.into();
    if s.trim().is_empty() {
        let err = format!("{} must not be empty", param_name);
        Err(StdError::generic_err(err))
    } else {
        Ok(s)
    }
}

/// A helper that ensures address params are non-empty.
pub fn validate_address<H: Into<Addr>>(input: H) -> StdResult<Addr> {
    let h: Addr = input.into();
    if h.to_string().trim().is_empty() {
        Err(StdError::generic_err("address must not be empty"))
    } else {
        Ok(h)
    }
}

/// Helper function for mint to
pub fn mint_to(
    denom: String,
    data: MintBurnData,
    contract_address: Addr,
) -> Result<Vec<CosmosMsg<ProvenanceMsg>>, ContractError> {
    let msgs = vec![
        cm_mint_marker_supply(data.amount.u128(), denom.clone(), contract_address.clone())?,
        cm_withdraw_coins(&denom, data.amount.u128(), data.address, contract_address)?,
    ];

    Ok(msgs)
}

/// Helper function for burn from
pub fn burn_from(
    denom: String,
    data: MintBurnData,
    contract_address: Addr,
    querier: &ProvenanceQuerier,
) -> Result<Vec<CosmosMsg<ProvenanceMsg>>, ContractError> {
    let marker_addr = get_marker_address(validate_string(denom.clone(), "denom")?, querier)?;

    let msgs = vec![
        cm_transfer_marker_coins(
            data.amount.u128(),
            &denom,
            marker_addr,
            data.address.clone(),
            contract_address.clone(),
        )?,
        cm_burn_marker_supply(data.amount.u128(), &denom, contract_address)?,
    ];

    Ok(msgs)
}
