use beatsaver_api::client::BeatSaverClient;
use dotenvy::dotenv;
use poise::{
    serenity_prelude::{self as serenity, CreateEmbed},
    CreateReply,
};

mod commands;
use log::{debug, info, warn};

struct Data {
    beatsaver_client: BeatSaverClient,
} // User data, which is stored and accessible in all command invocations
type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = dotenvy::var("DISCORD_TOKEN").expect("Missing Discord token");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::beatsaber::beatsaver::bsr(),
                commands::misc::status::status(),
                commands::misc::sync::sync(),
            ],
            pre_command: |ctx| {
                Box::pin(async move {
                    let author = ctx.author();

                    match ctx {
                        poise::Context::Application(app_ctx) => info!(
                            "{} used app command {} with options {:?}",
                            &author.name,
                            &app_ctx.interaction.data.name,
                            &app_ctx.interaction.data.options
                        ),
                        poise::Context::Prefix(pfx_ctx) => {
                            info!(
                                "{} used prefix command {}",
                                &author.name, &pfx_ctx.msg.content
                            )
                        }
                    }
                })
            },
            on_error: |error| {
                Box::pin(async move {
                    warn!("{:?}", error.to_string());

                    match error {
                        poise::FrameworkError::Command { error, ctx, .. } => {
                            let mut error_description: String = "".to_string();

                            error.chain().skip(1).for_each(|cause| {
                                warn!("because: {:?}", cause);
                                error_description.push_str(&format!("because: {}\n", cause));
                            });

                            let embed = CreateEmbed::new()
                                .title(format!("Error: {error}"))
                                .description(error_description);

                            let builder = CreateReply::default().embed(embed).ephemeral(true);
                            let _ = ctx.send(builder).await;
                        }
                        other => poise::builtins::on_error(other).await.unwrap(),
                    }
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                debug!("Setting activity text");
                ctx.set_activity(Some(serenity::ActivityData::custom(format!(
                    "v{}",
                    env!("CARGO_PKG_VERSION")
                ))));

                let bs_client = BeatSaverClient::new();

                info!("Mafuyu started!");
                Ok(Data {
                    beatsaver_client: bs_client,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
