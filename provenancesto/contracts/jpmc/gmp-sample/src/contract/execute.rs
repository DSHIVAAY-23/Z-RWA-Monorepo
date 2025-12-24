#[cfg(not(feature = "library"))]
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
        SendFund { to, amount } => try_send_fund(to, amount),
        Call { to, amount } => try_call(env.contract.address, to, amount),
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
    let message_payload = encode(&vec![
        Token::String(info.sender.to_string()),
        Token::String(message),
    ]);

    // {info.funds} used to pay gas. Must only contain 1 token type.
    let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

    let gmp_message: GmpMessage = GmpMessage {
        destination_chain,
        destination_address,
        payload: message_payload.to_vec(),
        type_: msg_type.into_i64(),
        fee: None,
    };

    let ibc_message = crate::ibc::MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: "channel-1".to_string(),
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

    let gmp_message: GmpMessage = GmpMessage {
        destination_chain,
        destination_address,
        payload: message_payload.to_vec(),
        type_: msg_type.into_i64(), //type = 1
        fee: None,
    };

    // info.funds used to pay gas. Must only contain 1 token type.
    let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

    let ibc_message = crate::ibc::MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: "channel-1".to_string(),
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
    let decoded = decode(
        &vec![ParamType::String, ParamType::String],
        payload.as_slice(),
    )
    .unwrap();

    // store message
    STORED_MESSAGE.save(
        deps.storage,
        &Message {
            sender: decoded[0].to_string(),
            message: decoded[1].to_string(),
        },
    )?;

    Ok(Response::new())
}

pub fn receive_message_cosmos(
    deps: DepsMut,
    sender: String,
    message: String,
) -> Result<Response, ContractError> {
    // store message
    STORED_MESSAGE.save(deps.storage, &Message { sender, message })?;

    Ok(Response::new())
}

fn try_send_fund(to: String, amount: Uint128) -> Result<Response, ContractError> {
    // Transfer stable coins from agent account
    let msg: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
        to_address: to,
        amount: vec![coin(amount.u128(), "vspn")],
    });

    Ok(Response::new().add_message(msg))
}

fn try_call(contract_addr: Addr, to: String, amount: Uint128) -> Result<Response, ContractError> {
    let exe_msg = ExecuteMsg::SendFund { to, amount };
    let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(contract_addr, &exe_msg, vec![])?);
    Ok(Response::new().add_message(msg))
}
