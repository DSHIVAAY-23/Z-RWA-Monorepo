#[cfg(not(feature = "library"))]
use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    use QueryMsg::*;

    match msg {
        GetStoredMessage {} => get_stored_message(deps),
        GetIBCResponse {} => try_get_ibc_response(deps),
    }
}

pub fn get_stored_message(deps: Deps) -> Result<QueryResponse, StdError> {
    let message = STORED_MESSAGE.may_load(deps.storage).unwrap().unwrap();
    let resp = GetStoredMessageResp {
        sender: message.sender,
        message: message.message,
    };
    to_binary(&resp)
}

fn try_get_ibc_response(deps: Deps) -> Result<QueryResponse, StdError> {
    let res = IBC_RESPONSE.load(deps.storage)?;

    to_binary(&res)
}
