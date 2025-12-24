use super::*;

/// Function to check wheather the given address is admin or not
pub fn is_admin(storage: &mut dyn Storage, addr: Addr) -> Result<(), ContractError> {
    let admins = ADMIN.load(storage)?;
    if !admins.contains(&addr) {
        return Err(ContractError::NotAdmin { address: addr });
    }

    Ok(())
}

/// Function to check wheather the given address is agent or not
pub fn is_agent(storage: &mut dyn Storage, denom: String, addr: Addr) -> Result<(), ContractError> {
    let agent = AGENT.load(storage, denom.as_bytes())?;

    if agent != addr {
        return Err(ContractError::NotAnAgent { address: addr });
    }
    Ok(())
}

/// Function to get agent by denom
pub fn get_agent_by_id(storage: &mut dyn Storage, denom: String) -> Result<Addr, ContractError> {
    let agent = AGENT.load(storage, denom.as_bytes())?;
    Ok(agent)
}

/// Helper function for addition of user management fees
pub fn add_user_management_fees(
    storage: &mut dyn Storage,
    denom: String,
    managed_user: ManagedUser,
) -> Result<(), ContractError> {
    let key = Key::new(denom, managed_user.user.clone()).as_bytes()?;
    if !MANAGEMENT_FEES.has(storage, &key) {
        MANAGEMENT_FEES.save(storage, &key, &managed_user.fee)?;
    } else {
        return Err(ContractError::AlreadyExists {
            addr: managed_user.user,
        });
    }
    Ok(())
}

/// Helper function for updation of user management fees
pub fn update_user_management_fees(
    storage: &mut dyn Storage,
    denom: String,
    managed_user: ManagedUser,
) -> Result<(), ContractError> {
    let key = Key::new(denom, managed_user.user).as_bytes()?;
    MANAGEMENT_FEES.save(storage, &key, &managed_user.fee)?;

    Ok(())
}

/// Helper function for removal of user management fees
pub fn remove_user_management_fees(
    storage: &mut dyn Storage,
    denom: String,
    managed_user: ManagedUser,
) -> Result<(), ContractError> {
    let key = Key::new(denom, managed_user.user.clone()).as_bytes()?;
    if MANAGEMENT_FEES.has(storage, &key) {
        MANAGEMENT_FEES.remove(storage, &key);
    } else {
        return Err(ContractError::NotFound {
            addr: managed_user.user,
        });
    }

    Ok(())
}
