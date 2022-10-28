mod bot;
use bot::*;

#[tokio::main]
async fn main() {
    let bot = Bot::new().expect("Failed to initialize bot");

    if let Err(e) = bot.run().await {
        println!("{:?}", e)
    }
}
