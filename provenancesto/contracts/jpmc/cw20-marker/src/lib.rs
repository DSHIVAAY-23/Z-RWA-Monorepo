#![warn(clippy::all)]
pub mod contract;
pub mod enums;
pub mod msg;
pub mod structs;

mod error;
mod helper;
mod ibc;
mod state;
mod types;

// #[cfg(test)]
// mod tests;

use crate::{
    enums::*, error::ContractError, helper::*, ibc::*, msg::*, state::*, structs::*, types::*,
};
use bincode::{deserialize, serialize};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    attr, coin, entry_point, to_binary, wasm_execute, Addr, BankMsg, Binary, CosmosMsg, Deps,
    DepsMut, Empty, Env, MessageInfo, QueryResponse, Response, StdError, StdResult, Storage,
    Uint128,
};
use cw_storage_plus::{Item, Map};
use ethabi::{decode, encode, ParamType, Token};
use provwasm_proc_macro::CosmwasmExt;
use provwasm_std::types::{
    cosmos::base::v1beta1::Coin,
    provenance::marker::v1::{
        Access, AccessGrant, MarkerAccount, MarkerQuerier, MarkerStatus, MarkerType,
        MsgActivateRequest, MsgAddMarkerRequest, MsgBurnRequest, MsgMintRequest,
        MsgTransferRequest, MsgWithdrawRequest,
    },
};
use schemars::JsonSchema;
use serde::{
    de::DeserializeOwned,
    {Deserialize, Serialize},
};
use serde_json_wasm::to_string;
use std::convert::TryFrom;
use std::fmt::Display;
use thiserror::Error;
