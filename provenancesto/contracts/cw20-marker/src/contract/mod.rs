use super::*;

mod execute;
mod init;
mod migrate;
mod query;

pub use self::{execute::*, init::*, migrate::*, query::*};

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn sudo(
    deps: DepsMut,
    env: Env,
    msg: SudoMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let own_addr = env.contract.address.to_string();
    match msg {
        SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCAck {
            channel,
            sequence,
            ack,
            success,
        }) => ibc::receive_ack(deps, own_addr, channel, sequence, ack, success),
        SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout { channel, sequence }) => {
            ibc::receive_timeout(deps, own_addr, channel, sequence)
        }
    }
}
