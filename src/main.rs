mod bot;
mod cfg;
mod err;
mod keys;
mod link;

use {
    bot::{Handler, BOT_GROUP},
    cfg::parse_config,
    err::*,
    keys::*,
    serenity::{
        framework::{standard::Configuration, StandardFramework},
        prelude::GatewayIntents,
        Client,
    },
    std::env::var,
};

#[tokio::main]
async fn main() -> Result<()> {
    let token = var("KUN_BOT_TOKEN")?;

    let (images, prefix, admins, whitelist) = parse_config().await?;

    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let framework = StandardFramework::new().group(&BOT_GROUP);
    framework.configure(Configuration::default().prefix(prefix.as_ref()));

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await?;

    println!("Bot is running with prefix [{}]", prefix.as_ref());

    {
        let mut data = client.data.write().await;

        data.insert::<Images>(images);
        data.insert::<Prefix>(prefix);
        data.insert::<Admins>(admins);
        data.insert::<Whitelist>(whitelist);
        data.insert::<MessageLink>(Default::default());
    }
    client.start_autosharded().await.map_err(Into::into)
}
