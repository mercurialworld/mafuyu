use beatsaver_api::client::BeatSaverClient;
use log::{debug, info, warn};
use poise::{
    serenity_prelude::{self as serenity, CreateEmbed, GatewayIntents},
    CreateReply,
};
use regex::Regex;

use crate::{commands, Data};

pub struct Mafuyu {
    pub client: serenity::Client,
}

impl Mafuyu {
    pub async fn new(token: &str, intents: GatewayIntents) -> Self {
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
                                author.name,
                                app_ctx.interaction.data.name,
                                app_ctx.interaction.data.options
                            ),
                            poise::Context::Prefix(pfx_ctx) => {
                                info!(
                                    "{} used prefix command {}",
                                    author.name, pfx_ctx.msg.content
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
                                    warn!("because: {cause:?}");
                                    error_description.push_str(&format!("because: {cause}\n"));
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

                    let beatsaver_client = BeatSaverClient::default();
                    let bsr_link_regex = Regex::new(
                        r"(?:https?://)?(?:www\.)?beatsaver\.com/maps/(?P<bsr>[a-fA-F0-9]+)",
                    )
                    .unwrap();
                    let hexstring_regex = Regex::new(r"^[a-fA-F0-9]+$").unwrap();

                    info!("Mafuyu started!");
                    Ok(Data {
                        beatsaver_client,
                        bsr_link_regex,
                        hexstring_regex,
                    })
                })
            })
            .build();

        let client = serenity::ClientBuilder::new(token, intents)
            .framework(framework)
            .await
            .expect("Unable to create framework");

        Self { client }
    }
}
