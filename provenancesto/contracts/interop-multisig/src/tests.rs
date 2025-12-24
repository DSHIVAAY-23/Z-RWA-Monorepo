use super::*;
use crate::contract::*;
use cosmwasm_std::{
    coin, from_binary,
    testing::{mock_env, mock_info, MockApi},
    Binary, CosmosMsg, MemoryStorage, OwnedDeps,
};
use prost::Message;
use provwasm_mocks::{mock_provenance_dependencies, MockProvenanceQuerier};
use provwasm_std::{
    shim::Any,
    types::{
        cosmos::{auth::v1beta1::BaseAccount, base::v1beta1::Coin},
        provenance::marker::v1::{
            AccessGrant, MarkerAccount, MarkerStatus, MsgAddMarkerRequest, MsgBurnRequest,
            MsgCancelRequest, MsgDeleteRequest, MsgMintRequest, MsgTransferRequest,
            MsgWithdrawRequest, QueryEscrowRequest, QueryEscrowResponse, QueryMarkerRequest,
            QueryMarkerResponse,
        },
    },
};
use std::convert::TryInto;

fn do_init(
    deps: &mut OwnedDeps<MemoryStorage, MockApi, MockProvenanceQuerier>,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    instantiate(deps.as_mut(), env, info, InitMsg {})
}

fn do_add_sub_admin(
    deps: &mut OwnedDeps<MemoryStorage, MockApi, MockProvenanceQuerier>,
    env: Env,
    info: MessageInfo,
) {
    let _ = do_init(deps, env.clone(), info.clone());

    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![info.sender.clone()]),
        }],
    };
    execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
}

fn do_create_marker(
    deps: &mut OwnedDeps<MemoryStorage, MockApi, MockProvenanceQuerier>,
    env: Env,
    info: MessageInfo,
    denom: String,
) -> Result<Response, ContractError> {
    let _ = do_init(deps, env.clone(), info.clone());

    let country_codes = vec![91, 1];

    let mut msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![info.sender.clone()]),
        }],
    };
    let _ = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create marker execute message
    msg = ExecuteMsg::Create {
        params: CreateParams {
            supply: Uint128::new(10_000),
            denom,
            denom_config: DenomConfig {
                token_limit: Uint128::new(1000),
                country_codes,
            },
            issuer: Addr::unchecked("issuer"),
            transfer_agent: Addr::unchecked("transfer_agent"),
            tokenization_agent: Addr::unchecked("tokenization_agent"),
            id: "unique".into(),
        },
    };

    execute(deps.as_mut(), env, info, msg)
}

fn do_whitelist(
    deps: &mut OwnedDeps<MemoryStorage, MockApi, MockProvenanceQuerier>,
    env: Env,
    info: MessageInfo,
    address: Addr,
) -> Result<Response, ContractError> {
    // Create whitelisting
    let msg = ExecuteMsg::Whitelist {
        lists: vec![WhiteListParams {
            denom: "budz".into(),
            data: vec![CountryCodeData {
                address,
                country_code: UpdateKind::Set(91),
            }],
        }],
    };

    execute(deps.as_mut(), env, info, msg)
}

fn do_add_issuer(
    deps: &mut OwnedDeps<MemoryStorage, MockApi, MockProvenanceQuerier>,
    env: Env,
    info: MessageInfo,
    address: Addr,
) {
    do_add_sub_admin(deps, env.clone(), info.clone());

    let msg = ExecuteMsg::ManageRoles {
        denom: "budz".into(),
        roles: vec![Role::Issuer {
            update_type: UpdateType::Add(address),
        }],
    };

    execute(deps.as_mut(), env, info, msg).unwrap();
}

#[test]
fn create_marker() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);
    let denom = String::from("budz");
    let contract_address = env.contract.address.clone();

    let _ = do_init(&mut deps, env.clone(), info.clone());

    // Expected marker supply
    let expected_msg: Binary = MsgAddMarkerRequest {
        amount: Some(Coin {
            denom: denom.clone(),
            amount: 10_000.to_string(),
        }),
        manager: contract_address.to_string(),
        from_address: contract_address.to_string(),
        status: MarkerStatus::Finalized.into(),
        marker_type: MarkerType::Restricted.into(),
        access_list: all_access(&contract_address),
        supply_fixed: false,
        allow_governance_control: false,
        allow_forced_transfer: true,
        required_attributes: vec![],
    }
    .try_into()
    .unwrap();

    // Call execute and ensure a cosmos message was dispatched
    let res = do_create_marker(&mut deps, env.clone(), info, denom.clone()).unwrap();
    assert_eq!(2, res.messages.len());

    // Assert the correct params were created
    match &res.messages[0].msg {
        CosmosMsg::Stargate { type_url, value } => {
            assert_eq!(type_url, "/provenance.marker.v1.MsgAddMarkerRequest");
            assert_eq!(value, &expected_msg)
        }
        _ => panic!("unexpected cosmos message"),
    }

    // Check authorized country lists
    let mut msg = QueryMsg::GetAuthorizedCountries {
        denom: denom.clone(),
    };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let country_codes: Vec<u8> = from_binary(&res).unwrap();
    assert!(country_codes.contains(&91));
    assert!(country_codes.contains(&1));

    // Check denom config
    msg = QueryMsg::GetDenomConfig {
        denom: denom.clone(),
    };
    let res = query(deps.as_ref(), env, msg).unwrap();
    let denom_config: DenomConfig = from_binary(&res).unwrap();
    assert!(denom_config.country_codes.contains(&91));
    assert_eq!(denom_config.token_limit.u128(), 1000);

    // Verifying accesses
    assert!(is_issuer(&deps.as_mut(), denom.clone(), Addr::unchecked("issuer")).is_ok());
    assert!(is_transfer_agent(
        &deps.as_mut(),
        denom.clone(),
        Addr::unchecked("transfer_agent")
    )
    .is_ok());
    assert!(is_tokenization_agent(
        &deps.as_mut(),
        denom.clone(),
        Addr::unchecked("tokenization_agent")
    )
    .is_ok());
}

