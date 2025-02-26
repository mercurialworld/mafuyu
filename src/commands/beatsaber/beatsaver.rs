use std::collections::HashMap;

use crate::{
    services::beatsaver::{
        api::get_map_data,
        map::{BSMap, MapDetail},
    },
    Context, Error,
};
use log::info;
use poise::{
    self,
    serenity_prelude::{
        self as serenity, Colour, CreateActionRow, CreateButton, CreateEmbed,
        CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu,
        CreateSelectMenuKind, CreateSelectMenuOption,
    },
    CreateReply,
};

/// Adds a colour to the map embed.
fn get_embed_colour(map: &BSMap) -> Colour {
    if map.ss_ranked || map.bl_ranked {
        Colour::from_rgb(243, 156, 18)
    } else if map.curated_at.is_some() {
        Colour::from_rgb(0, 188, 140)
    } else if map.uploader.verified_mapper {
        Colour::from_rgb(118, 70, 175)
    } else {
        Colour::from_rgb(68, 68, 68)
    }
}

/// Adds a difficulty colour to the map embed.
fn get_diff_colour(diff_name: &str) -> Colour {
    match diff_name {
        "ExpertPlus" => Colour::from_rgb(166, 149, 255),
        "Expert" => Colour::from_rgb(255, 149, 166),
        "Hard" => Colour::from_rgb(255, 183, 77),
        "Normal" => Colour::from_rgb(0, 238, 255),
        "Easy" => Colour::from_rgb(129, 199, 132),
        _ => unreachable!(),
    }
}

/// Creates the general map info embed.
fn create_map_info_embed(map: &BSMap, code: &String) -> CreateEmbed {
    let embed: CreateEmbed = CreateEmbed::new()
        .title(&map.name)
        .url(format!("https://beatsaver.com/maps/{}", code))
        .description(&map.description)
        .thumbnail(&map.versions[0].cover_url)
        .timestamp(map.uploaded);

    embed
}

/// Creates the metadata part of the embed.
fn create_map_metadata_embed(map: &BSMap, embed: CreateEmbed) -> CreateEmbed {
    embed
        .field("Mapper(s)", &map.metadata.level_author_name, false)
        .field("Artist(s)", &map.metadata.song_author_name, false)
        .fields([
            ("BPM", &map.metadata.bpm.to_string(), true),
            (
                "Length",
                &format!(
                    "{}:{:0>2}",
                    (map.metadata.duration / 60) % 60,
                    map.metadata.duration % 60
                ),
                true,
            ),
            (
                "Rating",
                &format!(
                    "▲ {} / ▼ {} ({}%)",
                    map.stats.upvotes,
                    map.stats.downvotes,
                    map.stats.score * 100.0
                ),
                true,
            ),
        ])
        .colour(get_embed_colour(map))
}

/// Creates the options menu for the available difficulties.
fn get_map_diffs(map: &BSMap) -> Vec<CreateSelectMenuOption> {
    let mut map_diffs: Vec<CreateSelectMenuOption> =
        vec![CreateSelectMenuOption::new("Metadata", "Metadata")];

    map_diffs.extend(map.versions[0].diffs.iter().map(|diff| {
        CreateSelectMenuOption::new(
            format!("{} ({})", diff.difficulty, diff.characteristic),
            format!("{}{}", diff.characteristic, diff.difficulty),
        )
    }));

    info!("{:?}", map_diffs);

    map_diffs
}

/// Creates the embed representing data for one difficulty.
fn create_map_diff_embed(diff: &MapDetail, mut embed: CreateEmbed) -> CreateEmbed {
    embed = embed.field(
        "Characteristic/Difficulty",
        format!("{} {}", diff.characteristic, diff.difficulty),
        false,
    );

    if let Some(x) = &diff.label {
        embed = embed.field("Label", x, false);
    }

    embed
        .fields(vec![
            ("Notes", diff.notes.to_string(), true),
            ("Bombs", diff.bombs.to_string(), true),
            ("Walls", diff.obstacles.to_string(), true),
            ("NJS", diff.njs.to_string(), true),
            ("NPS", diff.nps.to_string(), true),
            ("Lights", diff.events.to_string(), true),
        ])
        .colour(get_diff_colour(&diff.difficulty))
}

/// Creates a vector of embeds representing difficulty data.
fn create_map_diff_embeds(map: &BSMap, embed: CreateEmbed) -> HashMap<String, CreateEmbed> {
    let mut diffs = HashMap::new();
    for diff in map.versions[0].diffs.iter() {
        let diff_embed = embed.clone();

        diffs.insert(
            format!("{}{}", diff.characteristic, diff.difficulty),
            create_map_diff_embed(diff, diff_embed),
        );
    }

    diffs
}

/// Searches a Beat Saber custom map from BeatSaver.   
#[poise::command(slash_command)]
pub async fn bsr(
    ctx: Context<'_>,
    #[description = "The beatmap code (up to 5 alphanumeric characters)"] code: String,
) -> Result<(), Error> {
    let map: BSMap = get_map_data(&code).await?;
    let embed_base: CreateEmbed = create_map_info_embed(&map, &code);

    let metadata_embed: CreateEmbed = create_map_metadata_embed(&map, embed_base.clone());
    let embed_components = vec![
        CreateActionRow::SelectMenu(
            CreateSelectMenu::new(
                "diffsel",
                CreateSelectMenuKind::String {
                    options: get_map_diffs(&map),
                },
            )
            .placeholder("Select Difficulty"),
        ),
        CreateActionRow::Buttons(vec![
            CreateButton::new_link(&map.versions[0].download_url)
                .label("Download map")
                .emoji('⬇'),
            CreateButton::new_link(format!(
                "https://allpoland.github.io/ArcViewer/?id={}",
                &code
            ))
            .label("Preview map")
            .emoji('⏯'),
        ]),
    ];
    let mut diff_embeds: HashMap<String, CreateEmbed> =
        create_map_diff_embeds(&map, embed_base.clone());

    diff_embeds.insert("Metadata".to_string(), metadata_embed.clone());

    info!("difficulties: {:?}", diff_embeds);

    let builder: CreateReply = CreateReply::default()
        .embed(metadata_embed)
        .components(embed_components);

    // general metadata message
    let reply = ctx.send(builder).await?;

    // here's the collector for diffs and stuff
    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .message_id(reply.message().await?.id)
        .timeout(std::time::Duration::from_secs(15 * 60))
        .filter(move |mci| mci.data.custom_id == "diffsel")
        .await
    {
        info!("interaction happened");

        let diff_key = match &mci.data.kind {
            serenity::ComponentInteractionDataKind::StringSelect { values } => &values[0],
            _ => panic!("unexpected interaction data kind"),
        };

        let diff_builder = CreateInteractionResponseMessage::new()
            .embed(diff_embeds.get(diff_key).unwrap().clone());

        let new_message = CreateInteractionResponse::UpdateMessage(diff_builder);

        mci.create_response(ctx, new_message).await?;
    }

    Ok(())
}
