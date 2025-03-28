use crate::{commands::helpers::mapembed::MapEmbed, Context, Error};
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
    #[description = "The beatmap code (up to 5 alphanumeric characters). Will also accept BeatSaver links."]
    mut code: String,
) -> Result<(), Error> {
    // in case someone pastes directly from the twitch command clicky thingy
    if code.starts_with("!bsr ") {
        code = code[5..].to_string();
    }
    // in case someone pastes the link to the map with the code
    else if let Some(caps) = ctx.data().bsr_link_regex.captures(&code) {
        code = caps["bsr"].to_string();
    }

    info!("Code is {}", &code);

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
