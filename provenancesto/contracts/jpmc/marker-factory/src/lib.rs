#![warn(clippy::all)]
pub mod contract;
pub mod msg;

mod enums;
mod error;
mod helper;
mod state;

// #[cfg(test)]
// mod tests;

use crate::{enums::*, error::ContractError, helper::*, msg::*, state::*};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    attr, entry_point, to_binary, wasm_execute, Addr, CosmosMsg, Deps, DepsMut, Env,
    Instantiate2AddressError, MessageInfo, QueryResponse, Reply, Response, StdError, Storage,
    SubMsg, Uint128, WasmMsg,
};
use cw20_marker::{
    enums::{RequestType, UpdateType},
    structs::{PartialFreezeParams, Request},
};
use cw_storage_plus::{Item, Map};
use cw_utils::{parse_reply_instantiate_data, ParseReplyError};
use provwasm_std::types::cosmos::base::v1beta1::Coin;
use thiserror::Error;
