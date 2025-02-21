use poise;
use crate::{Context, Error};

#[poise::command(prefix_command)]
pub async fn sync(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}