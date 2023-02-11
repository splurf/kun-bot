use {
    super::res::Images,
    serenity::{
        client::Context,
        framework::standard::{
            macros::{command, group},
            CommandResult,
        },
        model::channel::Message,
    },
};

#[group]
#[commands(w)]
struct Bot;

#[command]
async fn w(ctx: &Context, msg: &Message) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;
    let data = ctx.data.read().await;
    let entry = data.get::<Images>().ok_or("Images do not exist")?;

    let image = Images::choose(entry).ok_or("`Images` is empty")?;

    msg.channel_id
        .send_message(&ctx.http, |m| image.as_embed(m))
        .await?;
    typing.stop().ok_or("Error attempting to stop typing")?;
    Ok(())
}
