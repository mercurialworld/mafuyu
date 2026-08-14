use axum::{routing::get, Router};
use dotenvy::dotenv;
use poise::serenity_prelude::{self as serenity};

use mafuyu::{
    api::{health, serve},
    core::bot::Mafuyu,
};
use tokio::join;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = dotenvy::var("DISCORD_TOKEN").expect("Missing Discord token");
    let intents = serenity::GatewayIntents::non_privileged();

    let mut mafuyu = Mafuyu::new(&token, intents).await;

    let app: Router = Router::new().route("/health", get(health));

    let _ = join!(serve(app, 5000), mafuyu.client.start());
}
