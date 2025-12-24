use super::*;

#[cw_serde]
pub struct Votes {
    pub yes: u8,
    pub no: u8,
    pub voters: Vec<Addr>,
    pub status: Status,
}

impl Votes {
    pub fn new(can_transact: bool, voters: Vec<Addr>) -> Self {
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
}
