use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    use QueryMsg::*;

    match msg {
        GetStoredMessage {} => get_stored_message(deps),
        GetIBCResponse {} => try_get_ibc_response(deps),
        GetDestConfig {} => try_get_dest_config(deps),
        GetOperators {} => try_get_operators(deps),
    }
}

pub fn get_stored_message(deps: Deps) -> Result<QueryResponse, StdError> {
    let messages = STORED_MESSAGE.may_load(deps.storage).unwrap().unwrap();
    to_binary(&messages)
}

fn try_get_ibc_response(deps: Deps) -> Result<QueryResponse, StdError> {
    let res = IBC_RESPONSE.load(deps.storage)?;

    to_binary(&res)
}

fn try_get_dest_config(deps: Deps) -> Result<QueryResponse, StdError> {
    let conig = DEST_CONFIG.load(deps.storage)?;

    to_binary(&conig)
}

// Query to get operator list.
fn try_get_operators(deps: Deps) -> Result<QueryResponse, StdError> {
    let addresses = OPERATORS.load(deps.storage)?;
    to_binary(&addresses)
}
