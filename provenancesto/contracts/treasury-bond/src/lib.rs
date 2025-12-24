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
use bincode::{deserialize, serialize};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    attr, coin, entry_point, to_json_binary, Addr, BankMsg, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, QueryResponse, Response, StdError, Storage, Uint128,
};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{
    de::DeserializeOwned,
    {Deserialize, Serialize},
};
use std::fmt::Display;
use thiserror::Error;
