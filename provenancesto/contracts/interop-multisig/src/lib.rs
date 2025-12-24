#![warn(clippy::all)]
pub mod contract;
pub mod enums;
pub mod error;
pub mod helper;
pub mod msg;
pub mod state;
pub mod structs;

// #[cfg(test)]
// mod tests;

use crate::{enums::*, error::ContractError, helper::*, msg::*, state::*, structs::*};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    attr, ensure, entry_point, to_json_binary, wasm_execute, Addr, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, QueryResponse, Response, StdError,
};
use cw_storage_plus::{Item, Map};
use thiserror::Error;
