use super::*;

mod execute;
mod init;
mod migrate;
mod query;

pub use self::{execute::*, init::*, migrate::*, query::*};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        _ => Err(ContractError::InvalidReplyId),
    }
}

fn handle_instantiate_reply(deps: DepsMut, reply: Reply) -> Result<Response, ContractError> {
    // parse contract info from data
    let raw_addr = parse_reply_instantiate_data(reply)?.contract_address;
    let contract_addr = deps.api.addr_validate(&raw_addr)?;
    let denom = DENOM.load(deps.storage)?;
    DENOM.remove(deps.storage);

    if DENOM_TO_CONTRACT.has(deps.storage, denom.to_string())
        || CONTRACT_TO_DENOM.has(deps.storage, contract_addr.clone())
    {
        return Err(ContractError::AlreadyExist {
            denom,
            addr: contract_addr,
        });
    }

    DENOM_TO_CONTRACT.save(deps.storage, denom.to_string(), &contract_addr)?;
    CONTRACT_TO_DENOM.save(deps.storage, contract_addr.clone(), &denom)?;

    if CONTRACTS
        .update(deps.storage, |mut contracts| -> Result<_, ContractError> {
            Ok({
                contracts.push(contract_addr.clone().into_string());
                contracts
            })
        })
        .is_err()
    {
        CONTRACTS.save(deps.storage, &vec![contract_addr.clone().into_string()])?;
    };

    Ok(Response::new().add_attribute("cw20_marker_contract_address", contract_addr))
}
