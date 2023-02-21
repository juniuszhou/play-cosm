use cosmwasm_schema::write_api;
use bridge::msg::{InstantiateMsg, QueryMsg, ExecuteMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
