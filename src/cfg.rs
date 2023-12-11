use {
    crate::{
        err::Result,
        keys::{Images, Prefix, Whitelist},
        link::try_into_guild_id,
    },
    clap::Parser,
    serenity::{
        builder::CreateMessage,
        model::prelude::{GuildId, UserId},
    },
    std::{fs::File, io::Read, path::PathBuf},
};

pub const WHITE_CHECK_MARK: char = '\u{2705}';

#[derive(Parser)]
#[command(author, version, about)]
struct KunBot {
    #[arg(short, long, default_value_t = String::from("s."))]
    prefix: String,

    #[arg(short, long, default_value_t = String::from("kun-bot"))]
    title: String,

    #[arg(short, long, default_value = "whitelist.txt")]
    wl_path: PathBuf,

    #[arg(required = true, num_args = 1..)]
    paths: Vec<PathBuf>,

    #[arg(required = true, num_args = 1.., last = true)]
    admins: Vec<UserId>,
}

pub async fn parse_config() -> Result<(Vec<CreateMessage>, Prefix, Vec<UserId>, Whitelist)> {
    let KunBot {
        prefix,
        title,
        wl_path,
        paths,
        admins,
    } = KunBot::parse();

    let whitelist = {
        let mut data = Vec::new();

        if let Ok(mut f) = File::open(&wl_path) {
            let mut buf = String::new();
            f.read_to_string(&mut buf)?;

            data.extend(
                buf.split_ascii_whitespace()
                    .map(try_into_guild_id)
                    .collect::<Result<Vec<GuildId>>>()?,
            );
        }
        Whitelist::new(data, wl_path)
    };

    Ok((
        Images::get_images(&title, paths).await?,
        Prefix::new(prefix),
        admins,
        whitelist,
    ))
}
