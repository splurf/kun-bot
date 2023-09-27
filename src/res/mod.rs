pub mod imgs;
pub mod keys;

use {
    super::{cfg::WHITELIST_PATH, Result},
    keys::{MessageLink, Whitelist},
    serenity::{
        framework::standard::{Args, CommandResult},
        model::prelude::{ChannelId, GuildId, MessageId},
        prelude::Context,
    },
    std::{fs::File, io::Write},
};

pub async fn delete_if_linked(
    ctx: &Context,
    channel_id: ChannelId,
    msg: &MessageId,
) -> CommandResult {
    let link_id = {
        //  delete the message if the message is linked
        let data = ctx.data.read().await;
        let links = data
            .get::<MessageLink>()
            .ok_or("Message link map hasn't been instantiated")?;
        links
            .get(msg)
            .ok_or("Message did not have a link to embed")?
            .clone()
    };
    ctx.http.delete_message(channel_id.0, link_id.0).await?;
    {
        //  remove the message from links if it was able to be deleted
        let mut data = ctx.data.write().await;
        let links = data
            .get_mut::<MessageLink>()
            .ok_or("Message link map hasn't been instantiated")?;
        links.remove(msg);
        links.remove(&link_id);
    }
    Ok(())
}

pub async fn link(ctx: &Context, from: MessageId, to: MessageId) -> CommandResult {
    let mut data = ctx.data.write().await;
    let links = data
        .get_mut::<MessageLink>()
        .ok_or("Message link map hasn't been instantiated")?;

    links.insert(from, to);
    links.insert(to, from);
    Ok(())
}

pub fn try_into_guild_id(s: &str) -> Result<GuildId> {
    GuildId::try_from(s.parse::<u64>()?).map_err(|_| crate::Inner::InvalidArg.into())
}

pub fn update_wl_file(whitelist: &[GuildId]) -> Result<()> {
    let mut f = File::create(WHITELIST_PATH)?;

    f.write_all(
        whitelist
            .iter()
            .map(|id| id.0.to_string())
            .collect::<Vec<String>>()
            .join(" ")
            .as_bytes(),
    )?;
    f.flush().map_err(Into::into)
}

pub async fn try_whitelist_add(ctx: &Context, args: Args) -> CommandResult {
    let id = try_into_guild_id(args.message())?;

    let mut data = ctx.data.write().await;
    let whitelist = data.get_mut::<Whitelist>().ok_or("Whitelist is not set")?;

    if whitelist.contains(&id) {
        Err("Server is already whitelisted".into())
    } else {
        whitelist.push(id);
        update_wl_file(whitelist.as_slice()).map_err(|e| e.to_string().into())
    }
}
