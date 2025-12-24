use super::*;

#[cw_serde]
pub enum ExecuteMsg {
    /// Transfer tokens from sender address to another address
    Transfer {
        amount: Uint128,
        to: Addr,
    },

    /// Transfer tokens from some account to another account
    TransferFrom {
        amount: Uint128,
        from: Addr,
        to: Addr,
    },

    Freeze {
        update_type: UpdateType<Vec<Addr>>,
    },

    PartialFreeze {
        params: Vec<PartialFreezeParams>,
    },

    ManageRoles {
        roles: Vec<Role>,
    },

    /// Approve some address to move some tokens on sender's behalf
    Approve {
        spender: Addr,
        amount: Uint128,
    },

    Request {
        request_id: String,
        amount: Uint128,
        request_type: RequestType,
    },

    RequestFrom {
        request_id: String,
        from: Addr,
        amount: Uint128,
        request_type: RequestType,
    },

    ApproveRequest {
        request_id: String,
        request_type: RequestType,
    },

    RejectRequest {
        request_id: String,
        request_type: RequestType,
    },

    ManageRequestAllowance {
        spender: Addr,
        update_type: UpdateType<Uint128>,
        request_type: RequestType,
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

    RescueCoins {
        denom: String,
        to_address: String,
        amount: u128,
    },

    UpdateDestConfig {
        config: DestConfig,
    },
    
    ClearBurnBalance {
        address: Addr,
    },
}
