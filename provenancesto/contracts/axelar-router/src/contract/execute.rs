use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        SendMessageEvm {
            destination_chain,
            destination_address,
            message,
            msg_type,
        } => send_message_evm(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
            msg_type,
        ),
        SendMessageCosmos {
            destination_chain,
            destination_address,
            message,
            msg_type,
        } => send_message_cosmos(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
            msg_type,
        ),
        ReceiveMessageEvm {
            source_chain,
            source_address,
            payload,
        } => receive_message_evm(deps, source_chain, source_address, payload),
        ReceiveMessageCosmos { sender, message } => receive_message_cosmos(deps, sender, message),
        UpdateDestConfig { config } => update_dest_config(deps, config),
        ClearStoredMessage {} => clear_stored_msg(deps),
        UpdateOperators { update_type } => try_update_operators(deps, info, update_type),
    }
}

// Sends a message via Axelar GMP to the EVM {destination_chain} and {destination_address}
pub fn send_message_evm(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    message: String,
    msg_type: MessageType,
) -> Result<Response, ContractError> {
    // Message payload to be received by the destination
    let message_payload = encode(&[
        Token::String(info.sender.to_string()),
        Token::String(message),
    ]);

    // {info.funds} used to pay gas. Must only contain 1 token type.
    let coin: cosmwasm_std::Coin =
        cw_utils::one_coin(&info).expect("Atleast one coin should be present!");

    let gmp_message: GmpMessage = GmpMessage {
        destination_chain,
        destination_address,
        payload: message_payload.to_vec(),
        type_: msg_type.into_i64(),
        fee: Some(Fee {
            amount: coin.amount.to_string(),
            recipient: String::from("axelar1zl3rxpp70lmte2xr6c4lgske2fyuj3hupcsvcd"),
        }),
    };

    let ibc_message = crate::ibc::MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: "channel-75".to_string(),
        token: Some(coin.into()),
        sender: env.contract.address.to_string(),
        receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5".to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
        memo: to_string(&gmp_message).unwrap(),
    };

    Ok(Response::new().add_message(ibc_message))
}

// Sends a message via Axelar GMP to the other cosmos chains
// only difference is how the {message_payload} is constructed
pub fn send_message_cosmos(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination_chain: String,
    destination_address: String,
    message: String,
    msg_type: MessageType,
) -> Result<Response, ContractError> {
    // Construct contract call
    let contract_call = serde_json_wasm::to_string(&ExecuteMsg::ReceiveMessageCosmos {
        sender: info.sender.to_string(),
        message,
    })
    .expect("Failed to serialize struct to JSON");
    let utf8_bytes = contract_call.as_bytes();
    let utf8_vec = utf8_bytes.to_owned();
    // prepend 4 bytes to indicate the payload verison
    let mut message_payload: Vec<u8> = vec![0, 0, 0, 2];
    message_payload.extend(utf8_vec);

    // info.funds used to pay gas. Must only contain 1 token type.
    let coin: cosmwasm_std::Coin =
        cw_utils::one_coin(&info).expect("Atleast one coin should be present!");

    let gmp_message: GmpMessage = GmpMessage {
        destination_chain,
        destination_address,
        payload: message_payload.to_vec(),
        type_: msg_type.into_i64(), //type = 1
        fee: Some(Fee {
            amount: coin.amount.to_string(),
            recipient: String::from("axelar1zl3rxpp70lmte2xr6c4lgske2fyuj3hupcsvcd"),
        }),
    };

    let ibc_message = crate::ibc::MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: "channel-75".to_string(),
        token: Some(coin.into()),
        sender: env.contract.address.to_string(),
        receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5".to_string(),
        timeout_height: None,
        timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
        memo: to_string(&gmp_message).unwrap(),
    };

    Ok(Response::new().add_message(ibc_message))
}

pub fn receive_message_evm(
    deps: DepsMut,
    _source_chain: String,
    _source_address: String,
    payload: Binary,
) -> Result<Response, ContractError> {
    // decode the payload
    // executeMsgPayload: [sender, message]
    let decoded = decode(&[ParamType::String, ParamType::String], payload.as_slice()).unwrap();
    // let dest_config = DEST_CONFIG.load(deps.storage)?;

    // store message
    let store_msg = Message {
        sender: decoded[0].to_string(),
        message: decoded[1].to_string(),
    };

    // // Authorizing sender
    // if let Err(err) = is_operator_of(&deps, store_msg.sender.to_string()) {
    //     return Ok(Response::new().add_message(create_response(
    //         contract_addr,
    //         dest_config.chain,
    //         err.to_string(),
    //     )?));
    // };

    if STORED_MESSAGE
        .update(deps.storage, |mut store| -> Result<_, StdError> {
            Ok({
                store.push(store_msg.clone());
                store
            })
        })
        .is_err()
    {
        STORED_MESSAGE.save(deps.storage, &vec![store_msg.clone()])?
    }

    operation(store_msg.message)
}

pub fn receive_message_cosmos(
    deps: DepsMut,
    sender: String,
    message: String,
) -> Result<Response, ContractError> {
    // store message
    let store_msg = Message { sender, message };
    // let dest_config = DEST_CONFIG.load(deps.storage)?;

    // // Authorizing sender
    // if let Err(err) = is_operator_of(&deps, store_msg.sender.to_string()) {
    //     return Ok(Response::new().add_message(create_response(
    //         contract_addr,
    //         dest_config.chain,
    //         err.to_string(),
    //     )?));
    // };

    if STORED_MESSAGE
        .update(deps.storage, |mut store| -> Result<_, StdError> {
            Ok({
                store.push(store_msg.clone());
                store
            })
        })
        .is_err()
    {
        STORED_MESSAGE.save(deps.storage, &vec![store_msg.clone()])?
    }

    operation(store_msg.message)
}

pub fn update_dest_config(deps: DepsMut, config: DestConfig) -> Result<Response, ContractError> {
    // store destination configuration
    DEST_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

pub fn clear_stored_msg(deps: DepsMut) -> Result<Response, ContractError> {
    STORED_MESSAGE.update(deps.storage, |mut msgs| -> Result<_, StdError> {
        Ok({
            msgs.clear();
            msgs
        })
    })?;

    Ok(Response::new())
}

// Function to manage different roles
pub fn try_update_operators(
    deps: DepsMut,
    info: MessageInfo,
    update_type: UpdateType<Vec<String>>,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();

    is_operator_of(&deps, info.sender.to_string())?;

    match update_type {
        UpdateType::Add(addrs) => {
            let updated =
                OPERATORS.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                    addresses.extend(addrs.clone());
                    addresses.sort();
                    addresses.dedup();
                    Ok(addresses)
                });
            if updated.is_err() {
                OPERATORS.save(deps.storage, &addrs)?;
            };
            attrs.push(attr("action", "router.contract.add_operators"));
        }
        UpdateType::Remove(addrs) => {
            OPERATORS.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                addresses.retain(|addr| !addrs.contains(addr));
                Ok(addresses)
            })?;
            attrs.push(attr("action", "router.contract.remove_operators"));
        }
    }

    Ok(Response::new().add_attributes(attrs))
}
