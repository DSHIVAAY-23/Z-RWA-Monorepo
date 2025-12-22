use super::*;

/// Asset Type, use to choose between different assets
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
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
