
use cw_storage_plus::{Item, Map};
use cw_controllers::Admin;

use cosmwasm_std::{
    attr, Addr
};

// define state via Admin struct define in controllers package
pub const ADMIN: Admin = Admin::new("admin");

// define state for current contract
pub const MYITEM: Item<String> = Item::new("my_item");
pub const MYMAP: Map<u64, String> = Map::new("my_map");

pub const MYADDR: Item<Addr> = Item::new("my_addr");
