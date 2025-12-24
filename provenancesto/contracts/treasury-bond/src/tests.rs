use super::*;
use crate::contract::*;
use cosmwasm_std::{
    from_binary,
    testing::{mock_env, mock_info, MockApi},
    MemoryStorage, OwnedDeps,
};
use provwasm_mocks::{mock_provenance_dependencies, MockProvenanceQuerier};

fn do_init(
    deps: &mut OwnedDeps<MemoryStorage, MockApi, MockProvenanceQuerier>,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    instantiate(deps.as_mut(), env, info, InitMsg {})
}

fn do_create_treasury_contract(
    deps: &mut OwnedDeps<MemoryStorage, MockApi, MockProvenanceQuerier>,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    do_init(deps, env.clone(), info.clone()).unwrap();

    let msg = ExecuteMsg::Create {
        params: CreateParams {
            denom: "budz".into(),
            issuer_name: "issuer".into(),
            issue_size: 1,
            face_value: 1,
            coupon_rate: 8,
            accrued_interest: 7,
            maturity_date: 1212454212,
            coupon_frequency: "Monthly".into(),
        },
    };
    execute(deps.as_mut(), env.clone(), info.clone(), msg)
}

#[test]
fn test_init() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);

    let res = do_init(&mut deps, env.clone(), info.clone()).unwrap();
    assert_eq!(
        res,
        Response::new().add_attributes(vec![attr("action", "provwasm.contracts.treasury.init",)])
    );
    assert!(is_admin(&mut deps.storage, sender.clone()).is_ok());

    let msg = QueryMsg::GetAdmins {};
    let res = query(deps.as_ref(), env, msg).unwrap();
    let admins: Vec<Addr> = from_binary(&res).unwrap();
    assert!(admins.contains(&sender));
}

#[test]
fn test_add_admins() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);
    let admin_1 = Addr::unchecked("admin_1");
    let admin_2 = Addr::unchecked("admin_2");

    do_init(&mut deps, env.clone(), info.clone()).unwrap();

    // Try adding admins
    let mut exec_msg = ExecuteMsg::ManageRoles {
        role: Role::Admin {
            update_type: UpdateType::Add(vec![admin_1.clone(), admin_2.clone()]),
        },
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attributes(vec![attr(
            "action",
            "provwasm.contracts.treasury.add_admin",
        )])
    );
    assert!(is_admin(&mut deps.storage, sender.clone()).is_ok());
    assert!(is_admin(&mut deps.storage, admin_1.clone()).is_ok());
    assert!(is_admin(&mut deps.storage, admin_2.clone()).is_ok());

    let msg = QueryMsg::GetAdmins {};
    let res = query(deps.as_ref(), env.clone(), msg.clone()).unwrap();
    let admins: Vec<Addr> = from_binary(&res).unwrap();
    assert_eq!(admins.len(), 3);
    assert!(admins.contains(&sender));
    assert!(admins.contains(&admin_1));
    assert!(admins.contains(&admin_2));

    // Try adding duplicates admins
    exec_msg = ExecuteMsg::ManageRoles {
        role: Role::Admin {
            update_type: UpdateType::Add(vec![admin_1.clone()]),
        },
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap();
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let admins: Vec<Addr> = from_binary(&res).unwrap();
    // Admin list's size remain same
    assert_eq!(admins.len(), 3);

    // Try adding with non-admin accounts
    let sender = Addr::unchecked("non-admin");
    let info = mock_info(&sender.to_string(), &[]);
    let err = execute(deps.as_mut(), env, info, exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAdmin { address: sender });
}

