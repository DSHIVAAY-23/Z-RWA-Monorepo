use super::*;

#[cw_serde]
pub struct InitMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Create {
        params: CreateMarkerParams,
    },
    GrantAccess {
        denom: String,
        address: Addr,
    },
    ForceTransfer {
        denom: String,
        params: Vec<ForceTransferParams>,
    },
    Freeze {
        denom: String,
        update_type: UpdateType<Vec<Addr>>,
    },
    PartialFreeze {
        denom: String,
        params: Vec<PartialFreezeParams>,
    },
    Send {
        amount: Uint128,
        denom: String,
        to: Addr,
    },
    MintTo {
        mint_to_params: Vec<MintBurnParams>,
    },
    BurnFrom {
        burn_from_params: Vec<MintBurnParams>,
    },
    ManageRoles {
        denom: String,
        roles: Vec<Role>,
    },
    SendMessageEvm {
        destination_chain: String,
        destination_address: String,
        message: String,
        msg_type: MessageType,
    },
    SendMessageCosmos {
        destination_chain: String,
        destination_address: String,
        message: String,
        msg_type: MessageType,
    },
    ReceiveMessageCosmos {
        sender: String,
        message: String,
    },
    ReceiveMessageEvm {
        source_chain: String,
        source_address: String,
        payload: Binary,
    },
    RequestOrder {
        order_id: String,
        denom: String,
        from: Addr,
        amount: Uint128,
        request_type: RequestType,
    },
    UpdateDestConfig {
        config: DestConfig,
    },
    SetIbcResponse {
        is_required: bool,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Marker)]
    GetByAddress { address: String },

    #[returns(Marker)]
    GetByDenom { denom: String },

    #[returns(Option<Vec<Addr>>)]
    GetFreezedAccounts { denom: String },

    #[returns(Option<Uint128>)]
    GetFrozenBalance { denom: String, address: Addr },

    #[returns(Vec<Addr>)]
    GetSubAdmins {},

    #[returns(Addr)]
    GetAdmin {},

    #[returns(cosmwasm_std::Coin)]
    GetBalance { denom: String, address: Addr },

    #[returns(Uint128)]
    GetFrozenTokens { denom: String },

    #[returns(Uint128)]
    GetCiculatingSupply { denom: String },

    #[returns(Vec<IBCResponse>)]
    #[serde(rename = "get_ibc_response")]
    GetIBCResponse {},

    #[returns(Vec<String>)]
    GetOperators {},

    #[returns(Request)]
    GetRequestOf { order_id: String },

    #[returns(Uint128)]
    GetRequestBalanceOf {
        owner: Addr,
        request_type: RequestType,
    },

    #[returns(DestConfig)]
    GetDestConfig {},

    #[returns(bool)]
    IsIbcResponseRequired {},

    #[returns(Uint64)]
    GetHoldPeriod { denom: String },
}

/// Migrate the contract.
#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum IBCLifecycleComplete {
    #[serde(rename = "ibc_ack")]
    IBCAck {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
        /// String encoded version of the ack as seen by OnAcknowledgementPacket(..)
        ack: String,
        /// Whether an ack is a success of failure according to the transfer spec
        success: bool,
    },
    #[serde(rename = "ibc_timeout")]
    IBCTimeout {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
    },
}

/// Message type for `sudo` entry_point
#[cw_serde]
pub enum SudoMsg {
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
}
