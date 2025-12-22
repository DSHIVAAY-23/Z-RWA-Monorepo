use crate::{constants::*, enums::*, errors::*, events::*, instructions::*, states::*, structs::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{transfer_checked, Token2022, TransferChecked},
    token_interface::TokenAccount,
};
use token_program::{
    cpi::accounts::{BurnTokenFrom, MintToken},
    TokenParams,
};

mod constants;
mod enums;
mod errors;
mod events;
mod instructions;
mod states;
mod structs;

declare_id!("H9txrMrTfGU6LXYWBzMHzmzuQWbNYcW1vHFLRcxCYiKn");

#[program]
pub mod fund_contract {
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

    pub fn create_fund(ctx: Context<Create>, params: CreateParams) -> Result<()> {
        instructions::create(ctx, params)
    }

    pub fn share_dividends(
        ctx: Context<ShareDividends>,
        params: ShareDividendsParams,
    ) -> Result<()> {
        instructions::share_dividend(ctx, params)
    }

    pub fn waterfall_distribution(
        ctx: Context<DistributeAndBurn>,
        params: DistributionParams,
    ) -> Result<()> {
        instructions::distribute_and_burn(ctx, params)
    }

    pub fn update_stable_coins(ctx: Context<StableCoins>, params: StableCoinParams) -> Result<()> {
        instructions::update_stable_coin(ctx, params)
    }
}
