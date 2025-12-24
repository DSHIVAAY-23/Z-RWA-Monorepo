use super::*;

pub type Bytes<'a> = &'a [u8];

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");

pub const VALIDATORS: Item<Vec<Addr>> = Item::new("validators");

pub const VOTES: Map<Bytes, Votes> = Map::new("votes");

pub const THRESHOLD: Item<u8> = Item::new("threshold");

pub const INTEROP_CORE_CONTRACT: &str =
    "tp1hfcpqqxl0e9g6terx5qw0nvqrfty9thequ6c8czc9k7vytyd98ys9pj40a";