#[test]
fn whitelist() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let info = mock_info(sender.as_str(), &[]);
    let denom = String::from("budz");

    let to_address = Addr::unchecked("toaddress");
    let err = do_whitelist(&mut deps, env.clone(), info.clone(), to_address.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::Unauthorized {
            err: "Address `sender`: Don't have Tokenization and Sub Admin rights!".into()
        }
    );

    do_add_sub_admin(&mut deps, env.clone(), info.clone());

    let res = do_whitelist(&mut deps, env.clone(), info.clone(), to_address.clone()).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = QueryMsg::GetCountryCodeByAddress {
        denom: denom.clone(),
        address: to_address.clone(),
    };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let country_code: u8 = from_binary(&res).unwrap();
    assert_eq!(country_code, 91);

    // Unsetting the whitelisting
    let msg = ExecuteMsg::Whitelist {
        lists: vec![WhiteListParams {
            denom: denom.clone(),
            data: vec![CountryCodeData {
                address: to_address.clone(),
                country_code: UpdateKind::Unset {},
            }],
        }],
    };

    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = QueryMsg::GetCountryCodeByAddress {
        denom,
        address: to_address,
    };
    let err = query(deps.as_ref(), env, msg).unwrap_err();
    assert_eq!(err, StdError::NotFound { kind: "u8".into() });
}

#[test]
fn freezelist() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    do_add_issuer(
        &mut deps,
        env.clone(),
        info.clone(),
        Addr::unchecked("sender"),
    );

    let freezing_address = Addr::unchecked("blacklist");
    let denom = String::from("budz");

    // Create freezing
    let msg = ExecuteMsg::Freeze {
        denom: denom.clone(),
        update_type: UpdateType::Add(vec![freezing_address.clone()]),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = QueryMsg::GetFreezedAccounts {
        denom: denom.clone(),
    };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let accounts: Option<Vec<Addr>> = from_binary(&res).unwrap();

    assert!(accounts
        .expect("expected an address")
        .contains(&freezing_address));

    // Removing freezing
    let msg = ExecuteMsg::Freeze {
        denom: denom.clone(),
        update_type: UpdateType::Remove(vec![freezing_address.clone()]),
    };
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = QueryMsg::GetFreezedAccounts { denom };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let accounts: Option<Vec<Addr>> = from_binary(&res).unwrap();

    assert!(!accounts
        .expect("expected an address")
        .contains(&freezing_address));
}

#[test]
fn withdraw_coins() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);
    let contract_address = env.contract.address.to_string();
    let denom = String::from("budz");

    let expected_msg: Binary = MsgWithdrawRequest {
        denom: denom.to_string(),
        administrator: contract_address.clone(),
        to_address: contract_address,
        amount: vec![Coin {
            denom: denom.to_string(),
            amount: "20".to_string(),
        }],
    }
    .try_into()
    .unwrap();

    // Create marker
    do_create_marker(&mut deps, env.clone(), info.clone(), denom.to_string()).unwrap();

    // Create a withdraw execute message
    let msg = ExecuteMsg::Withdraw {
        denom: denom.to_string(),
        amount: Uint128::new(20),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match &res.messages[0].msg {
        CosmosMsg::Stargate { type_url, value } => {
            assert_eq!(type_url, "/provenance.marker.v1.MsgWithdrawRequest");
            assert_eq!(value, &expected_msg)
        }
        _ => panic!("unexpected cosmos message"),
    }
}

#[test]
fn mint_coins() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);
    let denom = String::from("budz");

    let expected_msg: Binary = MsgMintRequest {
        amount: Some(Coin {
            denom: denom.to_string(),
            amount: "20".to_string(),
        }),
        administrator: env.contract.address.to_string(),
    }
    .try_into()
    .unwrap();

    // Create marker
    do_create_marker(&mut deps, env.clone(), info.clone(), denom.to_string()).unwrap();

    // Create a mint coins marker handler message
    let msg = ExecuteMsg::Mint {
        amount: Uint128::new(20),
        denom: denom.into(),
    };

    // Call handle and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match &res.messages[0].msg {
        CosmosMsg::Stargate { type_url, value } => {
            assert_eq!(type_url, "/provenance.marker.v1.MsgMintRequest");
            assert_eq!(value, &expected_msg)
        }
        _ => panic!("unexpected cosmos message"),
    }
}

