use crate::utils::{Context, Error};
use anyhow::anyhow;
use poise::serenity_prelude::{Guild, PartialGuild};

pub async fn get_guild(ctx: &Context<'_>) -> Result<PartialGuild, Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| anyhow!("failed to get guild"))?;
    Ok(Guild::get(&ctx, guild_id).await?)
}
