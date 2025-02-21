use log::info;
use poise;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn bsr(
    ctx: Context<'_>,
    #[description = "The beatmap code (up to 5 alphanumeric characters)"] code: String,
) -> Result<(), Error> {
    // map_info = beatsaver

    ctx.say(format!("https://beatsaver.com/maps/{}", code)).await?;

    Ok(())
}