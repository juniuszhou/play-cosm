use cosmwasm_std::{Addr, StdResult, Storage};
use cw_controllers::Admin;
use cw_storage_plus::{Item, Map};

// define state via Admin struct define in controllers package
pub const ADMIN: Admin = Admin::new("admin");

// define state for current contract
pub const BRIDGE_TOKEN: Map<Addr, ()> = Map::new("bridge_token_map");

pub const ETH_ADDRESS_TO_CW20: Map<String, Addr> = Map::new("eth_address_to_cw20");

pub const BRIDGE_TOKEN_INDEX: Item<u64> = Item::new("bridge_token_index");

pub fn add_bridge_token(
    storage: &mut dyn Storage,
    eth_contract: &str,
    address: &Addr,
) -> StdResult<()> {
    BRIDGE_TOKEN.update(storage, address.to_owned(), |_| -> StdResult<_> { Ok(()) })?;

    ETH_ADDRESS_TO_CW20.update(storage, eth_contract.to_string(), |_| -> StdResult<_> {
        Ok(address.to_owned())
    })?;
    Ok(())
}

pub fn get_cw20_via_eth_address(storage: &dyn Storage, eth_address: &str) -> Option<Addr> {
    ETH_ADDRESS_TO_CW20
        .load(storage, eth_address.to_string())
        .ok()
}

pub fn get_next_bridge_token_index(storage: &mut dyn Storage) -> u64 {
    match BRIDGE_TOKEN_INDEX.load(storage) {
        Ok(index) => index + 1,
        Err(_) => 0_u64,
    }
}

pub fn update_bridge_token_index(storage: &mut dyn Storage) -> StdResult<()> {
    let next = get_next_bridge_token_index(storage);
    BRIDGE_TOKEN_INDEX.update(storage, |_| -> StdResult<_> { Ok(next) })?;

    Ok(())
}
