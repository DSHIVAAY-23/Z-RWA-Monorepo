use super::*;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Name of the token represented by this contract
    /// Name will most likely be the asset "long name" or equivalent
    #[returns(String)]
    GetName {},

    /// In Provenance Symbol is not supported
    #[returns(String)]
    GetSymbol {},

    /// Decimals are 0 for Marker Tokens and cannot be updated
    #[returns(u8)]
    GetDecimals {},

    /// Total supply of tokens minted in this contract
    #[returns(Coin)]
    GetTotalSupply {},

    /// Query for the token balance of some address
    #[returns(cosmwasm_std::Coin)]
    GetBalanceOf { address: Addr },

    /// Remaining tokens that can be moved on sender's behalf by another address
    #[returns(Uint128)]
    GetAllowance { owner: Addr, spender: Addr },

    #[returns(Marker)]
    GetByAddress { address: String },

    #[returns(Marker)]
    GetByDenom {},

    #[returns(Option<Vec<Addr>>)]
    GetFreezedAccounts {},

    #[returns(Option<Uint128>)]
    GetFrozenBalance { address: Addr },

    #[returns(Vec<Addr>)]
    GetSubAdmins {},

    #[returns(Uint128)]
    GetFrozenTokens {},

    #[returns(Request)]
    GetRequestOf { request_id: String },

    #[returns(Uint128)]
    GetRequestAllowances {
        owner: Addr,
        spender: Addr,
        request_type: RequestType,
    },

    #[returns(Uint128)]
    GetBurnBalanceOf { owner: Addr },

    #[returns(Addr)]
    GetTokenizationAgent {},

    #[returns(DestConfig)]
    GetDestConfig {},
}
