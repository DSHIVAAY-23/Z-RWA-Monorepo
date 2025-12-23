use super::*;
use crate::contract::*;
use cosmwasm_std::{
    from_binary,
    testing::{mock_env, mock_info, MockApi},
    MemoryStorage, OwnedDeps,
};
use provwasm_mocks::{mock_dependencies as mock_provenance_dependencies, ProvenanceMockQuerier};
use provwasm_std::ProvenanceQuery;

fn do_init(
    deps: &mut OwnedDeps<MemoryStorage, MockApi, ProvenanceMockQuerier, ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    instantiate(deps.as_mut(), env, info, InitMsg {})
}

fn do_create_fund(
    deps: &mut OwnedDeps<MemoryStorage, MockApi, ProvenanceMockQuerier, ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    do_init(deps, env.clone(), info.clone()).unwrap();

    let msg = ExecuteMsg::Create {
        params: CreateParams {
            denom: "budz".into(),
            fund_name: "test".into(),
            asset_type: AssetType::Token,
            issuer_name: "issuer".into(),
            target_aum: Uint128::default(),
            nav_launch_price: Uint128::default(),
            ccy: "USD".into(),
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
        Response::new().add_attributes(vec![attr("action", "provwasm.contracts.fund.init",)])
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
    let mut exec_msg = ExecuteMsg::ManageAdmins {
        update_type: UpdateType::Add(vec![admin_1.clone(), admin_2.clone()]),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attributes(vec![attr("action", "provwasm.contracts.fund.add_admin",)])
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
    exec_msg = ExecuteMsg::ManageAdmins {
        update_type: UpdateType::Add(vec![admin_1.clone()]),
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
    let mut exec_msg = ExecuteMsg::ManageAdmins {
        update_type: UpdateType::Add(vec![admin_1.clone(), admin_2.clone()]),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();
    let msg = QueryMsg::GetAdmins {};
    let res = query(deps.as_ref(), env.clone(), msg.clone()).unwrap();
    let admins: Vec<Addr> = from_binary(&res).unwrap();
    assert_eq!(admins.len(), 3);
    let old_admin_count = admins.len();

    // Try removing admin
    exec_msg = ExecuteMsg::ManageAdmins {
        update_type: UpdateType::Remove(vec![admin_2.clone()]),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attributes(vec![
                attr("action", "provwasm.contracts.fund.remove_admin",)
            ])
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
    exec_msg = ExecuteMsg::ManageAdmins {
        update_type: UpdateType::Remove(vec![admin_2.clone()]),
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
fn test_remove_agent() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);
    let agent_1 = Addr::unchecked("agent_1");
    let denom = String::from("budz");

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();
    assert!(is_agent(&mut deps.storage, denom.clone(), sender.clone()).is_ok());

    // Try removing non-agent account
    let mut exec_msg = ExecuteMsg::ManageAgent {
        denom: denom.clone(),
        update_type: UpdateType::Remove(agent_1.clone()),
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotFound {
            addr: agent_1.clone()
        }
    );

    // Removing current agent `sender`
    exec_msg = ExecuteMsg::ManageAgent {
        denom: denom.clone(),
        update_type: UpdateType::Remove(sender.clone()),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attributes(vec![attr("action", "provwasm.contracts.fund.remove_agent")])
    );
    let err = is_agent(&mut deps.storage, denom.clone(), sender.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::Std(StdError::not_found("cosmwasm_std::addresses::Addr"))
    );

    // Try removing same agent
    let err = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotFound {
            addr: sender.clone()
        }
    );

    // Adding new agent
    exec_msg = ExecuteMsg::ManageAgent {
        denom: denom.clone(),
        update_type: UpdateType::Add(agent_1.clone()),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap();

    // Try removing with non-admin accounts
    exec_msg = ExecuteMsg::ManageAgent {
        denom: denom.clone(),
        update_type: UpdateType::Remove(agent_1.clone()),
    };
    let sender = Addr::unchecked("non-admin");
    let info = mock_info(&sender.to_string(), &[]);
    let err = execute(deps.as_mut(), env, info, exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAdmin { address: sender });
}

#[test]
fn test_remove_agents() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);
    let agent_1 = Addr::unchecked("agent_1");
    let denom = String::from("budz");

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();
    assert!(is_agent(&mut deps.storage, denom.clone(), sender.clone()).is_ok());

    // Try adding agent, but fail as agent `sender` already present
    let mut exec_msg = ExecuteMsg::ManageAgent {
        denom: denom.clone(),
        update_type: UpdateType::Add(agent_1.clone()),
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::AlreadyExists {
            addr: agent_1.clone()
        }
    );

    // Removing current agent `sender` before adding the new one
    exec_msg = ExecuteMsg::ManageAgent {
        denom: denom.clone(),
        update_type: UpdateType::Remove(sender.clone()),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attributes(vec![attr("action", "provwasm.contracts.fund.remove_agent")])
    );
    let err = is_agent(&mut deps.storage, denom.clone(), sender.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::Std(StdError::not_found("cosmwasm_std::addresses::Addr"))
    );

    // Finally adding new agent
    exec_msg = ExecuteMsg::ManageAgent {
        denom: denom.clone(),
        update_type: UpdateType::Add(agent_1.clone()),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap();
    assert_eq!(
        res,
        Response::new().add_attributes(vec![attr("action", "provwasm.contracts.fund.add_agent",)])
    );
    assert!(is_agent(&mut deps.storage, denom.clone(), agent_1.clone()).is_ok());

    let msg = QueryMsg::GetAgentByDenom {
        denom: denom.clone(),
    };
    let res = query(deps.as_ref(), env.clone(), msg.clone()).unwrap();
    let agent: Addr = from_binary(&res).unwrap();
    assert_eq!(agent_1, agent);

    // Try adding same agent
    let err = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::AlreadyExists {
            addr: agent_1.clone()
        }
    );

    // Try adding with non-admin accounts
    let sender = Addr::unchecked("non-admin");
    let info = mock_info(&sender.to_string(), &[]);
    let err = execute(deps.as_mut(), env, info, exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAdmin { address: sender });
}

#[test]
fn test_create_fund() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);
    let denom = String::from("budz");

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();
    assert!(is_admin(&mut deps.storage, sender.clone()).is_ok());
    assert!(is_agent(&mut deps.storage, denom.clone(), sender.clone()).is_ok());

    let msg = QueryMsg::GetAgentByDenom { denom };
    let res = query(deps.as_ref(), env, msg).unwrap();
    let agent: Addr = from_binary(&res).unwrap();
    assert_eq!(agent, sender);
}

#[test]
fn test_add_user_management_fees() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);
    let denom = String::from("budz");
    let user_1 = Addr::unchecked("user_1");
    let user_2 = Addr::unchecked("user_2");
    let fees_1 = Uint128::new(100);
    let fees_2 = Uint128::new(200);

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();

    // Adding fees
    let mut exec_msg = ExecuteMsg::ManagementFees {
        denom: denom.clone(),
        managed_users: UpdateType::Add(vec![
            ManagedUser {
                user: user_1.clone(),
                fee: fees_1,
            },
            ManagedUser {
                user: user_2.clone(),
                fee: fees_2,
            },
        ]),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();

    let mut msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_1.clone(),
    };
    let mut res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let mut fees: Uint128 = from_binary(&res).unwrap();
    assert_eq!(fees, fees_1);

    msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_2,
    };
    res = query(deps.as_ref(), env.clone(), msg).unwrap();
    fees = from_binary(&res).unwrap();
    assert_eq!(fees, fees_2);

    // Try adding same user again
    exec_msg = ExecuteMsg::ManagementFees {
        denom: denom.clone(),
        managed_users: UpdateType::Add(vec![ManagedUser {
            user: user_1.clone(),
            fee: fees_1,
        }]),
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::AlreadyExists {
            addr: user_1.clone()
        }
    );

    // Try adding same user again with missing denom
    let sample_denom = String::from("sample");
    exec_msg = ExecuteMsg::ManagementFees {
        denom: sample_denom,
        managed_users: UpdateType::Add(vec![ManagedUser {
            user: user_1.clone(),
            fee: fees_1,
        }]),
    };
    let err = execute(deps.as_mut(), env.clone(), info, exec_msg).unwrap_err();
    assert_eq!(
        err,
        StdError::not_found("cosmwasm_std::addresses::Addr").into()
    );

    // Try adding with non-agent accounts
    let sender = Addr::unchecked("non-agent");
    let info = mock_info(&sender.to_string(), &[]);
    exec_msg = ExecuteMsg::ManagementFees {
        denom,
        managed_users: UpdateType::Add(vec![ManagedUser {
            user: user_1.clone(),
            fee: fees_1,
        }]),
    };
    let err = execute(deps.as_mut(), env, info, exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAnAgent { address: sender });
}

