#[cfg(not(feature = "library"))]
use super::*;

// version info for migration info
const CONTRACT_NAME: &str = "token_contract";
const CONTRACT_VERSION: &str = "1.0.0";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: Instantiate,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // check valid token info
    msg.validate()?;

    // admin info
    let admin = info.sender.clone();
    ADMIN.save(deps.storage, &admin)?;
    SUB_ADMIN.save(deps.storage, &vec![admin])?;

    // create initial accounts
    let total_supply = create_accounts(&mut deps, &msg)?;

    if let Some(limit) = msg.get_cap() {
        if total_supply > limit {
            return Err(StdError::generic_err("Initial supply greater than cap").into());
        }
    }

    let mint = match msg.mint {
        Some(m) => Some(MinterData {
            minter: deps.api.addr_validate(&m.minter)?,
            cap: m.cap,
        }),
        None => None,
    };

    // store token info
    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: 0,
        total_supply,
        mint,
    };
    TOKEN_INFO.save(deps.storage, &data)?;

    // Giving access to Issuer, Transfer and Tokenization Agents
    manage_roles(
        deps,
        info,
        vec![
            Role::Issuer {
                update_type: UpdateType::Add(msg.issuer),
            },
            Role::TransferAgent {
                update_type: UpdateType::Add(msg.transfer_agent),
            },
            Role::TokenizationAgent {
                update_type: UpdateType::Add(msg.tokenization_agent),
            },
        ],
    )?;

    Ok(Response::new().add_attribute("action", "intantiated"))
}

fn create_accounts(deps: &mut DepsMut, msg: &Instantiate) -> StdResult<Uint128> {
    let mut total_supply = Uint128::zero();
    for account in &msg.initial_balances {
        let address = deps.api.addr_validate(&account.address)?;
        BALANCES.save(deps.storage, &address, &account.amount)?;
        total_supply += account.amount;
    }

    Ok(total_supply)
}
