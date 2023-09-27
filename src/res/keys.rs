use {
    crate::cfg::RawConfigCache,
    serenity::{
        model::prelude::{GuildId, MessageId, UserId},
        prelude::TypeMapKey,
    },
    std::collections::HashMap,
};

pub struct Whitelist;

impl TypeMapKey for Whitelist {
    type Value = Vec<GuildId>;
}

pub struct Admins;

impl TypeMapKey for Admins {
    type Value = Vec<UserId>;
}

pub struct MessageLink;

impl TypeMapKey for MessageLink {
    type Value = HashMap<MessageId, MessageId>;
}

pub struct ConfigCache;

impl TypeMapKey for ConfigCache {
    type Value = RawConfigCache;
}
