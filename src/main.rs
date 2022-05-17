use kun_bot::{Bot, Config};

#[tokio::main]
async fn main() {
    let bot: Bot = Config::load().expect("Failed to read configuration").into();

    if let Err(e) = bot.run().await {
        println!("{}", e)
    }
}
