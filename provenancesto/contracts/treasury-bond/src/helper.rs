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

pub fn add_or_update_payments(
    storage: &mut dyn Storage,
    denom: String,
    user: Addr,
    payment: Uint128,
) -> Result<(), ContractError> {
    let key = Key::new(denom, user.clone()).as_bytes()?;
    let payments = Payments::new(payment);

    if PAYMENTS
        .update(storage, &key, |pay_opt| -> Result<_, ContractError> {
            match pay_opt {
                Some(mut pay) => Ok({
                    pay.add(payments.clone());
                    pay
                }),
                None => Ok(payments.clone()),
            }
        })
        .is_err()
    {
        PAYMENTS.save(storage, &key, &payments)?;
    }

    Ok(())
}
