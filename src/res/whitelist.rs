use serenity::prelude::TypeMapKey;

pub struct Whitelist;

impl TypeMapKey for Whitelist {
    type Value = Vec<u64>;
}
