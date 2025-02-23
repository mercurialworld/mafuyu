use crate::{
    services::beatsaver::{api::get_map_data, map::BSMap},
    Context, Error,
};
use log::info;
use poise::{
    self,
    serenity_prelude::{
        CreateActionRow, CreateButton, CreateEmbed, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption
    },
    CreateReply,
};

/// Creates the general map info embed.
fn create_map_info_embed(map: &BSMap, code: String) -> CreateEmbed {
    let embed: CreateEmbed;

    embed = CreateEmbed::new()
        .title(&map.name)
        .url(format!("https://beatsaver.com/maps/{}", code))
        .description(&map.description)
        .field("Mapper(s)", &map.metadata.level_author_name, false)
        .field("Artist(s)", &map.metadata.song_author_name, false)
        .fields([
            ("BPM", &map.metadata.bpm.to_string(), true),
            (
                "Length",
                &format!(
                    "{}:{:0>2}",
                    map.metadata.duration % 60,
                    (map.metadata.duration / 60) % 60
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
        .thumbnail(&map.versions[0].cover_url)
        .timestamp(map.uploaded);

    embed
}

/// Creates the options menu for the available difficulties.
fn get_map_diffs(map: &BSMap) -> Vec<CreateSelectMenuOption> {
    let map_diffs: Vec<CreateSelectMenuOption>;

    map_diffs = map.versions[0]
        .diffs
        .iter()
        .map(|diff| {
            CreateSelectMenuOption::new(
                format!("{} ({})", diff.difficulty, diff.characteristic),
                format!("{}{}", diff.characteristic, diff.difficulty),
            )
        })
        .collect();

    map_diffs
}

/// Searches a Beat Saber custom map from BeatSaver.   
#[poise::command(slash_command)]
pub async fn bsr(
    ctx: Context<'_>,
    #[description = "The beatmap code (up to 5 alphanumeric characters)"] code: String,
) -> Result<(), Error> {
    info!("/bsr used with code {}", &code);
    let embed: CreateEmbed;
    let diffs: Vec<CreateSelectMenuOption>;
    let builder: CreateReply;

    match get_map_data(&code).await {
        Ok(map) => {
            embed = create_map_info_embed(&map, code);
            diffs = get_map_diffs(&map);

            builder =
                CreateReply::default()
                    .embed(embed)
                    .components(vec![
                        CreateActionRow::SelectMenu(
                            CreateSelectMenu::new(
                                "diffsel",
                                CreateSelectMenuKind::String { options: diffs },
                            )
                            .placeholder("Select Difficulty"),
                        ),
                        CreateActionRow::Buttons(vec![
                            CreateButton::new_link(&map.versions[0].download_url)
                                .label("Download map")
                                .emoji('⬇')
                            ]
                        )
                    ]);
        }
        Err(err) => {
            embed = CreateEmbed::new()
                .title("Error!")
                .description(err.to_string());

            builder = CreateReply::default().embed(embed);
        }
    }

    ctx.send(builder).await?;
    Ok(())
}
