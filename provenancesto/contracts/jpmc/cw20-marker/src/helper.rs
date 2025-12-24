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

pub fn get_consolidated_balance(deps: Deps, address: Addr) -> StdResult<Uint128> {
    let denom = get_denom(deps.storage)?;
    let bal = deps.querier.query_balance(address, denom)?;

    Ok(bal.amount)
}

pub fn ensure_bal_not_frozen(deps: Deps, address: Addr) -> Result<(), ContractError> {
    let denom = get_denom(deps.storage)?;
    let bal = get_consolidated_balance(deps, address.clone())?;
    let frozen_bal = PARTIAL_FREEZE
        .load(deps.storage, address.clone())
        .unwrap_or(Uint128::default());

    check_bal_avalaility(
        frozen_bal,
        bal,
        ContractError::BalanceFrozen { denom, address },
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
                let err = format!("Account: `{}` Freezed!", addr);
                return Err(ContractError::Unauthorized { err });
            }
        }
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

pub fn create_marker<S: Into<String> + Clone>(
    supply: u128,
    denom: S,
    contract_address: Addr,
) -> StdResult<Vec<CosmosMsg>> {
    let coin = Coin {
        amount: supply.to_string(),
        denom: validate_string(denom.clone(), "denom")?,
    };

    let msgs = vec![
        MsgAddMarkerRequest {
            amount: Some(coin),
            manager: validate_address(contract_address.clone())?.to_string(),
            from_address: validate_address(contract_address.clone())?.to_string(),
            status: MarkerStatus::Finalized.into(),
            marker_type: MarkerType::Restricted.into(),
            access_list: all_access(&contract_address),
            supply_fixed: false,
            allow_governance_control: false,
            allow_forced_transfer: true,
            required_attributes: vec![],
        }
        .into(),
        MsgActivateRequest {
            denom: validate_string(denom, "denom")?,
            administrator: validate_address(contract_address)?.to_string(),
        }
        .into(),
    ];

    Ok(msgs)
}

pub fn withdraw_coins<S: Into<String> + Clone, H: Into<Addr>>(
    denom: S,
    amount: u128,
    recipient: H,
    contract_address: Addr,
) -> StdResult<CosmosMsg> {
    if amount == 0 {
        return Err(StdError::generic_err("withdraw amount must be > 0"));
    }
    let coin = Coin {
        denom: validate_string(denom.clone(), "denom")?,
        amount: amount.to_string(),
    };
    Ok(MsgWithdrawRequest {
        denom: validate_string(denom, "denom")?,
        administrator: validate_address(contract_address)?.to_string(),
        to_address: validate_address(recipient)?.to_string(),
        amount: vec![coin],
    }
    .into())
}

pub fn mint_marker_supply<S: Into<String>>(
    amount: u128,
    denom: S,
    contract_address: Addr,
) -> StdResult<CosmosMsg> {
    if amount == 0 {
        return Err(StdError::generic_err("mint amount must be > 0"));
    }
    let coin = Coin {
        denom: validate_string(denom, "denom")?,
        amount: amount.to_string(),
    };

    Ok(MsgMintRequest {
        amount: Some(coin),
        administrator: validate_address(contract_address)?.to_string(),
    }
    .into())
}

pub fn burn_marker_supply<S: Into<String>>(
    amount: u128,
    denom: S,
    contract_address: Addr,
) -> StdResult<CosmosMsg> {
    if amount == 0 {
        return Err(StdError::generic_err("burn amount must be > 0"));
    }
    let coin = Coin {
        denom: validate_string(denom, "denom")?,
        amount: amount.to_string(),
    };
    Ok(MsgBurnRequest {
        amount: Some(coin),
        administrator: validate_address(contract_address)?.to_string(),
    }
    .into())
}

pub fn transfer_marker_coins<S: Into<String>, H: Into<Addr>>(
    amount: u128,
    denom: S,
    to: H,
    from: H,
    contract_address: H,
) -> StdResult<CosmosMsg> {
    if amount == 0 {
        return Err(StdError::generic_err("transfer amount must be > 0"));
    }
    let coin = Coin {
        denom: validate_string(denom, "denom")?,
        amount: amount.to_string(),
    };
    Ok(MsgTransferRequest {
        amount: Some(coin),
        administrator: contract_address.into().to_string(),
        from_address: validate_address(from)?.to_string(),
        to_address: validate_address(to)?.to_string(),
    }
    .into())
}

