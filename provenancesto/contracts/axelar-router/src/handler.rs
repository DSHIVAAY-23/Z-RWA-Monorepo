use super::*;

pub fn operation(message: String) -> Result<Response, ContractError> {
    let op: Vec<&str> = message.split('|').collect();

    if op.len() == 1 {
        return Ok(Response::new());
    }

    let mut msgs = Vec::new();
    match op[0] {
        "mint" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::RequestOrder {
                order_id: op[1].to_string(),
                denom: op[2].to_string(),
                from: Addr::unchecked(op[3]),
                amount: Uint128::from(
                    op[4]
                        .parse::<u128>()
                        .expect("Error while parsing in `u128`!"),
                ),
                request_type: cw20_marker::enums::RequestType::Mint,
            };
            let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
                TOKEN_CONTRACT,
                &exe_msg,
                vec![Coin {
                    denom: DENOM.to_string(),
                    amount: Uint128::from(3000000u128),
                }],
            )?);
            msgs.push(msg);
        }
        "burn" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::RequestOrder {
                order_id: op[1].to_string(),
                denom: op[2].to_string(),
                from: Addr::unchecked(op[3]),
                amount: Uint128::from(
                    op[4]
                        .parse::<u128>()
                        .expect("Error while parsing in `u128`!"),
                ),
                request_type: cw20_marker::enums::RequestType::Burn,
            };
            let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
                TOKEN_CONTRACT,
                &exe_msg,
                vec![Coin {
                    denom: DENOM.to_string(),
                    amount: Uint128::from(3000000u128),
                }],
            )?);
            msgs.push(msg);
        }
        _ => (),
    }

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", op[0]))
}
