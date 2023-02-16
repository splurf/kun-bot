mod bot;
mod res;

use {
    bot::{Handler, BOT_GROUP},
    res::{Images, MessageLink},
    serenity::{framework::StandardFramework, prelude::GatewayIntents, Client},
    std::env::var,
};

#[tokio::main]
async fn main() -> Result<(), String> {
    let token = var("KUN_BOT_TOKEN").expect("`KUN_BOT_TOKEN` environmental variable not found");

    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("s."))
        .group(&BOT_GROUP);

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .map_err(|e| e.to_string())?;

    {
        let mut data = client.data.write().await;
        data.insert::<Images>(Images::get_image_files()?);
        data.insert::<MessageLink>(Default::default());
    }

    if let Err(e) = client.start_autosharded().await {
        eprintln!("{}", e)
    }
    Ok(())
}