#[test]
fn burn_coins() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);
    let denom = String::from("budz");

    let expected_msg: Binary = MsgBurnRequest {
        amount: Some(Coin {
            denom: denom.to_string(),
            amount: "20".to_string(),
        }),
        administrator: env.contract.address.to_string(),
    }
    .try_into()
    .unwrap();

    // Create marker
    do_create_marker(&mut deps, env.clone(), info.clone(), denom.to_string()).unwrap();

    // Create a burn coins marker handler message
    let msg = ExecuteMsg::Burn {
        amount: Uint128::new(20),
        denom: denom.into(),
    };

    // Call handle and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match &res.messages[0].msg {
        CosmosMsg::Stargate { type_url, value } => {
            assert_eq!(type_url, "/provenance.marker.v1.MsgBurnRequest");
            assert_eq!(value, &expected_msg)
        }
        _ => panic!("unexpected cosmos message"),
    }
}

#[test]
fn cancel_marker() {
    // Create default provenance mocks.
    let denom = String::from("budz");
    let sender = Addr::unchecked("sender");
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info(sender.as_str(), &[]);

    let expected_msg: Binary = MsgCancelRequest {
        denom: denom.to_string(),
        administrator: env.contract.address.to_string(),
    }
    .try_into()
    .unwrap();

    // Create a cancel marker handler message
    let msg = ExecuteMsg::Cancel {
        denom: denom.clone(),
    };

    // Fail due to no access rights
    let err = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::NoDeleteAccess {
            address: sender.clone()
        }
    );

    do_add_sub_admin(&mut deps, env.clone(), info.clone());
    let role_msg = ExecuteMsg::ManageRoles {
        denom,
        roles: vec![Role::Agent {
            update_type: UpdateType::Add(vec![sender]),
            marker_access: vec![AccessControls::Delete],
        }],
    };
    let _ = execute(deps.as_mut(), env.clone(), info.clone(), role_msg).unwrap();

    // Call handle and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match &res.messages[0].msg {
        CosmosMsg::Stargate { type_url, value } => {
            assert_eq!(type_url, "/provenance.marker.v1.MsgCancelRequest");
            assert_eq!(value, &expected_msg)
        }
        _ => panic!("unexpected cosmos message"),
    }
}

#[test]
fn destroy_marker() {
    // Create default provenance mocks.
    let denom = String::from("budz");
    let sender = Addr::unchecked("sender");
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info(sender.as_str(), &[]);

    let expected_msg: Binary = MsgDeleteRequest {
        denom: denom.to_string(),
        administrator: env.contract.address.to_string(),
    }
    .try_into()
    .unwrap();

    // Create a destroy marker handler message
    let msg = ExecuteMsg::Destroy {
        denom: denom.to_string(),
    };

    // Fail due to no access rights
    let err = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::NoDeleteAccess {
            address: sender.clone()
        }
    );

    do_add_sub_admin(&mut deps, env.clone(), info.clone());
    let role_msg = ExecuteMsg::ManageRoles {
        denom,
        roles: vec![Role::Agent {
            update_type: UpdateType::Add(vec![sender]),
            marker_access: vec![AccessControls::Delete],
        }],
    };
    let _ = execute(deps.as_mut(), env.clone(), info.clone(), role_msg).unwrap();

    // Call handle and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match &res.messages[0].msg {
        CosmosMsg::Stargate { type_url, value } => {
            assert_eq!(type_url, "/provenance.marker.v1.MsgDeleteRequest");
            assert_eq!(value, &expected_msg)
        }
        _ => panic!("unexpected cosmos message"),
    }
}

#[test]
fn transfer_coins() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);
    let to_address = Addr::unchecked("toaddress");
    let denom = String::from("budz");
    let amount = Uint128::new(20);

    let expected_msg: Binary = MsgTransferRequest {
        amount: Some(Coin {
            denom: denom.to_string(),
            amount: "20".to_string(),
        }),
        administrator: env.contract.address.to_string(),
        from_address: env.contract.address.to_string(),
        to_address: "toaddress".to_string(),
    }
    .try_into()
    .unwrap();

    // Create marker
    do_create_marker(&mut deps, env.clone(), info.clone(), denom.to_string()).unwrap();

    // Create whitelisting
    do_whitelist(&mut deps, env.clone(), info.clone(), to_address.clone()).unwrap();

    // Create a transfer execute message
    let msg = ExecuteMsg::Transfer {
        amount,
        denom: denom.clone(),
        to: "toaddress".into(),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Assert the correct params were created
    match &res.messages[0].msg {
        CosmosMsg::Stargate { type_url, value } => {
            assert_eq!(type_url, "/provenance.marker.v1.MsgTransferRequest");
            assert_eq!(value, &expected_msg)
        }
        _ => panic!("unexpected cosmos message"),
    }

    // Query for balance
    let expected_coin = Coin {
        denom: String::from("denom"),
        amount: 0.to_string(),
    };
    let msg = QueryMsg::GetBalance {
        denom: "denom".into(),
        address: to_address.clone(),
    };
    let res = query(deps.as_ref(), env, msg).unwrap();
    let coin: Coin = from_binary(&res).unwrap();
    assert_eq!(coin, expected_coin);
}

