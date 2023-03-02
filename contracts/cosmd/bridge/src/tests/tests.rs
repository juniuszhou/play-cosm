use cosmwasm_std::{coin, coins, Addr, BankMsg, Coin, CosmosMsg, Uint128};
use cw20::{Balance, Cw20Coin, Cw20CoinVerified};
use cw_controllers::AdminResponse;
use cw_multi_test::{App, ContractWrapper, Executor};
use cw_utils::NativeBalance;

use super::mock::{
    create_cw20_contract, execute_cw20_allowance, query_cw20_allowance, query_cw20_balance, COSMOS,
    ETH_ADDRESS, store_cw20_code,
};
use crate::contract::{execute, instantiate, query};
use crate::msg::{CosmosToken, EthClaim, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::tests::mock::CW20_AMOUNT;
use crate::state::{set_cw20_via_eth_address,};

#[test]
fn test_bridge_claim_new_erc20() {
    let sender = Addr::unchecked("sender");
    let admin = Addr::unchecked("admin");
    let receiver = Addr::unchecked("receiver");
    let init_amount = 100;
    let claim_amount = 10_u128;

    let mut app = App::default();
    let cw20_code_id = store_cw20_code(&mut app);

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
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


    // let cw20_address = create_cw20_contract(&mut app, &contract_addr);
    //
    // let contract_balance = query_cw20_balance(&app, &cw20_address, &contract_addr);

    // assert_eq!(contract_balance.u128(), CW20_AMOUNT);

    // set_cw20_via_eth_address(app.sto)

    let claim_message = ExecuteMsg::BridgeClaim {
        claims: vec![EthClaim {
            claim_hash: "00".to_string(),
            token_address: ETH_ADDRESS.to_string(),
            cosmos_token: None,
            receiver: receiver.to_string(),
            amount: claim_amount,
        }],
    };

    app.execute_contract(
        sender.clone(),
        contract_addr.clone(),
        &claim_message,
        &vec![],
    )
        .unwrap();

    assert_eq!(true, false);
    // let contract_balance = query_cw20_balance(&mut app, &cw20_address, &contract_addr);
    // assert_eq!(contract_balance.u128(), init_amount - claim_amount);
    //
    // let receiver_balance = query_cw20_balance(&app, &cw20_address, &receiver);
    //
    // assert_eq!(receiver_balance.u128(), claim_amount);
}

#[test]
fn test_bridge_claim_cw20() {
    let sender = Addr::unchecked("sender");
    let admin = Addr::unchecked("admin");
    let receiver = Addr::unchecked("receiver");
    let init_amount = 100;
    let claim_amount = 10_u128;

    let mut app = App::default();
    let cw20_code_id = store_cw20_code(&mut app);

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
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

    let cw20_address = create_cw20_contract(&mut app, &contract_addr, cw20_code_id);

    let contract_balance = query_cw20_balance(&app, &cw20_address, &contract_addr);

    assert_eq!(contract_balance.u128(), CW20_AMOUNT);

    let claim_message = ExecuteMsg::BridgeClaim {
        claims: vec![EthClaim {
            claim_hash: "00".to_string(),
            token_address: ETH_ADDRESS.to_string(),
            cosmos_token: Some(CosmosToken::CW20(Cw20CoinVerified {
                address: cw20_address.clone(),
                amount: Uint128::from(claim_amount),
            })),
            receiver: receiver.to_string(),
            amount: 0_u128,
        }],
    };

    app.execute_contract(
        sender.clone(),
        contract_addr.clone(),
        &claim_message,
        &vec![],
    )
    .unwrap();

    let contract_balance = query_cw20_balance(&mut app, &cw20_address, &contract_addr);
    assert_eq!(contract_balance.u128(), init_amount - claim_amount);

    let receiver_balance = query_cw20_balance(&app, &cw20_address, &receiver);

    assert_eq!(receiver_balance.u128(), claim_amount);
}

#[test]
fn test_bridge_claim_native() {
    let sender = Addr::unchecked("sender");
    let admin = Addr::unchecked("admin");
    let receiver = Addr::unchecked("receiver");
    let init_amount = 100;
    let claim_amount = 10_u128;


    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(init_amount, COSMOS))
            .unwrap();
    });
    let cw20_code_id = store_cw20_code(&mut app);

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
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

    let native_transfer = BankMsg::Send {
        to_address: contract_addr.to_string(),
        amount: vec![Coin {
            denom: COSMOS.to_string(),
            amount: Uint128::from(init_amount),
        }],
    };

    app.execute(sender.clone(), CosmosMsg::Bank(native_transfer))
        .unwrap();

    let contract_balance = app
        .wrap()
        .query_balance(contract_addr.clone(), COSMOS)
        .unwrap()
        .amount;
    assert_eq!(contract_balance.u128(), init_amount);

    let claim_message = ExecuteMsg::BridgeClaim {
        claims: vec![EthClaim {
            claim_hash: "00".to_string(),
            token_address: ETH_ADDRESS.to_string(),
            cosmos_token: Some(CosmosToken::Native(Coin {
                denom: COSMOS.to_string(),
                amount: Uint128::from(claim_amount),
            })),
            receiver: receiver.to_string(),
            amount: 0_u128,
        }],
    };

    app.execute_contract(
        sender.clone(),
        contract_addr.clone(),
        &claim_message,
        &vec![],
    )
    .unwrap();

    let contract_balance = app
        .wrap()
        .query_balance(contract_addr.clone(), COSMOS)
        .unwrap()
        .amount;
    assert_eq!(contract_balance.u128(), init_amount - claim_amount);

    let receiver_balance = app
        .wrap()
        .query_balance(receiver.clone(), COSMOS)
        .unwrap()
        .amount;
    assert_eq!(receiver_balance.u128(), claim_amount);
}

#[test]
fn test_burn() {
    let sender = Addr::unchecked("sender");
    let admin = Addr::unchecked("admin");
    let init_amount = 100;
    let lock_amount = 10_u128;
    let eth_receiver = ETH_ADDRESS;

    let mut app = App::default();
    let cw20_code_id = store_cw20_code(&mut app);

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
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

    let cw20_address = create_cw20_contract(&mut app, &sender, cw20_code_id);

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
fn test_lock_cw20() {
    let sender = Addr::unchecked("sender");
    let admin = Addr::unchecked("admin");
    let init_amount = 100;
    let lock_amount = 10_u128;
    let eth_receiver = ETH_ADDRESS;

    let mut app = App::default();
    let cw20_code_id = store_cw20_code(&mut app);

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
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

    let cw20_address = create_cw20_contract(&mut app, &sender, cw20_code_id);

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
    let cw20_code_id = store_cw20_code(&mut app);

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let contract_addr = app
        .instantiate_contract(
            code_id,
            // set admin address as sender
            admin,
            &InstantiateMsg {cw20_code_id},
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
    let cw20_code_id = store_cw20_code(&mut app);
    let address_string = "admin";
    let admin_address = Addr::unchecked(address_string);

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            // set admin address as sender
            admin_address,
            &InstantiateMsg {cw20_code_id},
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
    let cw20_code_id = store_cw20_code(&mut app);

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {cw20_code_id},
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
