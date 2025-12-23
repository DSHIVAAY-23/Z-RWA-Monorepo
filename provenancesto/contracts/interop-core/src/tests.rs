use super::*;
use crate::contract::*;
use cosmwasm_std::{
    from_json,
    testing::{mock_env, mock_info, MockApi},
    MemoryStorage, OwnedDeps,
};
use provwasm_mocks::{mock_provenance_dependencies, MockProvenanceQuerier};

fn do_init(
    deps: &mut OwnedDeps<MemoryStorage, MockApi, MockProvenanceQuerier>,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    instantiate(
        deps.as_mut(),
        env,
        info,
        InitMsg {
            multi_sig: Addr::unchecked("multisig"),
            deployed_chain: String::from("Provenance"),
        },
    )
}

#[test]
fn test_init() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    let res = do_init(&mut deps, env.clone(), info.clone()).unwrap();
    assert_eq!(0, res.messages.len());
    assert_eq!(1, res.attributes.len());

    // Check admin lists
    let mut msg = QueryMsg::GetAdmins {};
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let admins: Vec<Addr> = from_json(&res).unwrap();
    assert!(admins.contains(&info.sender));

    let executor = EXECUTER.load(&deps.storage).unwrap();
    let multisig = Addr::unchecked("multisig");
    assert_eq!(executor, multisig);

    msg = QueryMsg::GetSourceChain {};
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let source_chain: String = from_json(&res).unwrap();
    assert_eq!(source_chain, String::from("Provenance"));
}

#[test]
fn test_send_mint_instruction() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    let _ = do_init(&mut deps, env.clone(), info.clone());

    let msg = ExecuteMsg::SendInstruction {
        params: SendParams {
            portfolios: vec![Portfolio {
                dest_chain: String::from("Provenance"),
                dest_address: String::from("0xe1EE8B61deB84D424C5df1daE73E404A9C2175F7"),
                investor: String::from("0x0B70373D5BA5b0Da8672fF62704bFD117211C2C2"),
                token: String::from("0xC29295f67F5d476105f19E8513da0E5027e73e39"),
                amount: 100,
                order_id: 11,
                action: Action::Mint,
            }],
        },
    };

    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    assert_eq!(7, res.attributes.len());
}

#[test]
fn test_send_burn_instruction() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    let _ = do_init(&mut deps, env.clone(), info.clone());

    let msg = ExecuteMsg::SendInstruction {
        params: SendParams {
            portfolios: vec![Portfolio {
                dest_chain: String::from("Holesky"),
                dest_address: String::from("0x5aE630fEA4a056183F534504AD8Baaa2B8Bd3a1E"),
                investor: String::from("0x0B70373D5BA5b0Da8672fF62704bFD117211C2C2"),
                token: String::from("0xC29295f67F5d476105f19E8513da0E5027e73e39"),
                amount: 100,
                order_id: 1,
                action: Action::Burn,
            }],
        },
    };

    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    assert_eq!(7, res.attributes.len());
}

#[test]
fn test_update_source_chain_config() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    do_init(&mut deps, env.clone(), info.clone()).unwrap();

    let msg = QueryMsg::GetSourceChain {};
    let res = query(deps.as_ref(), env.clone(), msg.clone()).unwrap();
    let source_chain: String = from_json(&res).unwrap();
    assert_eq!(source_chain, String::from("Provenance"));

    let chain = String::from("Holesky");
    let exe_msg = ExecuteMsg::UpdateSourceChain {
        chain: chain.clone(),
    };

    execute(deps.as_mut(), env.clone(), info, exe_msg).unwrap();

    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let source_chain: String = from_json(&res).unwrap();
    assert_eq!(source_chain, chain);
}

#[test]
fn test_add_admins() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    do_init(&mut deps, env.clone(), info.clone()).unwrap();

    let admin_1 = Addr::unchecked("admin_1");
    let admin_2 = Addr::unchecked("admin_2");
    let msg = ExecuteMsg::ManageRoles {
        roles: vec![Role::Admins {
            update_type: UpdateType::Add(vec![admin_1.clone(), admin_2.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check admin lists
    let msg = QueryMsg::GetAdmins {};
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let admins: Vec<Addr> = from_json(&res).unwrap();
    assert_eq!(admins.len(), 3);
    assert!(admins.contains(&info.sender));
    assert!(admins.contains(&admin_1));
    assert!(admins.contains(&admin_2));
}

#[test]
fn test_remove_admins() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    do_init(&mut deps, env.clone(), info.clone()).unwrap();

    let admin_1 = Addr::unchecked("admin_1");
    let admin_2 = Addr::unchecked("admin_2");

    // Add Admins
    let msg = ExecuteMsg::ManageRoles {
        roles: vec![Role::Admins {
            update_type: UpdateType::Add(vec![admin_1.clone(), admin_2.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check admin lists
    let msg = QueryMsg::GetAdmins {};
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let admins: Vec<Addr> = from_json(&res).unwrap();
    assert_eq!(admins.len(), 3);
    assert!(admins.contains(&info.sender));
    assert!(admins.contains(&admin_1));
    assert!(admins.contains(&admin_2));

    // Remove Admins
    let msg = ExecuteMsg::ManageRoles {
        roles: vec![Role::Admins {
            update_type: UpdateType::Remove(vec![admin_1.clone(), info.sender.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check admin lists
    let msg = QueryMsg::GetAdmins {};
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let admins: Vec<Addr> = from_json(&res).unwrap();
    assert_eq!(admins.len(), 1);
    assert!(!admins.contains(&info.sender));
    assert!(!admins.contains(&admin_1));
    assert!(admins.contains(&admin_2));
}

#[test]
fn test_update_executor() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    do_init(&mut deps, env.clone(), info.clone()).unwrap();

    let executor = EXECUTER.load(&deps.storage).unwrap();
    let multisig = Addr::unchecked("multisig");
    assert_eq!(executor, multisig);

    // Update Executor
    let new_executor = Addr::unchecked("new_address");
    let msg = ExecuteMsg::ManageRoles {
        roles: vec![Role::Executer {
            addr: new_executor.clone(),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check executor
    let executor = EXECUTER.load(&deps.storage).unwrap();
    assert_eq!(executor, new_executor);
}

#[test]
fn test_executor_instruction() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("multisig", &[]);

    do_init(&mut deps, env.clone(), info.clone()).unwrap();

    let executor = EXECUTER.load(&deps.storage).unwrap();
    let multisig = Addr::unchecked("multisig");
    assert_eq!(executor, multisig);

    let msg = ExecuteMsg::ExecuteInstruction {
        source_chain: String::from("Holesky"),
        source_address: String::from("0xe1EE8B61deB84D424C5df1daE73E404A9C2175F7"),
        payload: "000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000001200000000000000000000000000000000000000000000000000000000000000160000000000000000000000000000000000000000000000000000000000000000f00000000000000000000000000000000000000000000000000000000000000297470316c7a3772773370343874737a746a6171706e717a7a37767a7766637a726c6b6372776b67717900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008544a546573742d39000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002865323466353737636661666334666161653163343265396335333335616130633564353734326462000000000000000000000000000000000000000000000000".to_string(),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(res.attributes.len(), 7);
}