#[test]
fn update_frozen_balance() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);
    let denom = String::from("budz");

    // Create marker
    do_create_marker(&mut deps, env.clone(), info.clone(), denom.to_string()).unwrap();

    let frozen_balance = Uint128::from(500u128);
    let address = Addr::unchecked("some_address");
    // Create a update frozen balance execute message for set
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Agent {
            update_type: UpdateType::Add(vec![Addr::unchecked("sender")]),
            marker_access: vec![AccessControls::Freeze],
        }],
    };

    // Call execute and ensure a cosmos message was dispatched
    let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // Create a update frozen balance execute message for set
    let msg = ExecuteMsg::PartialFreeze {
        denom: denom.clone(),
        params: vec![PartialFreezeParams {
            update_type: UpdateType::Add(frozen_balance),
            address: address.clone(),
        }],
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = QueryMsg::GetFrozenBalance {
        denom: denom.clone(),
        address: address.clone(),
    };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let balance: Option<Uint128> = from_binary(&res).unwrap();

    assert!(balance
        .expect("expected frozen_balance")
        .eq(&frozen_balance));

    // Create a update frozen balance execute message for unset
    let msg = ExecuteMsg::PartialFreeze {
        denom: denom.clone(),
        params: vec![PartialFreezeParams {
            update_type: UpdateType::Remove(frozen_balance),
            address: address.clone(),
        }],
    };

    let sender = Addr::unchecked("sender");
    do_add_issuer(&mut deps, env.clone(), info.clone(), sender);

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = QueryMsg::GetFrozenBalance { denom, address };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let balance: Option<Uint128> = from_binary(&res).unwrap();

    assert!(balance
        .expect("expected frozen_balance")
        .eq(&Uint128::zero()));
}

#[test]
fn update_token_limit() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);
    let denom = String::from("budz");
    let limit = Uint128::from(500u128);

    // Create marker
    do_create_marker(&mut deps, env.clone(), info.clone(), denom.to_string()).unwrap();

    // Create a update token limit execute message
    let msg = ExecuteMsg::UpdateTokenLimit {
        denom: denom.clone(),
        limit,
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = QueryMsg::GetDenomConfig { denom };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let denom_config: DenomConfig = from_binary(&res).unwrap();

    assert!(denom_config.token_limit.eq(&limit));
}

#[test]
fn update_country_code() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);
    let denom = String::from("budz");
    let mut country_code = 91;

    // Create marker
    do_create_marker(&mut deps, env.clone(), info.clone(), denom.to_string()).unwrap();

    // Try to add existing country code again
    let msg = ExecuteMsg::UpdateCountryCode {
        denom: denom.clone(),
        update_type: UpdateType::Add(country_code),
    };

    // Call execute and ensure a cosmos message was dispatched
    let err = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::CountryCodeAlreadyExists { code: country_code }
    );

    // Try to add new country code
    country_code = 61;
    let msg = ExecuteMsg::UpdateCountryCode {
        denom: denom.clone(),
        update_type: UpdateType::Add(country_code),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = QueryMsg::GetDenomConfig {
        denom: denom.clone(),
    };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let denom_config: DenomConfig = from_binary(&res).unwrap();

    assert!(denom_config.country_codes.contains(&country_code));

    // Try to remove recent country code added
    country_code = 61;
    let msg = ExecuteMsg::UpdateCountryCode {
        denom: denom.clone(),
        update_type: UpdateType::Remove(country_code),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    let msg = QueryMsg::GetDenomConfig {
        denom: denom.clone(),
    };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let denom_config: DenomConfig = from_binary(&res).unwrap();

    assert!(!denom_config.country_codes.contains(&country_code));

    // Try to remove non-existing country code
    let msg = ExecuteMsg::UpdateCountryCode {
        denom: denom.clone(),
        update_type: UpdateType::Remove(country_code),
    };

    // Call execute and ensure a cosmos message was dispatched
    let err = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::CountryCodeNotExists { code: country_code }
    );
}

#[test]
fn mint_to() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let mut info = mock_info(sender.as_str(), &[]);
    let denom = String::from("budz");
    let amount = Uint128::new(500);

    // Create marker
    do_create_marker(&mut deps, env.clone(), info.clone(), denom.to_string()).unwrap();

    // Whitelist address
    do_whitelist(&mut deps, env.clone(), info.clone(), sender.clone()).unwrap();

    // Try mintTo
    let msg = ExecuteMsg::MintTo {
        mint_to_params: vec![MintBurnParams {
            denom: denom.clone(),
            mint_burn_data: vec![MintBurnData {
                address: sender.clone(),
                amount,
            }],
        }],
    };

    // Fail due to no access rights
    info = mock_info("random", &[]);
    let err = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::Unauthorized {
            err: "Address `random`: Don't have Issuer, Tokenization, Sub Admin or Mint rights!"
                .into()
        }
    );

    // Providing Issuer access
    info = mock_info(sender.as_str(), &[]);
    do_add_issuer(&mut deps, env, info.clone(), sender.clone());
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(1, res.messages.len());
}

