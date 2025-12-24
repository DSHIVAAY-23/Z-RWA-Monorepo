use super::*;

/// Function to check wheather the user has admin rights or not
pub fn is_admin(deps: &DepsMut, address: Addr) -> Result<(), ContractError> {
    ADMINS.load(deps.storage).map_or(
        Err(ContractError::NotAdmin {
            address: address.clone(),
        }),
        |sub_admins| {
            if !sub_admins.contains(&address) {
                Err(ContractError::NotAdmin { address })
            } else {
                Ok(())
            }
        },
    )
}
