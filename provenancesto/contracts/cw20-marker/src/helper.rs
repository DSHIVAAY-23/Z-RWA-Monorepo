use super::*;

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
fn get_consolidated_balance(deps: Deps, address: Addr, denom: String) -> StdResult<Uint128> {
    let bal = deps.querier.query_balance(address, denom)?;

    Ok(bal.amount)
}

/// Function to check balance is not frozen
pub fn ensure_bal_not_frozen(
    deps: Deps,
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

/// Function to check wheather the user has admin access or not
pub fn is_admin(deps: &DepsMut, addr: Addr) -> Result<(), ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    ensure!(admin == addr, ContractError::NotAdmin { address: addr });

    Ok(())
}

/// Function to check wheather the user has sub_admin access or not
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

/// Function to check wheather the user is operator or not
pub fn is_operator_of(deps: &DepsMut, address: String) -> Result<(), ContractError> {
    OPERATORS.load(deps.storage).map_or(
        Err(ContractError::NotAnOperator {
            address: address.clone(),
        }),
        |operators| {
            if !operators.contains(&address) {
                Err(ContractError::NotAnOperator { address })
            } else {
                Ok(())
            }
        },
    )
}

/// Function to check wheather the user has issuer access or not
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

/// Function to check wheather the user has transfer agent access or not
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

/// Function to check wheather the user has tokenization agent access or not
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

/// Helper function to create new marker with Finalized state
pub fn create_marker<S: Into<String> + Clone>(
    denom: S,
    _contract_address: Addr,
) -> StdResult<Vec<CosmosMsg<ProvenanceMsg>>> {
    let coin = cosmwasm_std::Coin {
        amount: Uint128::zero(),
        denom: validate_string(denom.clone(), "denom")?,
    };

    let msgs = vec![
        CosmosMsg::Custom(ProvenanceMsg {
            route: provwasm_std::ProvenanceRoute::Marker,
            params: ProvenanceMsgParams::Marker(MarkerMsgParams::CreateMarker {
                coin: coin.clone(),
                marker_type: MarkerType::Restricted,
                allow_forced_transfer: true,
            }),
            version: String::from("1.0"),
        }),
        CosmosMsg::Custom(ProvenanceMsg {
            route: provwasm_std::ProvenanceRoute::Marker,
            params: ProvenanceMsgParams::Marker(MarkerMsgParams::ActivateMarker {
                denom: validate_string(denom, "denom")?,
            }),
            version: String::from("1.0"),
        }),
    ];

    Ok(msgs)
}

// Function to provide marker accesses to an address
pub fn grant_marker_access<S: Into<String>, H: Into<Addr>>(
    denom: S,
    address: H,
    permissions: Vec<MarkerAccess>,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(CosmosMsg::Custom(ProvenanceMsg {
        route: provwasm_std::ProvenanceRoute::Marker,
        params: ProvenanceMsgParams::Marker(MarkerMsgParams::GrantMarkerAccess {
            denom: validate_string(denom, "denom")?,
            address: validate_address(address)?,
            permissions,
        }),
        version: String::from("1.0"),
    }))
}

/// Helper function for coin withdrawal
pub fn withdraw_coins<S: Into<String> + Clone, H: Into<Addr>>(
    denom: S,
    amount: u128,
    recipient: H,
    _contract_address: Addr,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    if amount == 0 {
        return Err(StdError::generic_err("withdraw amount must be > 0"));
    }
    let coin = cosmwasm_std::Coin {
        denom: validate_string(denom.clone(), "denom")?,
        amount: Uint128::from(amount),
    };
    Ok(CosmosMsg::Custom(ProvenanceMsg {
        route: provwasm_std::ProvenanceRoute::Marker,
        params: ProvenanceMsgParams::Marker(MarkerMsgParams::WithdrawCoins {
            marker_denom: validate_string(denom, "denom")?,
            coin,
            recipient: validate_address(recipient)?,
        }),
        version: String::from("1.0"),
    }))
}

