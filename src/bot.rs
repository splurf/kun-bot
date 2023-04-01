use {
    super::res::{delete_if_linked, link, Images},
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

const BLACKLISTED: [Option<GuildId>; 1] = [Some(GuildId(788543390886264842))];

#[group]
#[commands(w)]
struct Bot;

#[command]
async fn w(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if !BLACKLISTED.contains(&msg.guild_id) {
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
        Ok(())
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
