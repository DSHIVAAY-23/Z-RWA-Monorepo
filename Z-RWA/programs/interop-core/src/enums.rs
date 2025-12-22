use super::*;

/// Update Type
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum UpdateType {
    Add { addresses: Vec<Pubkey> },
    Remove { addresses: Vec<Pubkey> },
}

/// Roles
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Role {
    Executer { addr: Pubkey },
}

// Action
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
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
    type Err = Error;

    fn from_str(val: &str) -> Result<Self> {
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

    fn to_provenance_u8(self) -> u8 {
        match self {
            Self::Mint => 1,
            Self::Burn => 2,
            Self::Ack => 3,
        }
    }

    fn to_other_u8(self) -> u8 {
        match self {
            Self::Mint => 4,
            Self::Burn => 5,
            Self::Ack => 3,
        }
    }

    pub fn to_uint(self) -> Uint {
        Uint::from(self.to_provenance_u8())
    }

    pub fn to_other_uint(self) -> Uint {
        Uint::from(self.to_other_u8())
    }

    pub fn to_request_type(self) -> Result<RequestType> {
        match self {
            Self::Burn => Ok(RequestType::Burn),
            Self::Mint => Ok(RequestType::Mint),
            Self::Ack => Err(CustomError::InvalidRequest.into()),
        }
    }
}
