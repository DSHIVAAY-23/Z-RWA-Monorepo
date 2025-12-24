use super::*;

/// Function to check wheather the user has admin rights or not
pub fn is_admin(deps: &Deps, address: Addr) -> Result<(), ContractError> {
    ADMINS.load(deps.storage).map_or(
        Err(ContractError::NotAdmin {
            address: address.clone(),
        }),
        |sub_admins| {
            if !sub_admins.contains(&address) {
                Err(ContractError::NotAdmin { address })
            } else {
                Ok(())
            }
        },
    )
}

/// Function to check wheather the user has executer rights or not
pub fn is_executer(deps: &Deps, address: Addr) -> Result<(), ContractError> {
    EXECUTER.load(deps.storage).map_or(
        Err(ContractError::NotAnExecuter {
            address: address.clone(),
        }),
        |executer| {
            if executer.ne(&address) {
                Err(ContractError::NotAdmin { address })
            } else {
                Ok(())
            }
        },
    )
}

/// Function to return function call attributes, basically used for event purpose
pub fn contract_call_attributes(
    source_chain: String,
    source_address: String,
    dest_chain: String,
    dest_address: String,
    sender: Addr,
    payload: Bytes,
) -> Result<Vec<Attribute>, ContractError> {
    Ok(vec![
        attr("source_chain", source_chain),
        attr("source_address", source_address),
        attr("destination_chain", dest_chain),
        attr("destination_address", dest_address),
        attr("sender", sender),
        attr("payload", hex::encode(payload)),
    ])
}