/// Helper function for increasing marker supply
pub fn mint_marker_supply<S: Into<String>>(
    amount: u128,
    denom: S,
    _contract_address: Addr,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    if amount == 0 {
        return Err(StdError::generic_err("mint amount must be > 0"));
    }
    let coin = cosmwasm_std::Coin {
        denom: validate_string(denom, "denom")?,
        amount: Uint128::from(amount),
    };

    Ok(CosmosMsg::Custom(ProvenanceMsg {
        route: provwasm_std::ProvenanceRoute::Marker,
        params: ProvenanceMsgParams::Marker(MarkerMsgParams::MintMarkerSupply { coin }),
        version: String::from("1.0"),
    }))
}

/// Helper function for decreasing marker supply
pub fn burn_marker_supply<S: Into<String>>(
    amount: u128,
    denom: S,
    _contract_address: Addr,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    if amount == 0 {
        return Err(StdError::generic_err("burn amount must be > 0"));
    }
    let coin = cosmwasm_std::Coin {
        denom: validate_string(denom, "denom")?,
        amount: Uint128::from(amount),
    };
    Ok(CosmosMsg::Custom(ProvenanceMsg {
        route: provwasm_std::ProvenanceRoute::Marker,
        params: ProvenanceMsgParams::Marker(MarkerMsgParams::BurnMarkerSupply { coin }),
        version: String::from("1.0"),
    }))
}

/// Helper function for cancelling marker
pub fn cancel_marker<S: Into<String>>(
    denom: S,
    _contract_address: Addr,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(CosmosMsg::Custom(ProvenanceMsg {
        route: provwasm_std::ProvenanceRoute::Marker,
        params: ProvenanceMsgParams::Marker(MarkerMsgParams::CancelMarker {
            denom: validate_string(denom, "denom")?,
        }),
        version: String::from("1.0"),
    }))
}

/// Helper function for destroying marker
pub fn destroy_marker<S: Into<String>>(
    denom: S,
    _contract_address: Addr,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(CosmosMsg::Custom(ProvenanceMsg {
        route: provwasm_std::ProvenanceRoute::Marker,
        params: ProvenanceMsgParams::Marker(MarkerMsgParams::DestroyMarker {
            denom: validate_string(denom, "denom")?,
        }),
        version: String::from("1.0"),
    }))
}

/// Helper function for transferring marker coins
pub fn transfer_marker_coins<S: Into<String>, H: Into<Addr>>(
    amount: u128,
    denom: S,
    to: H,
    from: H,
    _contract_address: H,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    if amount == 0 {
        return Err(StdError::generic_err("transfer amount must be > 0"));
    }
    let coin = cosmwasm_std::Coin {
        denom: validate_string(denom, "denom")?,
        amount: Uint128::from(amount),
    };
    Ok(CosmosMsg::Custom(ProvenanceMsg {
        route: provwasm_std::ProvenanceRoute::Marker,
        params: ProvenanceMsgParams::Marker(MarkerMsgParams::TransferMarkerCoins {
            coin,
            to: validate_address(to)?,
            from: validate_address(from)?,
        }),
        version: String::from("1.0"),
    }))
}

/// Helper function to get marker by an address
pub fn get_marker_by_address<H: Into<Addr>>(
    address: H,
    querier: &provwasm_std::ProvenanceQuerier,
) -> StdResult<Marker> {
    get_marker_by_denom(validate_address(address)?.to_string(), querier)
}

/// Helper function to get marker by denom
pub fn get_marker_by_denom<H: Into<String>>(
    denom: H,
    querier: &provwasm_std::ProvenanceQuerier,
) -> StdResult<Marker> {
    let denom_str = validate_string(denom, "denom")?;
    querier.get_marker_by_denom(&denom_str)
}

/// Helper function to get marker
pub fn get_marker(id: String, querier: &provwasm_std::ProvenanceQuerier) -> StdResult<Marker> {
    querier.get_marker_by_denom(&id)
}

/// Helper function to get marker address
pub fn get_marker_address(
    id: String,
    querier: &provwasm_std::ProvenanceQuerier,
) -> StdResult<Addr> {
    let marker = querier.get_marker_by_denom(&id)?;
    Ok(Addr::unchecked(marker.address))
}

pub fn access() -> Vec<MarkerAccess> {
    vec![
        MarkerAccess::Admin,
        MarkerAccess::Burn,
        MarkerAccess::Deposit,
        MarkerAccess::Delete,
        MarkerAccess::Mint,
        MarkerAccess::Transfer,
        MarkerAccess::Withdraw,
    ]
}

