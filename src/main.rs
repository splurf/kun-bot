mod bot;
mod cfg;
mod err;
mod res;

use {
    bot::{Handler, BOT_GROUP},
    cfg::{parse_config, ConfigCache},
    err::*,
    res::{Images, MessageLink, Whitelist},
    serenity::{framework::StandardFramework, prelude::GatewayIntents, Client},
    std::env::var,
};

#[tokio::main]
async fn main() -> Result<()> {
    let token = var("KUN_BOT_TOKEN")?;

    let (images, cache, whitelist) = parse_config()?;

    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(cache.prefix()))
        .group(&BOT_GROUP);

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await?;

    {
        let mut data = client.data.write().await;

        data.insert::<Images>(images);
        data.insert::<ConfigCache>(cache);
        data.insert::<Whitelist>(whitelist);
        data.insert::<MessageLink>(Default::default());
    }
    client.start_autosharded().await.map_err(Into::into)
}
