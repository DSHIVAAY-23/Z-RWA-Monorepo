use super::*;

#[account]
pub struct Maintainers {
    /// Admins
    pub admins: Vec<Pubkey>,
}

impl Maintainers {
    pub fn save(&mut self, admin: Pubkey) {
        self.admins = vec![admin];
    }

    pub fn add_admins(&mut self, admins: Vec<Pubkey>) {
        self.admins.extend(admins);
        self.admins.sort();
        self.admins.dedup();
    }

    pub fn remove_admins(&mut self, admins: Vec<Pubkey>) {
        self.admins.retain(|addr| !admins.contains(addr));
    }
}

#[account]
pub struct Threshold {
    pub value: u8,
}