pub fn get_marker_by_address<H: Into<Addr>>(
    address: H,
    querier: &MarkerQuerier<Empty>,
) -> StdResult<Marker> {
    get_marker(validate_address(address)?.to_string(), querier)
}

pub fn get_marker_by_denom<H: Into<String>>(
    denom: H,
    querier: &MarkerQuerier<Empty>,
) -> StdResult<Marker> {
    get_marker(validate_string(denom, "denom")?, querier)
}

pub fn get_marker(id: String, querier: &MarkerQuerier<Empty>) -> StdResult<Marker> {
    let response = querier.marker(id)?;
    if let Some(marker) = response.marker {
        return if let Ok(account) = MarkerAccount::try_from(marker) {
            let escrow = querier.escrow(account.clone().base_account.unwrap().address)?;
            Ok(Marker {
                marker_account: account,
                coins: escrow.escrow,
            })
        } else {
            Err(StdError::generic_err("unable to type-cast marker account"))
        };
    }
    Err(StdError::generic_err("no marker found for id"))
}

pub fn get_marker_address(id: String, querier: &MarkerQuerier<Empty>) -> StdResult<Addr> {
    let response = querier.marker(id)?;
    if let Some(marker) = response.marker {
        return if let Ok(account) = MarkerAccount::try_from(marker) {
            Ok(Addr::unchecked(account.base_account.unwrap().address))
        } else {
            Err(StdError::generic_err("unable to type-cast marker account"))
        };
    }
    Err(StdError::generic_err("no marker found for id"))
}

pub fn all_access(address: &Addr) -> Vec<AccessGrant> {
    vec![AccessGrant {
        address: address.to_string(),
        permissions: vec![
            Access::Admin.into(),
            Access::Burn.into(),
            Access::Deposit.into(),
            Access::Delete.into(),
            Access::Mint.into(),
            Access::Transfer.into(),
            Access::Withdraw.into(),
        ],
    }]
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

pub fn update_burn_balances(
    storage: &mut dyn Storage,
    address: Addr,
    update_type: UpdateType<Uint128>,
) -> Result<(), StdError> {
    match update_type {
        UpdateType::Add(amount) => {
            if BURN_BALANCES
                .update(
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
                )
                .is_err()
            {
                BURN_BALANCES.save(storage, address, &amount)?;
            }
        }
        UpdateType::Remove(amount) => {
            BURN_BALANCES.update(storage, address, |bals_opt| -> Result<_, StdError> {
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

pub fn get_denom(storage: &dyn Storage) -> Result<String, StdError> {
    let denom = DENOM.load(storage)?;

    Ok(denom)
}

pub fn is_tokenization_agent(deps: &DepsMut, sender: Addr) -> Result<(), ContractError> {
    let agent = TOKENIZATION_AGENT.load(deps.storage)?;
    if agent.ne(&sender) {
        let err = format!("`{}` in not tokenization agent!", sender);
        return Err(ContractError::Unauthorized { err });
    }

    Ok(())
}

pub fn get_allowance(
    storage: &dyn Storage,
    owner: Addr,
    spender: Addr,
) -> Result<Uint128, StdError> {
    let key = Key::new(owner, spender).as_bytes()?;
    let rem_bal = ALLOWANCE.load(storage, &key).unwrap_or(Uint128::default());

    Ok(rem_bal)
}

pub fn mint_to(
    denom: String,
    data: Data,
    contract_address: Addr,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let msgs = vec![
        mint_marker_supply(data.amount.u128(), denom.clone(), contract_address.clone())?,
        withdraw_coins(&denom, data.amount.u128(), data.address, contract_address)?,
    ];

    Ok(msgs)
}

pub fn create_response(
    dest_config: DestConfig,
    contract_addr: Addr,
    message: String,
) -> Result<CosmosMsg, ContractError> {
    let exe_msg = match dest_config.chain.as_str() {
        "onyx" => ExecuteMsg::SendMessageEvm {
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
        vec![coin(1u128, VSPN)],
    )?))
}