#[test]
fn test_update_user_management_fees_without_addition() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);
    let denom = String::from("budz");
    let user_1 = Addr::unchecked("user_1");
    let user_2 = Addr::unchecked("user_2");
    let mut fees_1 = Uint128::new(100);
    let mut fees_2 = Uint128::new(200);

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();

    // Adding fees
    let mut exec_msg = ExecuteMsg::ManagementFees {
        denom: denom.clone(),
        managed_users: UpdateType::Add(vec![
            ManagedUser {
                user: user_1.clone(),
                fee: fees_1,
            },
            ManagedUser {
                user: user_2.clone(),
                fee: fees_2,
            },
        ]),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();

    let mut msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_1.clone(),
    };
    let mut res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let mut fees: Uint128 = from_binary(&res).unwrap();
    assert_eq!(fees, fees_1);

    msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_2.clone(),
    };
    res = query(deps.as_ref(), env.clone(), msg).unwrap();
    fees = from_binary(&res).unwrap();
    assert_eq!(fees, fees_2);

    // Updating fees
    fees_1 = Uint128::new(300);
    fees_2 = Uint128::new(400);

    exec_msg = ExecuteMsg::ManagementFees {
        denom: denom.clone(),
        managed_users: UpdateType::Update(vec![
            ManagedUser {
                user: user_1.clone(),
                fee: fees_1,
            },
            ManagedUser {
                user: user_2.clone(),
                fee: fees_2,
            },
        ]),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();

    msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_1.clone(),
    };
    res = query(deps.as_ref(), env.clone(), msg).unwrap();
    fees = from_binary(&res).unwrap();
    assert_eq!(fees, fees_1);

    msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_2.clone(),
    };
    res = query(deps.as_ref(), env.clone(), msg).unwrap();
    fees = from_binary(&res).unwrap();
    assert_eq!(fees, fees_2);

    // Try updating with non-agent accounts
    let sender = Addr::unchecked("non-agent");
    let info = mock_info(&sender.to_string(), &[]);
    exec_msg = ExecuteMsg::ManagementFees {
        denom,
        managed_users: UpdateType::Update(vec![ManagedUser {
            user: user_1.clone(),
            fee: fees_1,
        }]),
    };
    let err = execute(deps.as_mut(), env, info, exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAnAgent { address: sender });
}

