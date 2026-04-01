use crate::{constants::*, enums::*, errors::*, events::*, instructions::*, states::*, structs::*};
use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use base_token_program::{cpi::accounts::RequestOrderAccounts, RequestOrder, RequestType};
use ethabi::{decode, encode, ethereum_types::Address, ParamType, Token, Uint};
use std::{
    fmt,
    fmt::{Display, Formatter},
    str::FromStr,
};

mod constants;
mod enums;
mod errors;
mod events;
mod instructions;
mod states;
mod structs;

declare_id!("BUReiDoMaaACpK2U8VQu6o2ziKcFhomRSgq5CdrvqQpC");

#[program]
pub mod interop_core {
    use super::*;

    pub fn init(ctx: Context<Initialize>, params: InitParams) -> Result<()> {
        instructions::initialize(ctx, params)
    }

    pub fn add_admin_accounts(ctx: Context<UpdateAdmins>, addresses: Vec<Pubkey>) -> Result<()> {
        instructions::add_admins(ctx, addresses)
    }

    pub fn remove_admin_accounts(ctx: Context<UpdateAdmins>, addresses: Vec<Pubkey>) -> Result<()> {
        instructions::remove_admins(ctx, addresses)
    }

    pub fn manage_roles(ctx: Context<ManageRoles>, role: Role) -> Result<()> {
        instructions::manage_role(ctx, role)
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, params: Order) -> Result<()> {
        instructions::mint_token(ctx, params)
    }

    pub fn burn_tokens(ctx: Context<BurnTokens>, params: Order) -> Result<()> {
        instructions::burn_token(ctx, params)
    }

    pub fn send_instructions(ctx: Context<SendInstructions>, params: SendParams) -> Result<()> {
        instructions::send_instruction(ctx, params)
    }

    pub fn execute_instructions(
        ctx: Context<ExecuteInstructions>,
        source_chain: String,
        source_address: String,
        payload: String,
    ) -> Result<()> {
        instructions::execute_instruction(ctx, source_chain, source_address, payload)
    }
}
