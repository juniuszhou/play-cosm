use cw_controllers::Admin;
use cw_storage_plus::{Item, Map};

use cosmwasm_std::{attr, Addr};

// define state via Admin struct define in controllers package
pub const ADMIN: Admin = Admin::new("admin");

// define state for current contract
pub const SINGLE: Item<String> = Item::new("my_item");
pub const DATA_MAP: Map<Addr, ()> = Map::new("my_map");

pub const DATA_ADDRESS: Item<Addr> = Item::new("my_addr");

pub const BRIDGE_TOKEN: Map<Addr, ()> = Map::new("my_map");