#[test]
fn test_update_user_management_fees() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);
    let denom = String::from("budz");
    let user_1 = Addr::unchecked("user_1");
    let user_2 = Addr::unchecked("user_2");
    let fees_1 = Uint128::new(100);
    let fees_2 = Uint128::new(200);

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();

    // Updating fees
    let exec_msg = ExecuteMsg::ManagementFees {
        denom: denom.clone(),
        managed_users: UpdateType::Update(vec![
            ManagedUser {
                user: user_1.clone(),
                fee: fees_1,
            },
            ManagedUser {
                user: user_2.clone(),
                fee: fees_2,
            },
        ]),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();

    let mut msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_1.clone(),
    };
    let mut res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let mut fees: Uint128 = from_binary(&res).unwrap();
    assert_eq!(fees, fees_1);

    msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_2.clone(),
    };
    res = query(deps.as_ref(), env, msg).unwrap();
    fees = from_binary(&res).unwrap();
    assert_eq!(fees, fees_2);
}

#[test]
fn test_remove_user_management_fees() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(&sender.to_string(), &[]);
    let denom = String::from("budz");
    let user_1 = Addr::unchecked("user_1");
    let user_2 = Addr::unchecked("user_2");
    let fees_1 = Uint128::new(100);
    let fees_2 = Uint128::new(200);

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();

    // Adding fees
    let mut exec_msg = ExecuteMsg::ManagementFees {
        denom: denom.clone(),
        managed_users: UpdateType::Add(vec![
            ManagedUser {
                user: user_1.clone(),
                fee: fees_1,
            },
            ManagedUser {
                user: user_2.clone(),
                fee: fees_2,
            },
        ]),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();

    let mut msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_1.clone(),
    };
    let mut res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let mut fees: Uint128 = from_binary(&res).unwrap();
    assert_eq!(fees, fees_1);

    msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_2.clone(),
    };
    res = query(deps.as_ref(), env.clone(), msg).unwrap();
    fees = from_binary(&res).unwrap();
    assert_eq!(fees, fees_2);

    // Remove management fees
    exec_msg = ExecuteMsg::ManagementFees {
        denom: denom.clone(),
        managed_users: UpdateType::Remove(vec![
            ManagedUser {
                user: user_1.clone(),
                fee: fees_1,
            },
            ManagedUser {
                user: user_2.clone(),
                fee: fees_2,
            },
        ]),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();

    msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_1.clone(),
    };
    res = query(deps.as_ref(), env.clone(), msg).unwrap();
    fees = from_binary(&res).unwrap();
    assert_eq!(fees, Uint128::zero());

    // Try removing user with missing denom
    let sample_denom = String::from("sample");
    exec_msg = ExecuteMsg::ManagementFees {
        denom: sample_denom,
        managed_users: UpdateType::Remove(vec![ManagedUser {
            user: user_2.clone(),
            fee: fees_1,
        }]),
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap_err();
    assert_eq!(
        err,
        StdError::not_found("cosmwasm_std::addresses::Addr").into()
    );

    msg = QueryMsg::GetManagementFees {
        denom: denom.clone(),
        user: user_2,
    };
    res = query(deps.as_ref(), env.clone(), msg).unwrap();
    fees = from_binary(&res).unwrap();
    assert_eq!(fees, Uint128::zero());

    // Try removing user that is not present
    let user_3 = Addr::unchecked("user_3");
    exec_msg = ExecuteMsg::ManagementFees {
        denom: denom.clone(),
        managed_users: UpdateType::Remove(vec![ManagedUser {
            user: user_3.clone(),
            fee: fees_1,
        }]),
    };
    let err = execute(deps.as_mut(), env.clone(), info, exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotFound { addr: user_3 });

    // Try removing with non-agent accounts
    let sender = Addr::unchecked("non-agent");
    let info = mock_info(&sender.to_string(), &[]);
    exec_msg = ExecuteMsg::ManagementFees {
        denom,
        managed_users: UpdateType::Remove(vec![ManagedUser {
            user: user_1.clone(),
            fee: fees_1,
        }]),
    };
    let err = execute(deps.as_mut(), env, info.clone(), exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAnAgent { address: sender });
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
    let dividend_1 = Uint128::new(100);
    let dividend_2 = Uint128::new(200);
    let dividend_3 = Uint128::new(300);

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();

    // Share Dividend
    let mut exec_msg = ExecuteMsg::ShareDividend {
        denom: denom.clone(),
        coin_type: CoinType::Usdt,
        shared_dividends: vec![
            SharedDividend {
                to: user_1.clone(),
                dividend: dividend_1,
                asset_type: AssetType::Token,
            },
            SharedDividend {
                to: user_2.clone(),
                dividend: dividend_2,
                asset_type: AssetType::StableCoin,
            },
            SharedDividend {
                to: user_3.clone(),
                dividend: dividend_3,
                asset_type: AssetType::Fiat,
            },
        ],
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();

    // Dividend for user_1
    let mut key = Key::new(denom.to_string(), user_1.clone())
        .as_bytes()
        .unwrap();
    let mut div = DIVIDEND.load(&deps.storage, &key).unwrap();
    assert_eq!(div, Dividend::Token(dividend_1));

    // Dividend for user_2
    key = Key::new(denom.to_string(), user_2).as_bytes().unwrap();
    div = DIVIDEND.load(&deps.storage, &key).unwrap();
    assert_eq!(div, Dividend::StableCoin(dividend_2));

    // Dividend for user_3
    key = Key::new(denom.to_string(), user_3).as_bytes().unwrap();
    div = DIVIDEND.load(&deps.storage, &key).unwrap();
    assert_eq!(div, Dividend::Fiat(dividend_3));

    // Try with non-agent accounts
    let sender = Addr::unchecked("non-agent");
    let info = mock_info(&sender.to_string(), &[]);
    exec_msg = ExecuteMsg::ShareDividend {
        denom: denom.clone(),
        coin_type: CoinType::Usdt,
        shared_dividends: vec![SharedDividend {
            to: user_1,
            dividend: dividend_1,
            asset_type: AssetType::Token,
        }],
    };
    let err = execute(deps.as_mut(), env, info.clone(), exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAnAgent { address: sender });
}

#[test]
fn test_distribute_and_burn() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let agent = Addr::unchecked("agent");
    let info = mock_info(&agent.to_string(), &[]);
    let denom = String::from("budz");
    let investor_1 = Addr::unchecked("investor_1");
    let investor_2 = Addr::unchecked("investor_2");
    let investor_3 = Addr::unchecked("investor_3");
    let token_1 = Uint128::new(100);
    let token_2 = Uint128::new(200);
    let token_3 = Uint128::new(300);
    let amount_1 = Uint128::new(200);
    let amount_2 = Uint128::new(400);
    let amount_3 = Uint128::new(600);

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();

    // Distribute and Burn
    let mut exec_msg = ExecuteMsg::DistributeAndBurn {
        denom: denom.clone(),
        coin_type: CoinType::Usdt,
        distributions: vec![
            Distribution {
                investor: investor_1.clone(),
                amount: amount_1,
                token: token_1,
            },
            Distribution {
                investor: investor_2,
                amount: amount_2,
                token: token_2,
            },
            Distribution {
                investor: investor_3,
                amount: amount_3,
                token: token_3,
            },
        ],
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();

    // Try with non-agent accounts
    let sender = Addr::unchecked("non-agent");
    let info = mock_info(&sender.to_string(), &[]);
    exec_msg = ExecuteMsg::DistributeAndBurn {
        denom: denom.clone(),
        coin_type: CoinType::Usdt,
        distributions: vec![Distribution {
            investor: investor_1,
            amount: amount_1,
            token: token_1,
        }],
    };
    let err = execute(deps.as_mut(), env, info.clone(), exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAnAgent { address: sender });
}

#[test]
fn test_rescue_token() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let agent = Addr::unchecked("agent");
    let info = mock_info(&agent.to_string(), &[]);
    let denom = String::from("budz");
    let to = Addr::unchecked("to");
    let amount = Uint128::new(500);

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();

    // Rescue Token
    let exec_msg = ExecuteMsg::SendStableCoins {
        denom: denom.clone(),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();
}

#[test]
#[should_panic]
fn test_fetch_price() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let agent = Addr::unchecked("agent");
    let info = mock_info(&agent.to_string(), &[]);
    let denom = String::from("budz");

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();

    // Fetching price
    let exec_msg = ExecuteMsg::FetchPrice { denom };

    // Will fail as price oracle contract is missing for testing
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap();

    // Try with non-admin accounts
    let sender = Addr::unchecked("non-admin");
    let info = mock_info(&sender.to_string(), &[]);
    let err = execute(deps.as_mut(), env, info, exec_msg).unwrap_err();
    assert_eq!(err, ContractError::NotAdmin { address: sender });
}

#[test]
fn test_update_currency() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let agent = Addr::unchecked("agent");
    let info = mock_info(&agent.to_string(), &[]);
    let denom = String::from("budz");

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();

    let mut global_config = GLOBAL_CONFIG.load(&deps.storage, denom.as_bytes()).unwrap();
    assert_eq!(global_config.ccy, String::from("USD"));

    // Update currency
    let ccy = String::from("INR");
    let exec_msg = ExecuteMsg::UpdateCurrency {
        denom: denom.clone(),
        ccy: ccy.clone(),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap();

    global_config = GLOBAL_CONFIG.load(&deps.storage, denom.as_bytes()).unwrap();
    assert_eq!(global_config.ccy, ccy);

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

    do_create_fund(&mut deps, env.clone(), info.clone()).unwrap();

    // Send stable coins
    let exec_msg = ExecuteMsg::SendStableCoins {
        denom: denom.clone(),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();
}
