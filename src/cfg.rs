use {
    crate::{
        err::Result,
        res::{
            imgs::{Image, Images},
            try_into_guild_id,
        },
    },
    clap::Parser,
    serenity::model::prelude::{GuildId, UserId},
    std::{fs::File, io::Read, path::PathBuf},
};

pub const WHITELIST_PATH: &'static str = "whitelist.txt";
pub const WHITE_CHECK_MARK: char = '\u{2705}';

#[derive(Parser)]
#[command(author, version, about)]
struct KunBot {
    #[arg(short, long, default_value_t = String::from("s."))]
    prefix: String,

    #[arg(short, long, default_value_t = String::from("kun-bot"))]
    title: String,

    #[arg(required = true, num_args = 1..)]
    paths: Vec<PathBuf>,

    #[arg(required = true, num_args = 1.., last = true)]
    admins: Vec<UserId>,
}

pub fn parse_config() -> Result<(Vec<Image>, RawConfigCache, Vec<UserId>, Vec<GuildId>)> {
    let KunBot {
        prefix,
        title,
        paths,
        admins,
    } = KunBot::parse();

    let mut whitelist = Vec::new();

    if let Ok(mut f) = File::open(WHITELIST_PATH) {
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;

        whitelist.extend(
            buf.split_ascii_whitespace()
                .map(try_into_guild_id)
                .collect::<Result<Vec<GuildId>>>()?,
        );
    }

    Ok((
        Images::get_images(title, paths)?,
        RawConfigCache { prefix },
        admins,
        whitelist,
    ))
}

pub struct RawConfigCache {
    prefix: String,
}

impl RawConfigCache {
    pub fn prefix(&self) -> &str {
        self.prefix.as_str()
    }
}
