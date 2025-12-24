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
    attr, entry_point, to_binary, wasm_execute, wasm_instantiate, Addr, Binary, CosmosMsg, Deps,
    DepsMut, Env, MessageInfo, Order, QueryResponse, Response, StdError, StdResult, Storage,
    Uint128,
};
use cw20::{Cw20Coin, MinterResponse};
use cw_storage_plus::{Item, Map};
use provwasm_std::{
    activate_marker, burn_marker_supply, cancel_marker, create_forced_transfer_marker,
    destroy_marker, finalize_marker, grant_marker_access, mint_marker_supply,
    transfer_marker_coins, withdraw_coins, MarkerAccess, ProvenanceMsg, ProvenanceQuerier,
    ProvenanceQuery,
};
use schemars::JsonSchema;
use serde::{
    de::DeserializeOwned,
    {Deserialize, Serialize},
};
use std::{
    fmt,
    fmt::{Display, Formatter},
};
use thiserror::Error;
