use super::*;

#[account]
pub struct Validators {
    pub addresses: Vec<Pubkey>,
}

impl Validators {
    pub fn add_validators(&mut self, addresses: Vec<Pubkey>) {
        self.addresses.extend(addresses);
        self.addresses.sort();
        self.addresses.dedup();
    }

    pub fn remove_validators(&mut self, addresses: Vec<Pubkey>) {
        self.addresses.retain(|addr| !addresses.contains(addr));
    }
}