#[test]
fn burn_from() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("sender");
    let mut info = mock_info(sender.as_str(), &[]);
    let denom = String::from("budz");
    let amount = Uint128::new(500);

    // Create marker
    do_create_marker(&mut deps, env.clone(), info.clone(), denom.to_string()).unwrap();

    // Whitelist address
    do_whitelist(&mut deps, env.clone(), info.clone(), sender.clone()).unwrap();

    // Try burnFrom
    let msg = ExecuteMsg::BurnFrom {
        burn_from_params: vec![MintBurnParams {
            denom: denom.clone(),
            mint_burn_data: vec![MintBurnData {
                address: sender.clone(),
                amount,
            }],
        }],
    };

    // Fail due to no access rights
    info = mock_info("random", &[]);
    let err = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::Unauthorized {
            err: "Address `random`: Don't have Issuer, Tokenization, Sub Admin or Burn rights!"
                .into()
        }
    );

    // Providing Issuer access
    info = mock_info(sender.as_str(), &[]);
    do_add_issuer(&mut deps, env, info.clone(), sender.clone());
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(2, res.messages.len());
}

#[test]
fn send() {
    // Create default provenance mocks.
    let denom = String::from("budz");
    let sender = Addr::unchecked("sender");
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info(sender.as_str(), &[]);

    // Create marker
    do_create_marker(&mut deps, env.clone(), info.clone(), denom.to_string()).unwrap();

    let to_address = Addr::unchecked("toaddress");
    let receiver = Addr::unchecked("receiver");
    let amount = Uint128::new(500);

    // Try send
    let send_msg = ExecuteMsg::Send {
        amount,
        denom: denom.clone(),
        to: receiver.clone(),
    };

    // Fail due to no access rights
    let err = execute(deps.as_mut(), mock_env(), info.clone(), send_msg.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::NoTransferAccess {
            address: sender.clone()
        }
    );

    do_add_sub_admin(&mut deps, env.clone(), info.clone());
    let role_msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Agent {
            update_type: UpdateType::Add(vec![sender.clone()]),
            marker_access: vec![AccessControls::Transfer],
        }],
    };
    let _ = execute(deps.as_mut(), env.clone(), info.clone(), role_msg).unwrap();

    // Failed as `to_address` is not whitelisted
    let err = execute(deps.as_mut(), mock_env(), info.clone(), send_msg.clone()).unwrap_err();
    assert_eq!(
        err,
        ContractError::CountryCodeAuthorizationFailed {
            denom: denom.clone(),
            address: receiver.clone()
        }
    );

    // Create whitelisting
    do_whitelist(&mut deps, env.clone(), info.clone(), to_address.clone()).unwrap();
    do_whitelist(&mut deps, env.clone(), info.clone(), receiver.clone()).unwrap();

    // Create a withdraw execute message
    let withdraw_msg = ExecuteMsg::Withdraw {
        amount: Uint128::new(800),
        denom: denom.clone(),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info.clone(), withdraw_msg).unwrap();
    assert_eq!(1, res.messages.len());

    // Create a transfer execute message
    let transfer_msg = ExecuteMsg::Transfer {
        amount: Uint128::new(500),
        denom: denom.clone(),
        to: to_address.into(),
    };

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info, transfer_msg).unwrap();
    assert_eq!(1, res.messages.len());

    let info = mock_info("sender", &[]);

    // Call execute and ensure a cosmos message was dispatched
    let res = execute(deps.as_mut(), mock_env(), info, send_msg).unwrap();
    assert_eq!(1, res.messages.len());
}

#[test]
fn query_marker() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let denom = String::from("budz");

    // Create a mock querier with our expected marker.
    let expected_marker = MarkerAccount {
        base_account: Some(BaseAccount {
            address: "tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u".to_string(),
            pub_key: None,
            account_number: 10,
            sequence: 0,
        }),
        manager: env.contract.address.to_string(),
        access_control: vec![AccessGrant {
            address: "tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz".to_string(),
            permissions: vec![1, 2, 3, 4, 5, 6, 7],
        }],
        status: MarkerStatus::Active.into(),
        denom: "nugz".to_string(),
        supply: "420".to_string(),
        marker_type: 0,
        supply_fixed: false,
        allow_governance_control: false,
        allow_forced_transfer: false,
        required_attributes: vec![],
    };

    let mock_marker_response = QueryMarkerResponse {
        marker: Some(Any {
            type_url: "/provenance.marker.v1.MarkerAccount".to_string(),
            value: expected_marker.encode_to_vec(),
        }),
    };

    QueryMarkerRequest::mock_response(&mut deps.querier, mock_marker_response);

    QueryEscrowRequest::mock_response(
        &mut deps.querier,
        QueryEscrowResponse {
            escrow: vec![Coin {
                denom: denom.to_string(),
                amount: "20".to_string(),
            }],
        },
    );

    // Query and ensure we got the expected marker
    let req = QueryMsg::GetByDenom {
        denom: "nugz".into(),
    };

    let bin = query(deps.as_ref(), mock_env(), req).unwrap();

    let marker: Marker = from_binary(&bin).unwrap();
    assert_eq!(marker.marker_account, expected_marker);
    assert_eq!(
        marker.marker_account.base_account.unwrap().address,
        "tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u"
    )
}

