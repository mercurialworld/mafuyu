use log::info;
use poise::{self, serenity_prelude::CreateEmbed, CreateReply};
use crate::{services::beatsaver::{api::get_map_data, map::BSMap}, Context, Error};

fn create_bsr_embed(map_info: Result<BSMap, Box<dyn std::error::Error>>, code: String) -> CreateEmbed {
    let embed: CreateEmbed;

    match map_info {
        Ok(map) => {
            embed = CreateEmbed::new()
                .title(map.name)
                .url(format!("https://beatsaver.com/maps/{}", code))
                .description(map.description)
                .field("Mapper(s)", map.metadata.level_author_name, false)
                .field("Artist(s)", map.metadata.song_author_name, false)
                .fields([
                    ("BPM", &map.metadata.bpm.to_string(), true),
                    ("Length", &format!("{}:{:0>2}", map.metadata.duration % 60, (map.metadata.duration / 60) % 60), true),
                    ("Rating", &format!("▲ {} / ▼ {} ({}%)", map.stats.upvotes, map.stats.downvotes, map.stats.score * 100.0), true),
                ])
                .thumbnail(&map.versions[0].cover_url)
                .timestamp(map.uploaded);
        }
        Err(error) => {
            embed = CreateEmbed::new()
                .title("Error")
                .description(error.to_string());
        }
    }

    embed
}

/// Searches a Beat Saber custom map from BeatSaver.   
#[poise::command(slash_command)]
pub async fn bsr(
    ctx: Context<'_>,
    #[description = "The beatmap code (up to 5 alphanumeric characters)"] code: String,
) -> Result<(), Error> {
    info!("/bsr used with code {}", &code);
    let map_info = get_map_data(&code).await;

    let embed = create_bsr_embed(map_info, code); 
    let builder = CreateReply::default().embed(embed);

    ctx.send(builder).await?;

    Ok(())
}
