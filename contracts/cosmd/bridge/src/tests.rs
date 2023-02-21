


use cosmwasm_std::Addr;
use cw_controllers::AdminResponse;
use cw_multi_test::{App, ContractWrapper, Executor};

use super::*;
use msg::*;

#[test]
fn check_admin() {
    let mut app = App::default();
    let address_string = "admin";
    let admin_address = Addr::unchecked(address_string);

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            // set admin address as sender
            admin_address,
            &InstantiateMsg {},
            &[],
            "Contract",
            // this admin ignored, don't know how to use it.
            None,
        )
        .unwrap();

    let resp: AdminResponse = app
        .wrap()
        .query_wasm_smart(addr, &QueryMsg::QueryAdmin {})
        .unwrap();
    assert_eq!(resp.admin.unwrap(), address_string);
}

#[test]
fn greet_query() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {},
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: GreetResp = app
        .wrap()
        .query_wasm_smart(addr, &QueryMsg::Greet {})
        .unwrap();

    assert_eq!(
        resp,
        GreetResp {
            message: "Hello World".to_owned()
        }
    );
}