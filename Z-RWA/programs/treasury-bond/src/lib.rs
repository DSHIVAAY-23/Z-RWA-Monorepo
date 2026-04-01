use crate::{constants::*, enums::*, errors::*, events::*, instructions::*, states::*, structs::*};
use anchor_lang::prelude::*;
use anchor_spl::token_2022::{transfer_checked, Token2022, TransferChecked};

mod constants;
mod enums;
mod errors;
mod events;
mod instructions;
mod states;
mod structs;

declare_id!("ZYFZi9Y9HEUXkJ1Pi2hpvsfM1Dwes96rhfg6hy3t2nA");

#[program]
pub mod treasury_bond {
    use super::*;

    pub fn init(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn add_admin_accounts(ctx: Context<ManageAdmins>, addresses: Vec<Pubkey>) -> Result<()> {
        instructions::add_admins(ctx, addresses)
    }

    pub fn remove_admin_accounts(ctx: Context<ManageAdmins>, addresses: Vec<Pubkey>) -> Result<()> {
        instructions::remove_admins(ctx, addresses)
    }

    pub fn update_agent_by_token(
        ctx: Context<UpdateAgent>,
        token: String,
        address: Pubkey,
    ) -> Result<()> {
        instructions::update_agent(ctx, token, address)
    }

    pub fn create_bond(ctx: Context<Create>, params: CreateParams) -> Result<()> {
        instructions::create(ctx, params)
    }

    pub fn share_stable_coins(
        ctx: Context<ShareStableCoin>,
        params: ShareStableCoinParams,
    ) -> Result<()> {
        instructions::share_stable_coin(ctx, params)
    }

    pub fn update_stable_coins(ctx: Context<StableCoins>, params: StableCoinParams) -> Result<()> {
        instructions::update_stable_coin(ctx, params)
    }

    pub fn update_credit_rating(
        ctx: Context<UpdateCreditRate>,
        token: String,
        rating: String,
    ) -> Result<()> {
        instructions::update_cr_rate(ctx, token, rating)
    }
}
