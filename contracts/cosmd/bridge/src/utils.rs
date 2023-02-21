use cosmwasm_std::{SubMsg, Response, WasmMsg, ReplyOn, to_binary};
use ethaddr::{Address};
use crate::error::ContractError;
use cw20_base::msg::InstantiateMsg;
use cw20::MinterResponse;

// use crate::msg::InstantiateMsg;

/// A `reply` call code ID used for sub-messages.
const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;

pub fn string_to_eth_address(address: &str) -> Result<Address, ContractError> {
    address.parse::<Address>().map_err(|_| ContractError::InvalidEthereumAddress {
        address: String::from(address)
    } )
}

pub fn create_cw20_contract(code_id: u64, msg: InstantiateMsg) -> Result<Response, ContractError> {
    let resp = Response::new();
    let sub_msg: Vec<SubMsg> = vec![SubMsg {
        msg: WasmMsg::Instantiate2 {
            code_id,
            msg: to_binary(&InstantiateMsg {
                name: msg.name,
                symbol: "uLP".to_string(),
                decimals: 6,
                initial_balances: vec![],
                mint: Some(MinterResponse {
                    minter: "".to_string(),
                    cap: None,
                }),
                marketing: None,

            })?,
            funds: vec![],
            admin: None,
            label: String::from("Astroport LP token"),
            salt: [0; 1].into(),
        }
            .into(),
        id: INSTANTIATE_TOKEN_REPLY_ID,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    }];

    Ok(resp.add_submessages(sub_msg))
}