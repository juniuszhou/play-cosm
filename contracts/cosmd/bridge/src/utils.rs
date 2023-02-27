use crate::error::ContractError;
use crate::state::{BRIDGE_TOKEN, get_next_bridge_token_index, update_bridge_token_index,};

use cosmwasm_std::{instantiate2_address, to_binary, Api, Addr, Binary, CodeInfoResponse, Deps, StdError, StdResult, Storage, WasmMsg, DepsMut, HexBinary};
use cw20::MinterResponse;
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use ethaddr::Address;

// use crate::msg::InstantiateMsg;
const BRIDGE_CW20_LABEL: &str = "bridge_cw20_label";
const DERIVE_CW20: &str = "derive_cw20";

pub fn string_to_eth_address(address: &str) -> Result<Address, ContractError> {
    address
        .parse::<Address>()
        .map_err(|_| ContractError::InvalidEthereumAddress {
            address: String::from(address),
        })
}

pub fn is_bridge_token(storage: &dyn Storage, address: &Addr) -> bool {
    BRIDGE_TOKEN.load(storage, address.to_owned()).is_ok()
}

// https://github.com/CosmWasm/cosmwasm/blob/v1.2.1/contracts/virus/src/contract.rs#L71-L78
pub fn create_cw20_contract(storage: &mut dyn Storage, api: &dyn Api, admin: &str, code_id: u64, checksum: HexBinary) -> StdResult<(Addr, WasmMsg)> {
    // let index = 0_i64;

    let index = get_next_bridge_token_index(storage);

    let cw20_path = format!("{DERIVE_CW20}/{index}");

    // let CodeInfoResponse { checksum, .. } = deps.querier.query_wasm_code_info(code_id)?;

    let salt = Binary::from(cw20_path.as_bytes());
    let admin_address = api.addr_canonicalize(admin)?;

    let cw20_address = instantiate2_address(&checksum, &admin_address, &salt)
        .map_err(|_| StdError::generic_err(""))?;
    let cw20_address = api.addr_humanize(&cw20_address)?;

    let message = Cw20InstantiateMsg {
        name: "bridge-token".to_string(),
        symbol: "bridge-token".to_string(),
        decimals: 18,
        initial_balances: vec![],
        mint: Some(MinterResponse {
            minter: admin.to_string(),
            cap: None,
        }),
        marketing: None,
    };
    update_bridge_token_index(storage)?;
    Ok((
        cw20_address,
        WasmMsg::Instantiate2 {
            admin: Some(admin.to_string()),
            code_id,
            label: BRIDGE_CW20_LABEL.to_string(),
            msg: to_binary(&message)?,
            funds: vec![],
            salt,
        },
    ))
}
