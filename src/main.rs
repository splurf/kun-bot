mod bot;
mod res;

use {
    bot::BOT_GROUP,
    res::Images,
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
        .await
        .map_err(|e| e.to_string())?;

    {
        let mut data = client.data.write().await;
        data.insert::<Images>(Images::get_image_files()?);
    }

    if let Err(e) = client.start_autosharded().await {
        eprintln!("{}", e)
    }
    Ok(())
}
