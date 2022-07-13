mod base;
use base::{Bot, Config};

#[tokio::main]
async fn main() {
    let bot: Bot = Config::load().expect("Failed to load configuration").into();

    if let Err(e) = bot.run().await {
        println!("{:?}", e)
    }
}