#[test]
fn test_try_add_subadmin() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty sub_admin address
    let init_msg = InitMsg {};
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a sub_admin address
    let addr = Addr::unchecked("sub_admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the sub_admin address was set correctly in the contract state
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert!(sub_admin.contains(&addr));
}

#[test]
fn test_try_remove_subadmin() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty sub_admin address
    let init_msg = InitMsg {};
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a sub_admin address
    let addr = Addr::unchecked("sub_admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the sub_admin address was set correctly in the contract state
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert!(sub_admin.contains(&addr));
    assert_eq!(sub_admin.len(), 1);

    // try sub_admin again
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(1, res.attributes.len());
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert_eq!(sub_admin.len(), 1);

    // remove sub_admin
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Remove(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(res.messages.len(), 0);

    // verify sub_admin was removed
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert_eq!(sub_admin.len(), 0);

    // try to remove sub_admin again
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Remove(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(1, res.attributes.len());
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert_eq!(sub_admin.len(), 0);
}

#[test]
fn test_try_add_issuer() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty sub_admin address
    let init_msg = InitMsg {};
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a sub_admin address
    let addr = Addr::unchecked("sub_admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(1, res.attributes.len());

    // Check that the sub_admin address was set correctly in the contract state
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert!(sub_admin.contains(&addr));
    let info = mock_info("sub_admin", &[coin(100, "token")]);

    // Set up issuer access holder
    let issuer_addr = Addr::unchecked("issuer");
    let denom = "token".to_string();
    let key = Key::new(denom.clone(), issuer_addr.clone())
        .as_bytes()
        .unwrap();
    ISSUER
        .save(deps.as_mut().storage, &key, &Vec::new())
        .unwrap();

    // Test successful add case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Issuer {
            update_type: UpdateType::Add(issuer_addr.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::AlreadyAdded {
            addr: issuer_addr.clone()
        }
    );

    let access_to_agents = ISSUER.load(deps.as_ref().storage, &key).unwrap();
    assert_eq!(access_to_agents.len(), 0);

    // Test unauthorized access case
    let info = mock_info("random_address", &[coin(100, "token")]);
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Issuer {
            update_type: UpdateType::Add(issuer_addr.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender
        }
    );
}

#[test]
fn test_try_remove_issuer() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty sub_admin address
    let init_msg = InitMsg {};
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a sub_admin address
    let addr = Addr::unchecked("sub_admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the sub_admin address was set correctly in the contract state
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert!(sub_admin.contains(&addr));
    let info = mock_info("sub_admin", &[coin(100, "token")]);

    // Set up issuer access holder
    let issuer_addr = Addr::unchecked("issuer");
    let denom = "token".to_string();
    let key = Key::new(denom.clone(), issuer_addr.clone())
        .as_bytes()
        .unwrap();

    // Test successful add case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Issuer {
            update_type: UpdateType::Add(issuer_addr.clone()),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attribute("action", "provwasm.contracts.interop_multisig.add_issuer")
    );

    let access_to_agents = ISSUER.load(deps.as_ref().storage, &key).unwrap();
    assert_eq!(access_to_agents.len(), 5);

    // Test successful remove case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Issuer {
            update_type: UpdateType::Remove(issuer_addr.clone()),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attribute("action", "provwasm.contracts.interop_multisig.remove_issuer")
    );

    let err = ISSUER.load(deps.as_ref().storage, &key).unwrap_err();
    assert_eq!(
        err,
        StdError::not_found("alloc::vec::Vec<interop_multisig::enums::AccessControls>")
    );

    // Test already removed case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Issuer {
            update_type: UpdateType::Remove(issuer_addr.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotFound {
            addr: issuer_addr.clone()
        }
    );

    // Test unauthorized access case
    let info = mock_info("random_address", &[coin(100, "token")]);
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Issuer {
            update_type: UpdateType::Add(issuer_addr.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender.clone()
        }
    );
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Issuer {
            update_type: UpdateType::Remove(issuer_addr.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender.clone()
        }
    );
}

#[test]
fn test_try_add_transfer_agent() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty sub_admin address
    let init_msg = InitMsg {};
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a sub_admin address
    let addr = Addr::unchecked("sub_admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the sub_admin address was set correctly in the contract state
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert!(sub_admin.contains(&addr));
    let info = mock_info("sub_admin", &[coin(100, "token")]);

    // Set up issuer access holder
    let transfer_agent_addr = Addr::unchecked("issuer");
    let denom = "token".to_string();
    let key = Key::new(denom.clone(), transfer_agent_addr.clone())
        .as_bytes()
        .unwrap();

    // Test successful add case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TransferAgent {
            update_type: UpdateType::Add(transfer_agent_addr.clone()),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attribute(
            "action",
            "provwasm.contracts.interop_multisig.add_transfer_agent"
        )
    );

    let access_to_agents = TRANSFER_AGENT.load(deps.as_ref().storage, &key).unwrap();
    assert_eq!(access_to_agents.len(), 3);

    // Test unauthorized access case
    let info = mock_info("random_address", &[coin(100, "token")]);
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TransferAgent {
            update_type: UpdateType::Add(transfer_agent_addr.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender
        }
    );
}

