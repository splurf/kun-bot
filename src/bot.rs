use {
    super::{
        cfg::WHITE_CHECK_MARK,
        res::{
            delete_if_linked,
            imgs::Images,
            keys::{Admins, Whitelist},
            link, try_whitelist_add,
        },
    },
    serenity::{
        async_trait,
        client::Context,
        framework::standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        model::prelude::{ChannelId, GuildId, Message, MessageId},
        prelude::EventHandler,
    },
};

#[group]
#[commands(a, w)]
struct Bot;

#[command]
async fn a(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let is_admin = {
        let data = ctx.data.read().await;
        let admins = data.get::<Admins>().ok_or("Root user is not set")?;
        admins.contains(&msg.author.id)
    };

    if is_admin && try_whitelist_add(ctx, args).await.is_ok() {
        msg.react(&ctx.http, WHITE_CHECK_MARK).await.map(|_| ())
    } else {
        msg.delete(&ctx.http).await
    }
    .map_err(Into::into)
}

#[command]
async fn w(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    //  determine if the server is whitelisted
    let whitelisted = {
        if let Some(id) = msg.guild_id {
            let data = ctx.data.read().await;

            data.get::<Whitelist>().map_or(false, |wl| wl.contains(&id))
        } else {
            false
        }
    };

    if whitelisted {
        if !args.is_empty() {
            return Err("Arguments were provided".into());
        }
        let image = {
            // read from `ctx.data` then handle sending the image to the recipient's channel
            let data = ctx.data.read().await;
            let entry = data.get::<Images>().ok_or("Images do not exist")?;
            Images::choose(entry).ok_or("`Images` is empty")?.clone()
        };
        let response = msg
            .channel_id
            .send_message(&ctx.http, |m| image.as_embed(m))
            .await?;
        link(ctx, msg.id, response.id).await
    } else {
        msg.delete(&ctx.http).await.map_err(Into::into)
    }
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        _: Option<GuildId>,
    ) {
        //  don't really care about this
        _ = delete_if_linked(&ctx, channel_id, &deleted_message_id).await;
    }
}
