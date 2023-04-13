use cosmwasm_std::Addr;
use cw_controllers::AdminResponse;
use cw_multi_test::{App, ContractWrapper, Executor};

use super::*;
use crate::message::*;
use crate::contract::{instantiate, execute, query};

#[cfg(test)]
mod tests {
    use crate::message::*;
    use cosmwasm_std::{Addr, BankMsg, Response};
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;

    #[test]
    fn greet_query() {
        let mut app = App::default();

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
}