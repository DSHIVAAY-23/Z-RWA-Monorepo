use super::*;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Addr)]
    GetContractByDenom { denom: String },

    #[returns(String)]
    GetDenomByContract { addr: Addr },

    /// Decimals are 0 for Marker Tokens and cannot be updated
    #[returns(u8)]
    GetDecimals { denom: String },

    /// Total supply of tokens minted in this contract
    #[returns(Coin)]
    GetTotalSupply { denom: String },

    /// Query for the token balance of some address
    #[returns(Coin)]
    GetBalanceOf { denom: String, address: Addr },

    /// Remaining tokens that can be moved on sender's behalf by another address
    #[returns(Uint128)]
    GetAllowance {
        denom: String,
        owner: Addr,
        spender: Addr,
    },

    #[returns(Option<Vec<Addr>>)]
    GetFreezedAccounts { denom: String },

    #[returns(Option<Uint128>)]
    GetFrozenBalance { denom: String, address: Addr },

    #[returns(Vec<Addr>)]
    GetSubAdmins {},

    #[returns(Addr)]
    GetAdmin {},

    #[returns(Uint128)]
    GetFrozenTokens { denom: String },

    #[returns(Request)]
    GetRequestOf { denom: String, request_id: String },

    #[returns(Uint128)]
    GetRequestAllowances {
        denom: String,
        owner: Addr,
        spender: Addr,
        request_type: RequestType,
    },

    #[returns(Uint128)]
    GetBurnBalanceOf { denom: String, owner: Addr },

    // Get code id of the token contract
    #[returns(u64)]
    GetCodeId {},
}
