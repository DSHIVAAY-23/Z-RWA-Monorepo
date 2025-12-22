use super::*;

/// The struct containing instructions for contract initialization
#[account]
pub struct InitParams {
    pub threshold: u8,
}

/// Votes
#[account]
#[derive(Default)]
pub struct Votes {
    pub yes: u8,
    pub no: u8,
    pub voters: Vec<Pubkey>,
    pub status: Status,
}

impl Votes {
    pub fn new(can_transact: bool, voters: Vec<Pubkey>) -> Self {
        let (yes, no) = if can_transact { (1, 0) } else { (0, 1) };
        Self {
            yes,
            no,
            voters,
            status: Status::Pending,
        }
    }

    pub fn update(&mut self, vote: &mut Self) {
        self.yes += vote.yes;
        self.no += vote.no;
        self.voters.append(&mut vote.voters)
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn clear(&mut self) {
        self.yes = u8::default();
        self.no = u8::default();
        self.voters = Vec::default();
        self.status = Status::default();
    }
}

#[account]
pub struct ExecuteTransactionParams {
    pub source_chain: String,
    pub source_address: String,
    pub tx_hash: String,
    pub payload: String,
}
