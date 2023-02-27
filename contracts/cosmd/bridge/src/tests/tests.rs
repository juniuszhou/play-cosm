use cosmwasm_std::{coin, coins, Addr, BankMsg, Coin, Uint128};
use cw20::{Balance, Cw20Coin, Cw20CoinVerified};
use cw_controllers::AdminResponse;
use cw_multi_test::{App, ContractWrapper, Executor};
use cw_utils::NativeBalance;

use super::mock::{
    create_cw20_contract, execute_cw20_allowance, query_cw20_allowance, query_cw20_balance, COSMOS,
    ETH_ADDRESS,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::contract::{execute, instantiate, query};

#[test]
fn test_burn() {
    // let sender = Addr::unchecked("sender");
    // let admin = Addr::unchecked("admin");
    // let init_amount = 100;
    // let lock_amount = 10_u128;
    // let eth_receiver = ETH_ADDRESS;
    //
    // let mut app = App::new();
    //
    // let code = ContractWrapper::new(execute, instantiate, query);
    // let code_id = app.store_code(Box::new(code));
    // let contract_addr = app
    //     .instantiate_contract(
    //         code_id,
    //         admin,
    //         &InstantiateMsg {},
    //         &[],
    //         "Contract",
    //         // this admin ignored, don't know how to use it.
    //         None,
    //     )
    //     .unwrap();
    //
    // let cw20_address = create_cw20_contract(&mut app, &sender);
    // add_cw20_contract(&mut app, app.deps, cw20_address);
    //
    // let sender_balance = query_cw20_balance(&mut app, &cw20_address, &sender);
    // assert_eq!(init_amount, sender_balance.u128());
    //
    // let contract_balance = query_cw20_allowance(&mut app, &cw20_address, &sender, &contract_addr);
    // assert_eq!(0, contract_balance.u128());
    //
    // execute_cw20_allowance(
    //     &mut app,
    //     &cw20_address,
    //     &sender,
    //     &contract_addr,
    //     lock_amount,
    // );
    //
    // let contract_balance = query_cw20_allowance(&mut app, &cw20_address, &sender, &contract_addr);
    // assert_eq!(lock_amount, contract_balance.u128());
    //
    // let lock_token = Balance::Cw20(Cw20CoinVerified {
    //     address: cw20_address.clone(),
    //     amount: Uint128::from(lock_amount),
    // });
    //
    // let message = ExecuteMsg::Lock {
    //     balances: vec![lock_token],
    //     receiver: eth_receiver.to_string(),
    // };
    //
    // app.execute_contract(sender.clone(), contract_addr.clone(), &message, &vec![]).unwrap();
    //
    // // check sender's balance after lock
    // let sender_balance = query_cw20_balance(&mut app, &cw20_address, &sender);
    // assert_eq!(init_amount - lock_amount, sender_balance.u128());
    // // check contract's balance after lock
    // let contract_balance = query_cw20_balance(&mut app, &cw20_address, &contract_addr);
    // assert_eq!(lock_amount, contract_balance.u128());
}

#[test]
fn test_lock_cw20() {
    let sender = Addr::unchecked("sender");
    let admin = Addr::unchecked("admin");
    let init_amount = 100;
    let lock_amount = 10_u128;
    let eth_receiver = ETH_ADDRESS;

    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let contract_addr = app
        .instantiate_contract(
            code_id,
            admin,
            &InstantiateMsg {},
            &[],
            "Contract",
            // this admin ignored, don't know how to use it.
            None,
        )
        .unwrap();

    let cw20_address = create_cw20_contract(&mut app, &sender);

    let sender_balance = query_cw20_balance(&mut app, &cw20_address, &sender);
    assert_eq!(init_amount, sender_balance.u128());

    let contract_balance = query_cw20_allowance(&mut app, &cw20_address, &sender, &contract_addr);
    assert_eq!(0, contract_balance.u128());

    execute_cw20_allowance(
        &mut app,
        &cw20_address,
        &sender,
        &contract_addr,
        lock_amount,
    );

    let contract_balance = query_cw20_allowance(&mut app, &cw20_address, &sender, &contract_addr);
    assert_eq!(lock_amount, contract_balance.u128());

    let lock_token = Balance::Cw20(Cw20CoinVerified {
        address: cw20_address.clone(),
        amount: Uint128::from(lock_amount),
    });

    let message = ExecuteMsg::Lock {
        balances: vec![lock_token],
        receiver: eth_receiver.to_string(),
    };

    app.execute_contract(sender.clone(), contract_addr.clone(), &message, &vec![])
        .unwrap();

    // check sender's balance after lock
    let sender_balance = query_cw20_balance(&mut app, &cw20_address, &sender);
    assert_eq!(init_amount - lock_amount, sender_balance.u128());
    // check contract's balance after lock
    let contract_balance = query_cw20_balance(&mut app, &cw20_address, &contract_addr);
    assert_eq!(lock_amount, contract_balance.u128());
}

#[test]
fn test_lock_native() {
    let sender = Addr::unchecked("sender");
    let admin = Addr::unchecked("admin");
    let init_amount = 100;
    let lock_amount = 10;
    let eth_receiver = ETH_ADDRESS;

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(init_amount, COSMOS))
            .unwrap();
    });

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let contract_addr = app
        .instantiate_contract(
            code_id,
            // set admin address as sender
            admin,
            &InstantiateMsg {},
            &[],
            "Contract",
            // this admin ignored, don't know how to use it.
            None,
        )
        .unwrap();

    let lock_token = coins(lock_amount, COSMOS);

    let sender_balance = app
        .wrap()
        .query_balance(sender.clone(), COSMOS)
        .unwrap()
        .amount
        .u128();
    assert_eq!(init_amount, sender_balance);

    let contract_balance = app
        .wrap()
        .query_balance(contract_addr.clone(), COSMOS)
        .unwrap()
        .amount
        .u128();
    assert_eq!(0, contract_balance);

    let message = ExecuteMsg::Lock {
        balances: vec![],
        receiver: eth_receiver.to_string(),
    };

    let _ = app
        .execute_contract(sender.clone(), contract_addr.clone(), &message, &lock_token)
        .unwrap();

    // check balance after lock
    let sender_balance = app
        .wrap()
        .query_balance(sender.clone(), COSMOS)
        .unwrap()
        .amount
        .u128();
    assert_eq!(init_amount - lock_amount, sender_balance);
    let contract_balance = app
        .wrap()
        .query_balance(contract_addr.clone(), COSMOS)
        .unwrap()
        .amount
        .u128();
    assert_eq!(lock_amount, contract_balance);
}

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
fn test_query_admin() {
    let sender = Addr::unchecked("owner");
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {},
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: AdminResponse = app
        .wrap()
        .query_wasm_smart(addr, &QueryMsg::QueryAdmin {})
        .unwrap();

    assert_eq!(
        resp,
        AdminResponse {
            admin: Some(sender.to_string()),
        }
    );
}
