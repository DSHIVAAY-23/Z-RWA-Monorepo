#![warn(clippy::all)]
pub mod contract;
pub mod enums;
pub mod error;
pub mod helper;
pub mod ibc;
pub mod msg;
pub mod state;
pub mod structs;
pub mod types;

// #[cfg(test)]
// mod tests;

use crate::{enums::*, error::ContractError, helper::*, msg::*, state::*, structs::*};
use bincode::{deserialize, serialize};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    attr, ensure, to_json_binary, wasm_execute, Addr, Binary, CosmosMsg, Env, MessageInfo,
    QueryResponse, Response, StdError, StdResult, Storage, Uint128, Uint64,
};
use cw_storage_plus::{Item, Map};
use ethabi::{decode, encode, ParamType, Token};
use osmosis_std_derive::CosmwasmExt;
use provwasm_std::{
    AccessGrant, Marker, MarkerAccess, MarkerMsgParams, MarkerType, ProvenanceMsg,
    ProvenanceMsgParams, ProvenanceQuery,
};

// Type aliases for the contract's dependencies
pub type Deps<'a> = cosmwasm_std::Deps<'a, ProvenanceQuery>;
pub type DepsMut<'a> = cosmwasm_std::DepsMut<'a, ProvenanceQuery>;
use schemars::JsonSchema;
use serde::{
    de::DeserializeOwned,
    {Deserialize, Serialize},
};
use serde_json_wasm::to_string;
use std::{
    convert::TryFrom,
    fmt,
    fmt::{Display, Formatter},
};
use thiserror::Error;
