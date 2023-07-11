mod images;
mod whitelist;

use {
    serenity::{
        framework::standard::CommandResult,
        model::prelude::{ChannelId, MessageId},
        prelude::{Context, TypeMapKey},
    },
    std::collections::HashMap,
};

pub use {images::*, whitelist::*};

pub struct MessageLink;

impl TypeMapKey for MessageLink {
    type Value = HashMap<MessageId, MessageId>;
}

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
