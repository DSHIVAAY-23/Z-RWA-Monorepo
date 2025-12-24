use super::*;

//custom access controls
#[cw_serde]
#[derive(Eq)]
pub enum AccessControls {
    Admin,
    Burn,
    Deposit,
    Delete,
    Mint,
    Transfer,
    Unspecified,
    Withdraw,
    Freeze,
    Unfreeze,
    ForceTransfer,
}

impl Display for AccessControls {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admin => write!(f, "admin"),
            Self::Burn => write!(f, "burn"),
            Self::Deposit => write!(f, "deposit"),
            Self::Delete => write!(f, "delete"),
            Self::Mint => write!(f, "mint"),
            Self::Transfer => write!(f, "transfer"),
            Self::Unspecified => write!(f, "unspecified"),
            Self::Withdraw => write!(f, "withdraw"),
            Self::Freeze => write!(f, "freeze"),
            Self::Unfreeze => write!(f, "unfreeze"),
            Self::ForceTransfer => write!(f, "force_transfer"),
        }
    }
}

impl AccessControls {
    pub fn issuer_rights() -> Vec<AccessControls> {
        vec![
            AccessControls::Mint,
            AccessControls::Burn,
            AccessControls::Freeze,
            AccessControls::Unfreeze,
            AccessControls::ForceTransfer,
        ]
    }

    pub fn transfer_agent_rights() -> Vec<AccessControls> {
        vec![
            AccessControls::Freeze,
            AccessControls::Unfreeze,
            AccessControls::ForceTransfer,
        ]
    }

    pub fn tokenization_agent_rights() -> Vec<AccessControls> {
        vec![AccessControls::Burn, AccessControls::Mint]
    }
}

#[cw_serde]
pub enum UpdateType<T> {
    Add(T),
    Remove(T),
}

#[cw_serde]
pub enum UpdateKind<T> {
    Set(T),
    Unset {},
}

#[cw_serde]
pub enum Role {
    Issuer {
        update_type: UpdateType<Addr>,
    },
    TransferAgent {
        update_type: UpdateType<Addr>,
    },
    TokenizationAgent {
        update_type: UpdateType<Addr>,
    },
    SubAdmin {
        update_type: UpdateType<Vec<Addr>>,
    },
    Admin {
        address: Addr,
    },
    Operators {
        update_type: UpdateType<Vec<String>>,
    },
}

#[cw_serde]
pub enum MessageType {
    Message,
    MessageWithToken,
    Token,
}

impl MessageType {
    pub fn into_i64(&self) -> i64 {
        use MessageType::*;

        match self {
            Message => 1,
            MessageWithToken => 2,
            Token => 3,
        }
    }
}

#[cw_serde]
pub enum RequestType {
    Mint,
    Burn,
}

impl Display for RequestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mint => write!(f, "Mint"),
            Self::Burn => write!(f, "Burn"),
        }
    }
}

impl Default for RequestType {
    fn default() -> Self {
        Self::Mint
    }
}