/// Helper function to provide all the accesses
pub fn all_access(address: &Addr) -> Vec<AccessGrant> {
    vec![AccessGrant {
        address: address.clone(),
        permissions: access(),
    }]
}

/// Helper function to provide all access to addresses
pub fn all_access_to_addresses(addresses: &[Addr], is_all_access: bool) -> Vec<AccessGrant> {
    let mut access_grant = Vec::new();

    let permissions = if is_all_access {
        access()
    } else {
        Vec::default()
    };

    addresses.iter().all(|addr| {
        access_grant.push(AccessGrant {
            address: addr.clone(),
            permissions: permissions.clone(),
        });
        true
    });

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

/// Helper function to update mint balances
pub fn update_mint_balances(
    storage: &mut dyn Storage,
    address: Addr,
    update_type: UpdateType<Uint128>,
) -> Result<(), ContractError> {
    match update_type {
        UpdateType::Add(amount) => {
            match MINT_BALANCES.update(
                storage,
                address.clone(),
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
                Err(_) => MINT_BALANCES.save(storage, address, &amount)?,
            };
        }
        UpdateType::Remove(amount) => {
            MINT_BALANCES.update(storage, address, |bals_opt| -> Result<_, ContractError> {
                match bals_opt {
                    Some(mut bals) => Ok({
                        bals -= amount;
                        bals
                    }),
                    None => Ok(amount),
                }
            })?;
        }
    }

    Ok(())
}

/// Helper function to update burn balances
pub fn update_burn_balances(
    storage: &mut dyn Storage,
    address: Addr,
    update_type: UpdateType<Uint128>,
) -> Result<(), ContractError> {
    match update_type {
        UpdateType::Add(amount) => {
            match BURN_BALANCES.update(
                storage,
                address.clone(),
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
                Err(_) => BURN_BALANCES.save(storage, address, &amount)?,
            };
        }
        UpdateType::Remove(amount) => {
            BURN_BALANCES.update(storage, address, |bals_opt| -> Result<_, ContractError> {
                match bals_opt {
                    Some(mut bals) => Ok({
                        bals -= amount;
                        bals
                    }),
                    None => Ok(amount),
                }
            })?;
        }
    }

    Ok(())
}

/// Helper function for mint to
pub fn mint_to(
    denom: String,
    data: MintBurnData,
    contract_address: Addr,
) -> Result<Vec<CosmosMsg<ProvenanceMsg>>, ContractError> {
    let msgs = vec![
        mint_marker_supply(data.amount.u128(), denom.clone(), contract_address.clone())?,
        withdraw_coins(&denom, data.amount.u128(), data.address, contract_address)?,
    ];

    Ok(msgs)
}

/// Helper function for burn from
pub fn burn_from(
    denom: String,
    data: MintBurnData,
    contract_address: Addr,
    querier: &provwasm_std::ProvenanceQuerier,
) -> Result<Vec<CosmosMsg<ProvenanceMsg>>, ContractError> {
    let marker_addr = get_marker_address(validate_string(denom.clone(), "denom")?, querier)?;

    let msgs = vec![
        transfer_marker_coins(
            data.amount.u128(),
            &denom,
            marker_addr,
            data.address.clone(),
            contract_address.clone(),
        )?,
        burn_marker_supply(data.amount.u128(), &denom, contract_address)?,
    ];

    Ok(msgs)
}

/// Helper function to create ibc response
pub fn create_response(
    dest_config: DestConfig,
    contract_addr: Addr,
    message: String,
    funds: Vec<cosmwasm_std::Coin>,
) -> Result<CosmosMsg<ProvenanceMsg>, ContractError> {
    let exe_msg = match dest_config.chain.as_str() {
        "Polygon" => ExecuteMsg::SendMessageEvm {
            message,
            destination_chain: dest_config.chain,
            destination_address: dest_config.address,
            msg_type: MessageType::Message,
        },
        _ => ExecuteMsg::SendMessageCosmos {
            message,
            destination_chain: dest_config.chain,
            destination_address: dest_config.address,
            msg_type: MessageType::Message,
        },
    };

    Ok(CosmosMsg::Wasm(wasm_execute(
        contract_addr,
        &exe_msg,
        funds,
    )?))
}