#[test]
fn test_remove_admins() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);
    let admin_1 = Addr::unchecked("admin_1");
    let admin_2 = Addr::unchecked("admin_2");

    do_init(&mut deps, env.clone(), info.clone()).unwrap();

    // Try adding admins
    let mut exec_msg = ExecuteMsg::ManageRoles {
        role: Role::Admin {
            update_type: UpdateType::Add(vec![admin_1.clone(), admin_2.clone()]),
        },
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();
    let msg = QueryMsg::GetAdmins {};
    let res = query(deps.as_ref(), env.clone(), msg.clone()).unwrap();
    let admins: Vec<Addr> = from_binary(&res).unwrap();
    assert_eq!(admins.len(), 3);
    let old_admin_count = admins.len();

    // Try removing admin
    exec_msg = ExecuteMsg::ManageRoles {
        role: Role::Admin {
            update_type: UpdateType::Remove(vec![admin_2.clone()]),
        },
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attributes(vec![attr(
            "action",
            "provwasm.contracts.treasury.remove_admin",
        )])
    );
    let mut err = is_admin(&mut deps.storage, admin_2.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotAdmin {
            address: admin_2.clone()
        }
    );

    let res = query(deps.as_ref(), env.clone(), msg.clone()).unwrap();
    let admins: Vec<Addr> = from_binary(&res).unwrap();
    // Admin list's size reduced
    assert_eq!(admins.len(), old_admin_count - 1);

    // Try removing non admins
    exec_msg = ExecuteMsg::ManageRoles {
        role: Role::Admin {
            update_type: UpdateType::Remove(vec![admin_2.clone()]),
        },
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap();
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let admins: Vec<Addr> = from_binary(&res).unwrap();
    // Admin list's size remain same
    assert_eq!(admins.len(), old_admin_count - 1);

    // Try removing with non-admin accounts
    let sender = Addr::unchecked("non-admin");
    let info = mock_info(&sender.to_string(), &[]);
    err = execute(deps.as_mut(), env, info, exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAdmin { address: sender });
}

#[test]
fn test_update_agent() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);
    let agent_1 = Addr::unchecked("agent_1");
    let denom = String::from("budz");

    do_create_treasury_contract(&mut deps, env.clone(), info.clone()).unwrap();
    assert!(is_agent(&mut deps.storage, denom.clone(), sender.clone()).is_ok());

    // Adding new agent
    let exec_msg = ExecuteMsg::ManageRoles {
        role: Role::Agent {
            denom: denom.clone(),
            address: agent_1.clone(),
        },
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap();
}

#[test]
fn test_create_treasury_contract() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);
    let denom = String::from("budz");

    do_create_treasury_contract(&mut deps, env.clone(), info.clone()).unwrap();
    assert!(is_admin(&mut deps.storage, sender.clone()).is_ok());
    assert!(is_agent(&mut deps.storage, denom.clone(), sender.clone()).is_ok());

    let msg = QueryMsg::GetAgentByDenom { denom };
    let res = query(deps.as_ref(), env, msg).unwrap();
    let agent: Addr = from_binary(&res).unwrap();
    assert_eq!(agent, sender);
}

#[test]
fn test_share_dividend() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let agent = Addr::unchecked("agent");
    let info = mock_info(&agent.to_string(), &[]);
    let denom = String::from("budz");
    let user_1 = Addr::unchecked("user_1");
    let user_2 = Addr::unchecked("user_2");
    let user_3 = Addr::unchecked("user_3");
    let payment_1 = Uint128::new(100);
    let payment_2 = Uint128::new(200);
    let payment_3 = Uint128::new(300);

    do_create_treasury_contract(&mut deps, env.clone(), info.clone()).unwrap();

    // Share Dividend
    let mut exec_msg = ExecuteMsg::ShareStableCoin {
        denom: denom.clone(),
        coin_type: CoinType::Usdt,
        share_params: vec![
            ShareParams {
                to: user_1.clone(),
                payment: payment_1,
            },
            ShareParams {
                to: user_2.clone(),
                payment: payment_2,
            },
            ShareParams {
                to: user_3.clone(),
                payment: payment_3,
            },
        ],
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();

    // Try with non-agent accounts
    let sender = Addr::unchecked("non-agent");
    let info = mock_info(&sender.to_string(), &[]);
    exec_msg = ExecuteMsg::ShareStableCoin {
        denom: denom.clone(),
        coin_type: CoinType::Usdt,
        share_params: vec![ShareParams {
            to: user_1,
            payment: payment_1,
        }],
    };
    let err = execute(deps.as_mut(), env, info.clone(), exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAnAgent { address: sender });
}

#[test]
fn test_update_credit_rating() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let agent = Addr::unchecked("agent");
    let info = mock_info(&agent.to_string(), &[]);
    let denom = String::from("budz");

    do_create_treasury_contract(&mut deps, env.clone(), info.clone()).unwrap();

    let mut global_config = GLOBAL_CONFIG.load(&deps.storage, denom.as_bytes()).unwrap();
    assert_eq!(global_config.credit_rating, String::default());

    // Update currency
    let rating = String::from("Rating");
    let exec_msg = ExecuteMsg::UpdateCreditRating {
        denom: denom.clone(),
        rating: rating.to_string(),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap();

    global_config = GLOBAL_CONFIG.load(&deps.storage, denom.as_bytes()).unwrap();
    assert_eq!(global_config.credit_rating, rating);

    // Try with non-agent accounts
    let sender = Addr::unchecked("non-agent");
    let info = mock_info(&sender.to_string(), &[]);
    let err = execute(deps.as_mut(), env, info, exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAnAgent { address: sender });
}

#[test]
fn test_send_stable_coins() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let agent = Addr::unchecked("agent");
    let info = mock_info(&agent.to_string(), &[]);
    let denom = String::from("budz");

    do_create_treasury_contract(&mut deps, env.clone(), info.clone()).unwrap();

    // Send stable coins
    let exec_msg = ExecuteMsg::SendStableCoins {
        denom: denom.clone(),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();
}