#[test]
fn test_try_remove_transfer_agent() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty sub_admin address
    let init_msg = InitMsg {};
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a sub_admin address
    let addr = Addr::unchecked("sub_admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the sub_admin address was set correctly in the contract state
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert!(sub_admin.contains(&addr));
    let info = mock_info("sub_admin", &[coin(100, "token")]);

    // Set up issuer access holder
    let transfer_agent_addr = Addr::unchecked("issuer");
    let denom = "token".to_string();
    let key = Key::new(denom.clone(), transfer_agent_addr.clone())
        .as_bytes()
        .unwrap();

    // Test successful add case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TransferAgent {
            update_type: UpdateType::Add(transfer_agent_addr.clone()),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attribute(
            "action",
            "provwasm.contracts.interop_multisig.add_transfer_agent"
        )
    );

    let access_to_agents = TRANSFER_AGENT.load(deps.as_ref().storage, &key).unwrap();
    assert_eq!(access_to_agents.len(), 3);

    // Test successful remove case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TransferAgent {
            update_type: UpdateType::Remove(transfer_agent_addr.clone()),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attribute(
            "action",
            "provwasm.contracts.interop_multisig.remove_transfer_agent"
        )
    );

    let err = TRANSFER_AGENT
        .load(deps.as_ref().storage, &key)
        .unwrap_err();
    assert_eq!(
        err,
        StdError::not_found("alloc::vec::Vec<interop_multisig::enums::AccessControls>")
    );

    // Test already removed case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TransferAgent {
            update_type: UpdateType::Remove(transfer_agent_addr.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotFound {
            addr: transfer_agent_addr.clone()
        }
    );

    // Test unauthorized access case
    let info = mock_info("random_address", &[coin(100, "token")]);
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TransferAgent {
            update_type: UpdateType::Add(transfer_agent_addr.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender.clone()
        }
    );
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TransferAgent {
            update_type: UpdateType::Remove(transfer_agent_addr.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender.clone()
        }
    );
}

#[test]
fn test_try_add_tokenization_agent() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty sub_admin address
    let init_msg = InitMsg {};
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a sub_admin address
    let addr = Addr::unchecked("sub_admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the sub_admin address was set correctly in the contract state
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert!(sub_admin.contains(&addr));
    let info = mock_info("sub_admin", &[coin(100, "token")]);

    // Set up issuer access holder
    let tokenization_agent = Addr::unchecked("issuer");
    let denom = "token".to_string();
    let key = Key::new(denom.clone(), tokenization_agent.clone())
        .as_bytes()
        .unwrap();

    // Test successful add case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TokenizationAgent {
            update_type: UpdateType::Add(tokenization_agent.clone()),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attribute(
            "action",
            "provwasm.contracts.interop_multisig.add_tokenization_agent"
        )
    );

    let access_to_agents = TOKENIZATION_AGENT
        .load(deps.as_ref().storage, &key)
        .unwrap();
    assert_eq!(access_to_agents.len(), 2);

    // Test unauthorized access case
    let info = mock_info("random_address", &[coin(100, "token")]);
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TokenizationAgent {
            update_type: UpdateType::Add(tokenization_agent.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender
        }
    );
}

#[test]
fn test_try_remove_tokenization_agent() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty sub_admin address
    let init_msg = InitMsg {};
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a sub_admin address
    let addr = Addr::unchecked("sub_admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the sub_admin address was set correctly in the contract state
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert!(sub_admin.contains(&addr));
    let info = mock_info("sub_admin", &[coin(100, "token")]);

    // Set up issuer access holder
    let tokenization_agent = Addr::unchecked("issuer");
    let denom = "token".to_string();
    let key = Key::new(denom.clone(), tokenization_agent.clone())
        .as_bytes()
        .unwrap();

    // Test successful add case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TokenizationAgent {
            update_type: UpdateType::Add(tokenization_agent.clone()),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attribute(
            "action",
            "provwasm.contracts.interop_multisig.add_tokenization_agent"
        )
    );

    let access_to_agents = TOKENIZATION_AGENT
        .load(deps.as_ref().storage, &key)
        .unwrap();
    assert_eq!(access_to_agents.len(), 2);

    // Test successful remove case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TokenizationAgent {
            update_type: UpdateType::Remove(tokenization_agent.clone()),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attribute(
            "action",
            "provwasm.contracts.interop_multisig.remove_tokenization_agent"
        )
    );

    let err = TOKENIZATION_AGENT
        .load(deps.as_ref().storage, &key)
        .unwrap_err();
    assert_eq!(
        err,
        StdError::not_found("alloc::vec::Vec<interop_multisig::enums::AccessControls>")
    );

    // Test already removed case
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TokenizationAgent {
            update_type: UpdateType::Remove(tokenization_agent.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotFound {
            addr: tokenization_agent.clone()
        }
    );

    // Test unauthorized access case
    let info = mock_info("random_address", &[coin(100, "token")]);
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TokenizationAgent {
            update_type: UpdateType::Add(tokenization_agent.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender.clone()
        }
    );
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::TokenizationAgent {
            update_type: UpdateType::Remove(tokenization_agent.clone()),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender.clone()
        }
    );
}

