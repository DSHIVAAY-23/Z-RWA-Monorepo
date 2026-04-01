use crate::{constants::*, enums::*, errors::*, events::*, instructions::*, states::*, structs::*};
use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use ethabi::{decode, ParamType};
use interop_core::cpi::accounts::ExecuteInstructions;
use std::str::FromStr;

mod constants;
mod enums;
mod errors;
mod events;
mod instructions;
mod states;
mod structs;

declare_id!("DTEbjSxupcMZTrojMe1FZTrqMZPM621oUstwA7d4PrT3");

#[program]
pub mod interop_multisig {
    use super::*;

    pub fn init(ctx: Context<Initialize>, threshold: u8) -> Result<()> {
        instructions::initialize(ctx, threshold)
    }

    pub fn add_admin_accounts(ctx: Context<UpdateAdmins>, addresses: Vec<Pubkey>) -> Result<()> {
        instructions::add_admins(ctx, addresses)
    }

    pub fn remove_admin_accounts(ctx: Context<UpdateAdmins>, addresses: Vec<Pubkey>) -> Result<()> {
        instructions::remove_admins(ctx, addresses)
    }

    pub fn add_validator_accounts(
        ctx: Context<ManageValidators>,
        addresses: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::add_validators(ctx, addresses)
    }

    pub fn remove_validator_accounts(
        ctx: Context<ManageValidators>,
        addresses: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::remove_validators(ctx, addresses)
    }

    pub fn update_threshold(ctx: Context<UpdateThreshold>, threshold: u8) -> Result<()> {
        instructions::update_threshold_value(ctx, threshold)
    }

    pub fn cast_votes(ctx: Context<CastVotes>, tx_hash: String, can_transact: bool) -> Result<()> {
        instructions::cast_vote(ctx, tx_hash, can_transact)
    }

    pub fn execute_transactions(
        ctx: Context<ExecuteTransactions>,
        params: ExecuteTransactionParams,
    ) -> Result<()> {
        instructions::execute_transaction(ctx, params)
    }

    pub fn extract_payload(ctx: Context<ExtractPayload>, payload: String) -> Result<()> {
        instructions::extract_payload_data(ctx, payload)
    }
}
