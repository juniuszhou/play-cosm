// use cosmwasm_std::{BankMsg};
// use cw_multi_test::{ContractWrapper, Executor, AppResponse};
use crate::message::*;

use crate::contract::{execute, instantiate, query};
use crate::test::COSMOS;

#[cfg(test)]
mod tests {
    // use crate::message::*;
    use cosmwasm_std::{Addr, BankMsg, Coin, Uint128};
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;


    #[test]
    fn bank_transfer() {
        let mut app = App::default();

        // app.execute()

        let admin_address = Addr::unchecked("cosmos1address");

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));


        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {admin: admin_address.to_string()},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let message = BankMsg::Send {
            to_address: addr.to_string(),
            amount: vec![Coin{
                denom: COSMOS.to_string(),
                amount: Uint128::from(100_u32),
            }],
        };

        let result = app.execute(admin_address, message.into());
        assert_eq!(result.unwrap().data, None);

    }
}