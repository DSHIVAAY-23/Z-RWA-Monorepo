use super::*;
use UpdateType;

#[cw_serde]
pub enum ExecuteMsg {
    DeployToken {
        msg: cw20_marker::msg::InitMsg,
    },

    TransferFrom {
        denom: String,
        amount: Uint128,
        from: Addr,
        to: Addr,
    },

    Freeze {
        denom: String,
        update_type: UpdateType<Vec<Addr>>,
    },

    PartialFreeze {
        denom: String,
        params: Vec<PartialFreezeParams>,
    },

    ManageRoles {
        roles: Vec<Role>,
    },

    /// Approve some address to move some tokens on sender's behalf
    Approve {
        denom: String,
        spender: Addr,
        amount: Uint128,
    },

    Request {
        denom: String,
        request_id: String,
        amount: Uint128,
        request_type: RequestType,
    },

    RequestFrom {
        denom: String,
        request_id: String,
        from: Addr,
        amount: Uint128,
        request_type: RequestType,
    },

    ApproveRequest {
        denom: String,
        request_id: String,
        request_type: RequestType,
    },

    RejectRequest {
        denom: String,
        request_id: String,
        request_type: RequestType,
    },

    ManageRequestAllowance {
        denom: String,
        spender: Addr,
        update_type: UpdateType<Uint128>,
        request_type: RequestType,
    },

    UpdateCode {
        code_id: u64,
    },

    UpgradeContract {
        contract_address: String,
    },

    RescueCoins {
        denom: String,
        target_denom: String,
        to_address: String,
        amount: u128,
    },
}
