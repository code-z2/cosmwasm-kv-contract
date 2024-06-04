use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

pub const STORE: Map<String, String> = Map::new("store");

pub const STORAGE_FEE: Item<Coin> = Item::new("sorage_fee");

pub const OWNER: Item<Addr> = Item::new("owner");
