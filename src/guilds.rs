use crate::utils::{Context, Error};
use anyhow::anyhow;
use poise::serenity_prelude::{Guild, PartialGuild};

pub async fn get_guild(ctx: &Context<'_>) -> Result<PartialGuild, Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => return Err(anyhow!("failed to get guild id").into()),
    };
    Ok(Guild::get(&ctx, guild_id).await?)
}