#[test]
fn test_grant_access_to_agents() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty sub_admin address
    let init_msg = InitMsg {};
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a sub_admin address
    let addr = Addr::unchecked("sub_admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the sub_admin address was set correctly in the contract state
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert!(sub_admin.contains(&addr));
    let info = mock_info("sub_admin", &[coin(100, "token")]);

    // setting mint access to admin and sub_admin
    let denom = "hotdogcoin".to_string();
    let access_info = vec![Addr::unchecked("admin"), Addr::unchecked("sub_admin")];
    let key = Key::new(denom.clone(), AccessControls::Mint)
        .as_bytes()
        .unwrap();
    AGENTS
        .save(deps.as_mut().storage, &key, &access_info)
        .unwrap();

    // grant mint access to agent
    let addr = Addr::unchecked("agent1");
    let marker_access = vec![AccessControls::Mint];
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Agent {
            update_type: UpdateType::Add(vec![addr.clone()]),
            marker_access: marker_access.clone(),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attribute("action", "provwasm.contracts.interop_multisig.grant_access")
    );

    let mint_access = AGENTS.load(deps.as_ref().storage, &key).unwrap();
    assert_eq!(mint_access.len(), 3);

    // Test unauthorized access case
    let info = mock_info("random_address", &[coin(100, "token")]);
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Agent {
            update_type: UpdateType::Add(vec![addr.clone()]),
            marker_access: marker_access.clone(),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender.clone()
        }
    );
}

#[test]
fn test_ungrant_access_to_agent() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty sub_admin address
    let init_msg = InitMsg {};
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a sub_admin address
    let addr = Addr::unchecked("sub_admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::SubAdmin {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the sub_admin address was set correctly in the contract state
    let sub_admin = SUB_ADMIN.load(deps.as_ref().storage).unwrap();
    assert!(sub_admin.contains(&addr));
    let info = mock_info("sub_admin", &[coin(100, "token")]);

    // setting mint access to admin and sub_admin
    let denom = "hotdogcoin".to_string();
    let access_info = vec![Addr::unchecked("admin"), Addr::unchecked("sub_admin")];
    let key = Key::new(denom.clone(), AccessControls::Mint)
        .as_bytes()
        .unwrap();
    AGENTS
        .save(deps.as_mut().storage, &key, &access_info)
        .unwrap();

    // grant mint access to agent
    let addr = Addr::unchecked("agent1");
    let marker_access = vec![AccessControls::Mint];
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Agent {
            update_type: UpdateType::Add(vec![addr.clone()]),
            marker_access: marker_access.clone(),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attribute("action", "provwasm.contracts.interop_multisig.grant_access")
    );

    let mint_access = AGENTS.load(deps.as_ref().storage, &key).unwrap();
    assert_eq!(mint_access.len(), 3);

    // ungrant mint access to agent
    let addr = Addr::unchecked("agent1");
    let marker_access = vec![AccessControls::Mint];
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Agent {
            update_type: UpdateType::Remove(vec![addr.clone()]),
            marker_access: marker_access.clone(),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res,
        Response::new().add_attribute("action", "provwasm.contracts.interop_multisig.ungrant_access")
    );

    let key = Key::new(denom.clone(), AccessControls::Mint)
        .as_bytes()
        .unwrap();
    let mint_access = AGENTS.load(deps.as_ref().storage, &key).unwrap();
    assert_eq!(mint_access.len(), 2);

    // Test unauthorized access case
    let info = mock_info("random_address", &[coin(100, "token")]);
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Agent {
            update_type: UpdateType::Add(vec![addr.clone()]),
            marker_access: marker_access.clone(),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender.clone()
        }
    );
    let msg = ExecuteMsg::ManageRoles {
        denom: denom.clone(),
        roles: vec![Role::Agent {
            update_type: UpdateType::Remove(vec![addr.clone()]),
            marker_access: marker_access.clone(),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(
        err,
        ContractError::NotSubAdmin {
            address: info.sender.clone()
        }
    );
}

#[test]
fn test_update_admin() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty sub_admin address
    let init_msg = InitMsg {};
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a sub_admin address
    let new_admin = Addr::unchecked("new_admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        denom: String::default(),
        roles: vec![Role::Admin {
            address: new_admin.clone(),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the admin address was set correctly in the contract state
    let msg = QueryMsg::GetAdmin {};
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let admin: Addr = from_binary(&res).unwrap();
    assert_eq!(new_admin, admin);
}
