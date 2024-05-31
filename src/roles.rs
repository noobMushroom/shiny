use crate::guilds::get_guild;
use crate::utils::{Context, Error};
use anyhow::anyhow;
use poise::serenity_prelude::{GuildId, Member, PartialGuild, Role, RoleId};

#[allow(unused)]
pub fn get_roles<'a>(guild: &'a PartialGuild) -> Vec<&'a str> {
    guild
        .roles
        .values()
        .map(|role| role.name.as_str())
        .collect()
}

pub fn get_user_roles(ctx: &Context<'_>, member: &Member) -> Result<Vec<Role>, Error> {
    member
        .roles(ctx)
        .ok_or_else(|| anyhow!("user doesn't have any roles").into())
}

pub async fn has_active_channel(
    ctx: &Context<'_>,
    roleid: &RoleId,
    guild_id: &GuildId,
) -> Result<bool, Error> {
    let user = ctx.author();
    Ok(user.has_role(ctx, guild_id, roleid).await?)
}

pub async fn get_role_id(ctx: &Context<'_>, role_name: &str) -> Result<RoleId, Error> {
    let guild = get_guild(ctx).await?;
    guild
        .roles
        .values()
        .find(|role| role.name == role_name)
        .map(|role| role.id)
        .ok_or_else(|| anyhow!("Failed to find role").into())
}
