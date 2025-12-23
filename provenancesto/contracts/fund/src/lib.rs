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

const MARKER_CONTRACT_ADDRESS: &str =
    "tp1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqf06p2p";

const ORACLE_CONTRACT_ADDRESS: &str =
    "tp15d2kxfntk3u8wtr42nsrgrtqf6jxf8lsn9qpj69nzkxh8ykhwfsq863kuz";
