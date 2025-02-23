use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use log::info;

mod commands;
mod services;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    colog::init();
    dotenv();

    let token = dotenvy::var("DISCORD_TOKEN").expect("Missing Discord token");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::misc::sync::sync(),
                commands::beatsaber::beatsaver::bsr()
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    info!("Mafuyu started!");
    client.unwrap().start().await.unwrap();
}