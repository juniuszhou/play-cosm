use crate::state::BRIDGE_TOKEN;
use crate::state::{add_bridge_token, get_cw20_via_eth_address};
use crate::utils::{is_bridge_token, create_cw20_contract,};

use super::mock::{ETH_ADDRESS, store_cw20_code};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{Addr, CodeInfoResponse};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::contract::{execute, instantiate, query};
use crate::msg::InstantiateMsg;

#[test]
fn test_create_new_erc20() {
    let mut app = App::default();
    let mut deps = mock_dependencies();
    let cw20_code_id = store_cw20_code(&mut app);
    let admin = Addr::unchecked("admin");

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    // create bridge contract
    let contract_addr = app
        .instantiate_contract(
            code_id,
            admin,
            &InstantiateMsg {cw20_code_id},
            &[],
            "Contract",
            // this admin ignored, don't know how to use it.
            None,
        )
        .unwrap();

    // get bridge's checksum
    // let CodeInfoResponse { checksum, .. } =
    //     deps.querier.query_wasm_code_info(code_id)?;
    //
    // // create cw20 contract
    // let (address, message) = create_cw20_contract(
    //     &mut deps.storage,
    //     &deps.api,
    //     // deps.storage,
    //     contract_addr.as_str(),
    //     code_id,
    //     checksum,
    // ).unwrap();
    //
    // assert_eq!(address, admin);
}

// test everything from internal never use the message
#[test]
fn test_add_bridge_token() {
    let eth_address = ETH_ADDRESS;
    let cw20_address = Addr::unchecked("bridge token");

    let mut deps = mock_dependencies();
    // let mut storage = mock_storage()

    // BRIDGE_TOKEN.load(&deps.storage, &cw20_address)

    assert!(!is_bridge_token(&deps.storage, &cw20_address));

    add_bridge_token(&mut deps.storage, &eth_address, &cw20_address).unwrap();

    assert!(is_bridge_token(&deps.storage, &cw20_address));

    let cw20_in_state = get_cw20_via_eth_address(&deps.storage, &eth_address);

    assert_eq!(cw20_in_state, Some(cw20_address));
}
