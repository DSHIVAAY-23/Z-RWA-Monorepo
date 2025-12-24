use super::*;

//custom access controls
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

// Request Type
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RequestType {
    Burn,
    Mint,
}

impl Display for RequestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Burn => write!(f, "burn"),
            Self::Mint => write!(f, "mint"),
        }
    }
}

// Cancel Type
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CancelType {
    Request,
    Approval,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UpdateKind<T> {
    Set(T),
    Unset {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UpdateCoin {
    Add(Cw20Coin),
    Sub(Cw20Coin),
    Discard(Cw20Coin),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UpdateType<T> {
    Add(T),
    Remove(T),
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
    Agent {
        update_type: UpdateType<Vec<Addr>>,
        marker_access: Vec<AccessControls>,
    },
}
