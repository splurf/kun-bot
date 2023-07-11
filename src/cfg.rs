use {
    crate::{
        err::Result,
        res::{Image, Images},
    },
    clap::Parser,
    serenity::prelude::TypeMapKey,
    std::path::PathBuf,
};

/// A simple Discord bot to provide randomly selected images from specified gallery(s).
#[derive(Parser)]
#[command(author, version, about)]
struct KunBot {
    #[arg(short, long, default_value_t = String::from("s."))]
    prefix: String,

    #[arg(short, long, default_value_t = String::from("kun-bot"))]
    title: String,

    #[arg(required = true, num_args = 1..)]
    paths: Vec<PathBuf>,

    #[arg(required = true, last = true, num_args = 1..)]
    whitelist: Vec<u64>,
}

pub fn parse_config() -> Result<(Vec<Image>, RawConfigCache, Vec<u64>)> {
    let KunBot {
        prefix,
        title,
        paths,
        whitelist,
    } = KunBot::parse();

    Ok((
        Images::get_images(title, paths)?,
        RawConfigCache { prefix },
        whitelist,
    ))
}

pub struct ConfigCache;

impl TypeMapKey for ConfigCache {
    type Value = RawConfigCache;
}

pub struct RawConfigCache {
    prefix: String,
}

impl RawConfigCache {
    pub fn prefix(&self) -> &str {
        self.prefix.as_str()
    }
}
