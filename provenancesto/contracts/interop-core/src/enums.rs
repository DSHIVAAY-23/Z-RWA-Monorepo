use super::*;

// Action
#[cw_serde]
pub enum Action {
    Burn,
    Mint,
    Ack,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mint => write!(f, "1"),
            Self::Burn => write!(f, "2"),
            Self::Ack => write!(f, "3"),
        }
    }
}

impl FromStr for Action {
    type Err = ContractError;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val {
            "1" => Ok(Self::Mint),
            "2" => Ok(Self::Burn),
            _ => Ok(Self::Ack),
        }
    }
}

impl Action {
    pub fn from_u32(val: u32) -> Self {
        match val {
            1 => Self::Mint,
            2 => Self::Burn,
            _ => Self::Ack,
        }
    }

    fn to_provenance_u8(&self) -> u8 {
        match self {
            Self::Mint => 1,
            Self::Burn => 2,
            Self::Ack => 3,
        }
    }

    fn to_other_u8(&self) -> u8 {
        match self {
            Self::Mint => 4,
            Self::Burn => 5,
            Self::Ack => 3,
        }
    }

    pub fn to_provenenace_uint(&self) -> Uint {
        Uint::from(self.to_provenance_u8())
    }

    pub fn to_other_uint(&self) -> Uint {
        Uint::from(self.to_other_u8())
    }

    pub fn to_event(&self) -> &str {
        match self {
            Action::Burn => "provwasm.contracts.interop_core.send_burn_instruction",
            Action::Mint => "provwasm.contracts.interop_core.send_mint_instruction",
            Action::Ack => "provwasm.contracts.interop_core.send_acknowledgement",
        }
    }
}

#[cw_serde]
pub enum UpdateType<T> {
    Add(T),
    Remove(T),
}

// Roles
#[cw_serde]
pub enum Role {
    Admins { update_type: UpdateType<Vec<Addr>> },
    Executer { addr: Addr },
}

// Error type
#[cw_serde]
pub enum ErrorType {
    Uint,
    Address,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uint => write!(f, "Uint"),
            Self::Address => write!(f, "Address"),
        }
    }
}
