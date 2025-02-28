use poise::{
    self,
    serenity_prelude::{CreateEmbed, CreateEmbedFooter},
    CreateReply,
};

use crate::{Context, Error};

// TODO: actually fucking deploy this thing

/// Shows status of the bot.
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    let status_embed = CreateEmbed::new()
        .title(format!("Mafuyu v{}", env!("CARGO_PKG_VERSION")))
        .description("A general purpose Discord application.")
        .field("Source", "https://github.com/mercurialworld/mafuyu", false)
        .footer(CreateEmbedFooter::new(
            "Made by @mercurial_world on Discord",
        ));

    ctx.send(CreateReply::default().embed(status_embed)).await?;

    Ok(())
}
