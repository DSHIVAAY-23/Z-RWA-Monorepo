use super::*;

pub fn get_token_contract_addr(deps: Deps, denom: String) -> Result<Addr, StdError> {
    let query_msg = marker_factory::msg::QueryMsg::GetContractByDenom { denom };
    let contract_addr: Addr = deps
        .querier
        .query_wasm_smart(FACTORY_CONTRACT_ADDRESS, &query_msg)?;

    Ok(contract_addr)
}

pub fn get_exec_msg(chain: String, address: String, message: String) -> ExecuteMsg {
    match chain.as_str() {
        "onyx" => ExecuteMsg::SendMessageEvm {
            destination_chain: chain,
            destination_address: address,
            message,
            msg_type: MessageType::Message,
        },
        _ => ExecuteMsg::SendMessageCosmos {
            destination_chain: chain,
            destination_address: address,
            message,
            msg_type: MessageType::Message,
        },
    }
}

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

pub fn create_response(
    contract_addr: Addr,
    chain: String,
    message: String,
) -> Result<CosmosMsg, ContractError> {
    let exe_msg = get_exec_msg(chain, contract_addr.to_string(), message);

    Ok(CosmosMsg::Wasm(wasm_execute(
        contract_addr,
        &exe_msg,
        vec![coin(1u128, VSPN)],
    )?))
}
