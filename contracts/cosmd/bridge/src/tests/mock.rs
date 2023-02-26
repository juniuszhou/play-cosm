// define the test native token
pub const COSMOS: &str = "cosmos";
pub const ETH_ADDRESS: &str = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";
pub const CW20_AMOUNT: u128 = 100;

use cw20::{AllowanceResponse, BalanceResponse, Cw20Coin};
use cw20_base::contract::{execute, instantiate, query};
use cw20_base::msg::{ExecuteMsg as Cw20ExecuteMsg, InstantiateMsg, QueryMsg};

use cosmwasm_std::{testing::MockStorage, to_binary, Addr, Deps, DepsMut, Uint128};

use crate::msg::ExecuteMsg;
use crate::state::BRIDGE_TOKEN;

use cw_multi_test::{App, BasicApp, ContractWrapper, Executor};

// https://github.com/CosmWasm/cosmwasm/blob/v1.2.1/contracts/virus/src/contract.rs#L71-L78
pub fn create_cw20_contract(app: &mut BasicApp, admin: &Addr) -> Addr {
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let init_balance = Cw20Coin {
        address: admin.to_string(),
        amount: Uint128::from(CW20_AMOUNT),
    };

    app.instantiate_contract(
        code_id,
        // set admin address as sender
        admin.to_owned(),
        &InstantiateMsg {
            name: "token".to_owned(),
            symbol: "symbol".to_owned(),
            decimals: 18_u8,
            initial_balances: vec![init_balance],
            mint: None,
            marketing: None,
        },
        &[],
        "Contract",
        // this admin ignored, don't know how to use it.
        None,
    )
    .unwrap()
}

pub fn query_cw20_balance(app: &BasicApp, cw20_address: &Addr, address: &Addr) -> Uint128 {
    let query_message = QueryMsg::Balance {
        address: address.to_string(),
    };
    let response = app.wrap().query_wasm_smart(cw20_address, &query_message);
    let balance: BalanceResponse = response.unwrap();
    balance.balance
}

pub fn query_cw20_allowance(
    app: &BasicApp,
    cw20_address: &Addr,
    owner: &Addr,
    spender: &Addr,
) -> Uint128 {
    let query_message = QueryMsg::Allowance {
        owner: owner.to_string(),
        spender: spender.to_string(),
    };
    let response = app.wrap().query_wasm_smart(cw20_address, &query_message);
    let balance: AllowanceResponse = response.unwrap();
    balance.allowance
}

pub fn execute_cw20_allowance(
    app: &mut BasicApp,
    cw20_address: &Addr,
    owner: &Addr,
    spender: &Addr,
    amount: u128,
) {
    let allowance_message = Cw20ExecuteMsg::IncreaseAllowance {
        spender: spender.to_string(),
        amount: Uint128::from(amount),
        expires: None,
    };

    let result = app.execute_contract(
        owner.to_owned(),
        cw20_address.to_owned(),
        &allowance_message,
        &vec![],
    );

    result.unwrap();
}
