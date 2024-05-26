use crate::utils::{Context, Error};

/// Responds with "world!"
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn create_channel(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Something").await?;
    Ok(())
}
