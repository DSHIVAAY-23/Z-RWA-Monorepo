#[cfg(not(feature = "library"))]
use super::*;

/// Handle messages that create and interact with with native provenance markers.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        ManageRoles { roles } => try_manage_roles(deps, info, roles),
        ExecuteTransaction {
            source_chain,
            source_address,
            tx_hash,
            payload,
        } => try_execute_transaction(deps, source_chain, source_address, tx_hash, payload),
        CastVote {
            tx_hash,
            can_transact,
        } => try_cast_vote(deps, info.sender, tx_hash, can_transact),
        UpdateThreshold { threshold } => try_update_threshold(deps, info.sender, threshold),
    }
}

/// Function to manage different roles
/// This function can perform batch operations, hence multiple addresses can be added or removed simultaneously.
/// This entry point can be use to modify different roles such as:-
///     - Admins
///     - Validators
/// Based on the `update_type` field addresses can be added or removed:-
///     - for addition `update_type` will be `UpdateType::Add(Addresses)`
///     - for removal `update_type` will be `UpdateType::Remove(Addresses)`
///
/// Arguements:-
///     - List of Roles, can be either Admins or Validators
///     - UpdateType, can be either Add or Remove
///     - List of addresses
///
/// Fails when:-
///     - caller is not admin,
///     - ADMINS map is empty in case of admin removals
///     - VALIDATORS map is empty in case of validator removals
///
/// Based on operation, any event can be emitted:-
///     - provwasm.contracts.interop_multisig.add_admins
///     - provwasm.contracts.interop_multisig.remove_admins
///     - provwasm.contracts.interop_multisig.add_validators
///     - provwasm.contracts.interop_multisig.remove_validators
pub fn try_manage_roles(
    deps: DepsMut,
    info: MessageInfo,
    roles: Vec<Role>,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();
    // Ensuring caller has the admin rights
    is_admin(&deps, info.sender.clone())?;

    for role in roles {
        match role {
            Role::Admins { update_type } => match update_type {
                UpdateType::Add(addrs) => {
                    if ADMINS
                        .update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                            addresses.extend(addrs.clone());
                            addresses.sort();
                            addresses.dedup();
                            Ok(addresses)
                        })
                        .is_err()
                    {
                        ADMINS.save(deps.storage, &addrs)?;
                    };
                    attrs.push(attr(
                        "action",
                        "provwasm.contracts.interop_multisig.add_admins",
                    ));
                }
                UpdateType::Remove(addrs) => {
                    ADMINS.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                        addresses.retain(|addr| !addrs.contains(addr));
                        Ok(addresses)
                    })?;
                    attrs.push(attr(
                        "action",
                        "provwasm.contracts.interop_multisig.remove_admins",
                    ));
                }
            },
            Role::Validators { update_type } => match update_type {
                UpdateType::Add(addrs) => {
                    if VALIDATORS
                        .update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                            addresses.extend(addrs.clone());
                            addresses.sort();
                            addresses.dedup();
                            Ok(addresses)
                        })
                        .is_err()
                    {
                        VALIDATORS.save(deps.storage, &addrs)?;
                    };
                    attrs.push(attr(
                        "action",
                        "provwasm.contracts.interop_multisig.add_validators",
                    ));
                }
                UpdateType::Remove(addrs) => {
                    VALIDATORS.update(
                        deps.storage,
                        |mut addresses| -> Result<_, ContractError> {
                            addresses.retain(|addr| !addrs.contains(addr));
                            Ok(addresses)
                        },
                    )?;
                    attrs.push(attr(
                        "action",
                        "provwasm.contracts.interop_multisig.remove_validators",
                    ));
                }
            },
        }
    }

    Ok(Response::new().add_attributes(attrs))
}

/// Function to execute transaction
/// Anyone can call this function
///
/// Arguements:-
///     - Source Chain
///     - Source Address
///     - Transaction Hash
///     - Payload Data
///
/// Fails when:-
///     - threshold is not met for the transaction
///     - cross program invocation fails
///
/// Emits Event:-
///     - provwasm.contracts.interop_multisig.execute_transaction
pub fn try_execute_transaction(
    deps: DepsMut,
    source_chain: String,
    source_address: String,
    tx_hash: String,
    payload: String,
) -> Result<Response, ContractError> {
    let threshold = THRESHOLD.load(deps.storage)?;

    VOTES.update(
        deps.storage,
        tx_hash.as_bytes(),
        |votes_opt| -> Result<_, ContractError> {
            let mut votes = votes_opt.ok_or(ContractError::NotFound {})?;

            // Ensuring threshold is met
            ensure!(votes.yes >= threshold, ContractError::ThresholdNotMet {});

            // Update status as approved
            votes.set_status(Status::Approved);

            Ok(votes)
        },
    )?;

    // Calling execute instruction of core contract
    let exe_msg = interop_core::msg::ExecuteMsg::ExecuteInstruction {
        source_chain,
        source_address,
        payload,
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(INTEROP_CORE_CONTRACT, &exe_msg, vec![])?);

    Ok(Response::new().add_message(msg).add_attribute(
        "action",
        "provwasm.contracts.interop_multisig.execute_transaction",
    ))
}

/// Function to cast votes
///
/// Arguements:-
///     - Transaction Hash
///     - can_transact can be either true or false
///  
/// Fails when:-
///     - caller is not from validator set
///     - voter is same
///
/// Emits event:-
///     - provwasm.contracts.interop_multisig.cast_vote
pub fn try_cast_vote(
    deps: DepsMut,
    sender: Addr,
    tx_hash: String,
    can_transact: bool,
) -> Result<Response, ContractError> {
    // Ensure caller is one of the validators
    let validators = VALIDATORS.load(deps.storage)?;
    ensure!(
        validators.contains(&sender),
        ContractError::Unauthorized { caller: sender }
    );

    let vote = Votes::new(can_transact, vec![sender.clone()]);
    let threshold = THRESHOLD.load(deps.storage)?;

    if VOTES
        .update(
            deps.storage,
            tx_hash.as_bytes(),
            |votes_opt| -> Result<_, ContractError> {
                match votes_opt {
                    Some(mut votes) => Ok({
                        // Ensuring not the same voter
                        ensure!(
                            !votes.voters.contains(&sender),
                            ContractError::PermissionDenied {}
                        );
                        votes.update(&mut vote.clone());

                        // Update status as Ready once threshold is met
                        if votes.yes >= threshold {
                            votes.set_status(Status::Ready);
                        }
                        votes
                    }),
                    None => Ok(vote.clone()),
                }
            },
        )
        .is_err()
    {
        VOTES.save(deps.storage, tx_hash.as_bytes(), &vote)?;
    }

    Ok(Response::new().add_attribute("action", "provwasm.contracts.interop_multisig.cast_vote"))
}

/// Function to update threshold
///
/// Arguements:-
///     - New threshold value
///
/// Fails when:-
///     - caller is not admin
///
/// Emits event:-
///     - provwasm.contracts.interop_multisig.update_threshold
pub fn try_update_threshold(
    deps: DepsMut,
    sender: Addr,
    threshold: u8,
) -> Result<Response, ContractError> {
    // Ensuring caller has the admin rights
    is_admin(&deps, sender)?;

    let old_threshold = THRESHOLD.load(deps.storage)?;

    THRESHOLD.save(deps.storage, &threshold)?;

    Ok(Response::new()
        .add_attribute(
            "action",
            "provwasm.contracts.interop_multisig.update_threshold",
        )
        .add_attribute("old_threshold", old_threshold.to_string())
        .add_attribute("new_threshold", threshold.to_string()))
}
