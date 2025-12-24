use super::*;

pub fn is_admin(deps: &DepsMut, addr: Addr) -> Result<(), ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    if admin != addr {
        return Err(ContractError::NotAdmin { address: addr });
    }

    Ok(())
}

pub fn is_subadmin(deps: &DepsMut, address: Addr) -> Result<(), ContractError> {
    SUB_ADMIN.load(deps.storage).map_or(
        Err(ContractError::NotSubAdmin {
            address: address.clone(),
        }),
        |sub_admins| {
            if !sub_admins.contains(&address) {
                Err(ContractError::NotSubAdmin { address })
            } else {
                Ok(())
            }
        },
    )
}

pub fn get_denom_by_contract(storage: &dyn Storage, addr: Addr) -> Result<String, StdError> {
    CONTRACT_TO_DENOM.load(storage, addr)
}

pub fn get_contract_by_denom(storage: &dyn Storage, denom: String) -> Result<Addr, StdError> {
    DENOM_TO_CONTRACT.load(storage, denom)
}

pub fn get_code_id(storage: &dyn Storage) -> Result<u64, StdError> {
    CODE_ID.load(storage)
}
