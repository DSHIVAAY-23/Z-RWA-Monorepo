use crate::{constants::*, errors::*, events::*, instructions::*, states::*, structs::*};
use anchor_lang::{
    prelude::*,
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, rent::Rent,
        sysvar::Sysvar,
    },
    Lamports,
};
use anchor_spl::{
    token_2022::{
        self, set_authority, Burn, FreezeAccount, MintTo, SetAuthority, ThawAccount, Token2022,
        TransferChecked,
    },
    token_interface::{
        token_metadata_initialize, Mint, TokenAccount, TokenInterface, TokenMetadataInitialize,
    },
};
pub use enums::*;
use std::{
    fmt,
    fmt::{Display, Formatter},
};
pub use structs::RequestOrder;

mod constants;
mod enums;
mod errors;
mod events;
mod instructions;
mod states;
mod structs;

declare_id!("7iaDbVGbVJdhZcKXWQmMw783nBqfDyx6K8V4yF6Kv8iq");

#[program]
pub mod base_token_program {
    use super::*;

    pub fn init(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn manage_admin(ctx: Context<UpdateAdmin>, address: Pubkey) -> Result<()> {
        instructions::update_admin(ctx, address)
    }

    pub fn add_sub_admin_accounts(
        ctx: Context<UpdateSubAdmins>,
        addresses: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::add_sub_admins(ctx, addresses)
    }

    

    pub fn remove_sub_admin_accounts(
        ctx: Context<UpdateSubAdmins>,
        addresses: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::remove_sub_admins(ctx, addresses)
    }

    pub fn create(ctx: Context<CreateToken>, params: CreateTokenParams) -> Result<()> {
        instructions::create_token(ctx, params)
    }

    pub fn mint_token(ctx: Context<MintToken>, params: TokenParams) -> Result<()> {
        instructions::mint(ctx, params)
    }

    pub fn burn_token(ctx: Context<BurnToken>, params: TokenParams) -> Result<()> {
        instructions::burn(ctx, params)
    }

    pub fn burn_token_from(ctx: Context<BurnTokenFrom>, params: TokenParams) -> Result<()> {
        instructions::burn_from(ctx, params)
    }

    pub fn freeze_user_account(ctx: Context<FreezeUserAccount>, token: String) -> Result<()> {
        instructions::freeze(ctx, token)
    }

    pub fn unfreeze_user_account(ctx: Context<UnfreezeUserAccount>, token: String) -> Result<()> {
        instructions::unfreeze(ctx, token)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, params: TransferParams) -> Result<()> {
        instructions::transfer(ctx, params)
    }

    pub fn force_transfer_tokens(
        ctx: Context<ForceTransferTokens>,
        params: ForceTransferParams,
    ) -> Result<()> {
        instructions::force_transfer(ctx, params)
    }

    pub fn partial_freeze_account(
        ctx: Context<PartialFreezeAccount>,
        token: String,
        amount: u64,
    ) -> Result<()> {
        instructions::partial_freeze(ctx, token, amount)
    }

    pub fn partial_unfreeze_account(
        ctx: Context<PartialUnfreezeAccount>,
        token: String,
        amount: u64,
    ) -> Result<()> {
        instructions::partial_unfreeze(ctx, token, amount)
    }

    pub fn update_issuer_by_token(
        ctx: Context<UpdateTokenConfig>,
        token: String,
        address: Pubkey,
    ) -> Result<()> {
        instructions::update_issuer(ctx, token, address)
    }

    pub fn update_tokenization_agent_by_token(
        ctx: Context<UpdateTokenConfig>,
        token: String,
        address: Pubkey,
    ) -> Result<()> {
        instructions::update_tokenization_agent(ctx, token, address)
    }

    pub fn update_transfer_agent_by_token(
        ctx: Context<UpdateTokenConfig>,
        token: String,
        address: Pubkey,
    ) -> Result<()> {
        instructions::update_transfer_agent(ctx, token, address)
    }

    pub fn request_orders(ctx: Context<RequestOrderAccounts>, params: RequestOrder) -> Result<()> {
        instructions::request_order(ctx, params)
    }
}
