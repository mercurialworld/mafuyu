use dotenvy::dotenv;
use poise::serenity_prelude::{self as serenity};

use mafuyu::core::bot::Mafuyu;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = dotenvy::var("DISCORD_TOKEN").expect("Missing Discord token");
    let intents = serenity::GatewayIntents::non_privileged();

    let mut mafuyu = Mafuyu::new(&token, intents).await;

    mafuyu.client.start().await.unwrap();
}
