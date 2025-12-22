use super::*;

#[event]
pub struct InitEvent {
    pub admin: Pubkey,
    pub executer: Pubkey,
    pub deloyed_chain: String,
}

impl InitEvent {
    pub fn new(admin: Pubkey, params: InitParams) -> Self {
        Self {
            admin,
            executer: params.multisig,
            deloyed_chain: params.deployed_chain,
        }
    }
}

#[event]
pub struct AddAdminsEvent {
    pub addresses: Vec<Pubkey>,
}

#[event]
pub struct RemoveAdminsEvent {
    pub addresses: Vec<Pubkey>,
}

#[event]
pub struct UpdateExecuterEvent {
    pub address: Pubkey,
}

#[event]
pub struct UpdateSourceChainEvent {
    pub chain: String,
}

#[event]
pub struct MintEvent {
    pub token: String,
    pub amount: u64,
}

#[event]
pub struct BurnEvent {
    pub token: String,
    pub amount: u64,
}

#[event]
pub struct SendInstructionEvent {
    pub action: Action,
    pub source_chain: String,
    pub source_address: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub sender: Pubkey,
    pub payload: String,
}

#[event]
pub struct ExecuteInstructionEvent {
    pub action: Action,
    pub source_chain: String,
    pub source_address: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub sender: Pubkey,
    pub payload: String,
}
