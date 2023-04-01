mod bot;
mod cfg;
mod res;

use {
    bot::{Handler, BOT_GROUP},
    cfg::{get_config, ConfigCache},
    res::{Images, MessageLink},
    serenity::{framework::StandardFramework, prelude::GatewayIntents, Client},
    std::env::var,
};

#[tokio::main]
async fn main() -> Result<(), String> {
    let token = var("KUN_BOT_TOKEN").expect("`KUN_BOT_TOKEN` environmental variable not found");
    
    let (images, config) = get_config()?;

    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(config.prefix()))
        .group(&BOT_GROUP);

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .map_err(|e| e.to_string())?;

    {
        let mut data = client.data.write().await;

        data.insert::<Images>(images);
        data.insert::<ConfigCache>(config);
        data.insert::<MessageLink>(Default::default());
    }

    if let Err(e) = client.start_autosharded().await {
        eprintln!("{}", e)
    }
    Ok(())
}
