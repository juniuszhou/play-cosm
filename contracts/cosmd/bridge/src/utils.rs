use crate::error::ContractError;
use crate::state::BRIDGE_TOKEN;

use cosmwasm_std::{instantiate2_address, to_binary, ReplyOn, Response, SubMsg, WasmMsg, Deps, Env, Addr};
use cw20::{Balance, MinterResponse};
use cw20_base::msg::InstantiateMsg;
use ethaddr::Address;

// use crate::msg::InstantiateMsg;

pub fn string_to_eth_address(address: &str) -> Result<Address, ContractError> {
    address
        .parse::<Address>()
        .map_err(|_| ContractError::InvalidEthereumAddress {
            address: String::from(address),
        })
}

pub fn is_bridge_token(    deps: Deps, address: &Addr) -> bool {
    BRIDGE_TOKEN.load(deps.storage, address.to_owned()).is_ok()
}

