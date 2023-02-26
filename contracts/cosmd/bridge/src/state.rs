use cw_controllers::Admin;
use cw_storage_plus::{Item, Map};

use cosmwasm_std::{attr, Addr};

// define state via Admin struct define in controllers package
pub const ADMIN: Admin = Admin::new("admin");

// define state for current contract
pub const BRIDGE_TOKEN: Map<Addr, ()> = Map::new("bridge_token_map");
