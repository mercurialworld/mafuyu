use crate::{
    commands::{autocomplete, helpers::mapembed::MapEmbed},
    Context, Error,
};
use beatsaver_api::models::map::Map;
use log::info;
use poise::{
    self,
    serenity_prelude::{
        self as serenity, CreateInteractionResponse, CreateInteractionResponseMessage,
    },
    CreateReply,
};

/// Searches a Beat Saber custom map from BeatSaver.   
#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn bsr(
    ctx: Context<'_>,
    #[description = "The beatmap ID, a link, or a BeatSaver search query."]
    #[autocomplete = "autocomplete::beatsaver::autocomplete_map"]
    query: String,
) -> Result<(), Error> {
    let code = query;

    let map: Map = ctx.data().beatsaver_client.map(&code).await?;
    let mut map_embed: MapEmbed = MapEmbed::new(map);

    let builder: CreateReply = CreateReply::default()
        .embed(map_embed.build_embeds()[0].clone()) // just the metadata
        .components(map_embed.build_embed_components());

    // general metadata message
    let reply = ctx.send(builder).await?;

    // collector for difficulty/metadata selection
    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .message_id(reply.message().await?.id)
        .timeout(std::time::Duration::from_secs(15 * 60))
        .filter(move |mci| mci.data.custom_id == "diffsel")
        .await
    {
        info!(
            "Difficulty request for !bsr {} requested by {}",
            &code,
            ctx.author().name
        );

        let diff_key = match &mci.data.kind {
            serenity::ComponentInteractionDataKind::StringSelect { values } => &values[0],
            _ => panic!("unexpected interaction data kind"),
        };

        map_embed.set_index(diff_key);

        let diff_builder = CreateInteractionResponseMessage::new()
            .embeds(map_embed.build_embeds())
            .components(map_embed.build_embed_components());

        let new_message = CreateInteractionResponse::UpdateMessage(diff_builder);

        mci.create_response(ctx, new_message).await?;
    }

    Ok(())
}
