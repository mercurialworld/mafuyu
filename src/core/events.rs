use std::{future::Future, pin::Pin};

use log::{info, warn};
use poise::{serenity_prelude::CreateEmbed, CreateReply, FrameworkError};

use crate::{Context, Data, Error};

pub async fn pre_command(ctx: Context<'_>) -> Pin<Box<impl Future<Output = ()> + use<'_>>> {
    Box::pin(async move {
        let author = ctx.author();

        match ctx {
            poise::Context::Application(app_ctx) => info!(
                "{} used app command {} with options {:?}",
                &author.name, &app_ctx.interaction.data.name, &app_ctx.interaction.data.options
            ),
            poise::Context::Prefix(pfx_ctx) => {
                info!(
                    "{} used prefix command {}",
                    &author.name, &pfx_ctx.msg.content
                )
            }
        }
    })
}

pub fn on_error(
    error: FrameworkError<'_, Data, Error>,
) -> Pin<Box<impl Future<Output = ()> + use<'_>>> {
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
}
