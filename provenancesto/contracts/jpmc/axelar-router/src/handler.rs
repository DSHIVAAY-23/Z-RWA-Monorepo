use super::*;

pub fn operation(
    deps: Deps,
    message: String,
    contract_addr: Addr,
) -> Result<Response, ContractError> {
    let op: Vec<&str> = message.split('|').collect();

    if op.len() == 1 {
        return Ok(Response::new());
    }

    let token_contract_addr = get_token_contract_addr(deps, op[1].to_string())?;
    let dest_config = DEST_CONFIG.load(deps.storage)?;

    let mut msgs = Vec::new();
    match op[0] {
        "requestMint" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::Request {
                request_id: op[2].to_string(),
                amount: Uint128::from(
                    op[3]
                        .parse::<u128>()
                        .expect("Error while parsing in `u128`!"),
                ),
                request_type: cw20_marker::enums::RequestType::Mint,
            };
            let msg: CosmosMsg =
                CosmosMsg::Wasm(wasm_execute(token_contract_addr, &exe_msg, Vec::default())?);
            msgs.push(msg);
        }
        "requestBurn" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::Request {
                request_id: op[2].to_string(),
                amount: Uint128::from(
                    op[3]
                        .parse::<u128>()
                        .expect("Error while parsing in `u128`!"),
                ),
                request_type: cw20_marker::enums::RequestType::Burn,
            };
            let msg: CosmosMsg =
                CosmosMsg::Wasm(wasm_execute(token_contract_addr, &exe_msg, Vec::default())?);
            msgs.push(msg);
        }
        "requestMintFrom" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::RequestFrom {
                request_id: op[2].to_string(),
                from: Addr::unchecked(op[3]),
                amount: Uint128::from(
                    op[4]
                        .parse::<u128>()
                        .expect("Error while parsing in `u128`!"),
                ),
                request_type: cw20_marker::enums::RequestType::Mint,
            };
            let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
                token_contract_addr,
                &exe_msg,
                vec![coin(1u128, VSPN)],
            )?);
            msgs.push(msg);
        }
        "requestBurnFrom" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::RequestFrom {
                request_id: op[2].to_string(),
                from: Addr::unchecked(op[3]),
                amount: Uint128::from(
                    op[4]
                        .parse::<u128>()
                        .expect("Error while parsing in `u128`!"),
                ),
                request_type: cw20_marker::enums::RequestType::Burn,
            };
            let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
                token_contract_addr,
                &exe_msg,
                vec![coin(1u128, VSPN)],
            )?);
            msgs.push(msg);
        }
        "approveMint" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::ApproveRequest {
                request_id: op[2].to_string(),
                request_type: cw20_marker::enums::RequestType::Mint,
            };
            let msg: CosmosMsg =
                CosmosMsg::Wasm(wasm_execute(token_contract_addr, &exe_msg, Vec::default())?);
            msgs.push(msg);
        }
        "approveBurn" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::ApproveRequest {
                request_id: op[2].to_string(),
                request_type: cw20_marker::enums::RequestType::Burn,
            };
            let msg: CosmosMsg =
                CosmosMsg::Wasm(wasm_execute(token_contract_addr, &exe_msg, Vec::default())?);
            msgs.push(msg);
        }
        "rejectMint" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::RejectRequest {
                request_id: op[2].to_string(),
                request_type: cw20_marker::enums::RequestType::Mint,
            };
            let msg: CosmosMsg =
                CosmosMsg::Wasm(wasm_execute(token_contract_addr, &exe_msg, Vec::default())?);
            msgs.push(msg);
        }
        "rejectBurn" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::RejectRequest {
                request_id: op[2].to_string(),
                request_type: cw20_marker::enums::RequestType::Burn,
            };
            let msg: CosmosMsg =
                CosmosMsg::Wasm(wasm_execute(token_contract_addr, &exe_msg, Vec::default())?);
            msgs.push(msg);
        }
        "increaseRequestMintAllowance" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::ManageRequestAllowance {
                spender: Addr::unchecked(op[2]),
                update_type: cw20_marker::enums::UpdateType::Add(Uint128::from(
                    op[3]
                        .parse::<u128>()
                        .expect("Error while parsing in `u128`!"),
                )),
                request_type: cw20_marker::enums::RequestType::Mint,
            };
            let msg: CosmosMsg =
                CosmosMsg::Wasm(wasm_execute(token_contract_addr, &exe_msg, Vec::default())?);
            msgs.push(msg);
        }
        "decreaseRequestMintAllowance" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::ManageRequestAllowance {
                spender: Addr::unchecked(op[2]),
                update_type: cw20_marker::enums::UpdateType::Remove(Uint128::from(
                    op[3]
                        .parse::<u128>()
                        .expect("Error while parsing in `u128`!"),
                )),
                request_type: cw20_marker::enums::RequestType::Mint,
            };
            let msg: CosmosMsg =
                CosmosMsg::Wasm(wasm_execute(token_contract_addr, &exe_msg, Vec::default())?);
            msgs.push(msg);
        }
        "increaseRequestBurnAllowance" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::ManageRequestAllowance {
                spender: Addr::unchecked(op[2]),
                update_type: cw20_marker::enums::UpdateType::Add(Uint128::from(
                    op[3]
                        .parse::<u128>()
                        .expect("Error while parsing in `u128`!"),
                )),
                request_type: cw20_marker::enums::RequestType::Burn,
            };
            let msg: CosmosMsg =
                CosmosMsg::Wasm(wasm_execute(token_contract_addr, &exe_msg, Vec::default())?);
            msgs.push(msg);
        }
        "decreaseRequestBurnAllowance" => {
            let exe_msg = cw20_marker::msg::ExecuteMsg::ManageRequestAllowance {
                spender: Addr::unchecked(op[2]),
                update_type: cw20_marker::enums::UpdateType::Remove(Uint128::from(
                    op[3]
                        .parse::<u128>()
                        .expect("Error while parsing in `u128`!"),
                )),
                request_type: cw20_marker::enums::RequestType::Burn,
            };
            let msg: CosmosMsg =
                CosmosMsg::Wasm(wasm_execute(token_contract_addr, &exe_msg, Vec::default())?);
            msgs.push(msg);
        }
        "totalSupply" => {
            let query_msg = cw20_marker::msg::QueryMsg::GetTotalSupply {};
            let coins: Coin = deps
                .querier
                .query_wasm_smart(token_contract_addr, &query_msg)?;

            let exe_msg = get_exec_msg(
                dest_config.chain,
                dest_config.address,
                coins.amount.to_string(),
            );

            let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
                contract_addr,
                &exe_msg,
                vec![coin(1u128, VSPN)],
            )?);
            msgs.push(msg);
        }
        "balanceOf" => {
            let query_msg = cw20_marker::msg::QueryMsg::GetBalanceOf {
                address: Addr::unchecked(op[2]),
            };
            let coins: Coin = deps
                .querier
                .query_wasm_smart(token_contract_addr, &query_msg)?;

            let exe_msg = get_exec_msg(
                dest_config.chain,
                dest_config.address,
                coins.amount.to_string(),
            );

            let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
                contract_addr,
                &exe_msg,
                vec![coin(1u128, VSPN)],
            )?);
            msgs.push(msg);
        }
        "mintRequestOf" => {
            let query_msg = cw20_marker::msg::QueryMsg::GetRequestOf {
                request_id: op[2].to_string(),
            };
            let req: Result<cw20_marker::structs::Request, _> = deps
                .querier
                .query_wasm_smart(token_contract_addr, &query_msg);

            let res_msg = if let Ok(msg) = req {
                format!("{:#?}", msg)
            } else {
                format!("Error: Request: {} not found!", op[2].to_string())
            };

            let exe_msg = get_exec_msg(dest_config.chain, dest_config.address, res_msg);

            let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
                contract_addr,
                &exe_msg,
                vec![coin(1u128, VSPN)],
            )?);
            msgs.push(msg);
        }
        "burnRequestOf" => {
            let query_msg = cw20_marker::msg::QueryMsg::GetRequestOf {
                request_id: op[2].to_string(),
            };
            let req: Result<cw20_marker::structs::Request, _> = deps
                .querier
                .query_wasm_smart(token_contract_addr, &query_msg);

            let res_msg = if let Ok(msg) = req {
                format!("{:#?}", msg)
            } else {
                format!("Error: Request: {} not found!", op[2].to_string())
            };

            let exe_msg = get_exec_msg(dest_config.chain, dest_config.address, res_msg);

            let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
                contract_addr,
                &exe_msg,
                vec![coin(1u128, VSPN)],
            )?);
            msgs.push(msg);
        }
        "burnBalanceOf" => {
            let query_msg = cw20_marker::msg::QueryMsg::GetBurnBalanceOf {
                owner: Addr::unchecked(op[2]),
            };
            let bal: Uint128 = deps
                .querier
                .query_wasm_smart(token_contract_addr, &query_msg)?;

            let exe_msg = get_exec_msg(dest_config.chain, dest_config.address, bal.to_string());

            let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
                contract_addr,
                &exe_msg,
                vec![coin(1u128, VSPN)],
            )?);
            msgs.push(msg);
        }
        "mintRequestAllowanceOf" => {
            let query_msg = cw20_marker::msg::QueryMsg::GetRequestAllowances {
                owner: Addr::unchecked(op[2]),
                spender: Addr::unchecked(op[3]),
                request_type: cw20_marker::enums::RequestType::Mint,
            };
            let allowance: Uint128 = deps
                .querier
                .query_wasm_smart(token_contract_addr, &query_msg)?;

            let exe_msg = get_exec_msg(
                dest_config.chain,
                dest_config.address,
                allowance.to_string(),
            );

            let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
                contract_addr,
                &exe_msg,
                vec![coin(1u128, VSPN)],
            )?);
            msgs.push(msg);
        }
        "burnRequestAllowanceOf" => {
            let query_msg = cw20_marker::msg::QueryMsg::GetRequestAllowances {
                owner: Addr::unchecked(op[2]),
                spender: Addr::unchecked(op[3]),
                request_type: cw20_marker::enums::RequestType::Burn,
            };
            let allowance: Uint128 = deps
                .querier
                .query_wasm_smart(token_contract_addr, &query_msg)?;

            let exe_msg = get_exec_msg(
                dest_config.chain,
                dest_config.address,
                allowance.to_string(),
            );

            let msg: CosmosMsg = CosmosMsg::Wasm(wasm_execute(
                contract_addr,
                &exe_msg,
                vec![coin(1u128, VSPN)],
            )?);
            msgs.push(msg);
        }
        _ => (),
    }

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", op[0]))
}
