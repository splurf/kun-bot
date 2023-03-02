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

#[group]
#[commands(w)]
struct Bot;

#[command]
async fn w(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if !args.is_empty() {
        return Err("Arguments were provided".into());
    }
    let response = {
        // read from `ctx.data` then handle sending the image to the recipient's channel
        let data = ctx.data.read().await;
        let entry = data.get::<Images>().ok_or("Images do not exist")?;
        let image = Images::choose(entry).ok_or("`Images` is empty")?;

        msg.channel_id
            .send_message(&ctx.http, |m| image.as_embed(m))
            .await?
    };
    link(ctx, msg.id, response).await
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message_delete(
        &self,
        ctx: Context,
        _: ChannelId,
        deleted_message_id: MessageId,
        _: Option<GuildId>,
    ) {
        //  don't really care about this
        let _ = delete_if_linked(&ctx, &deleted_message_id).await;
    }
}
