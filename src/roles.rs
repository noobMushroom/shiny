#![allow(unused)]
use crate::guilds::get_guild;
use crate::utils::{Context, Error};
use poise::serenity_prelude::Result;
use poise::serenity_prelude::{self as serenity, GuildId, Member, PartialGuild, Role, RoleId};

pub fn get_roles<'a>(guild: &'a PartialGuild) -> Vec<&'a str> {
    guild
        .roles
        .values()
        .map(|role| role.name.as_str())
        .collect()
}

pub fn get_user_roles(ctx: &Context<'_>, member: &Member) -> Result<Vec<Role>, String> {
    if let Some(role) = member.roles(ctx.cache()) {
        Ok(role)
    } else {
        Err(String::from("The user doesn't have any roles"))
    }
}

pub async fn has_active_channel(
    ctx: &Context<'_>,
    roleid: &RoleId,
    guild_id: &GuildId,
) -> Result<bool, serenity::prelude::SerenityError> {
    let user = ctx.author();
    Ok(user.has_role(ctx, guild_id, roleid).await?)
}

pub async fn give_role(ctx: &Context<'_>, role_id: &RoleId, member: &Member) -> Result<(), Error> {
    member.add_role(&ctx, role_id).await?;
    Ok(())
}

pub async fn remove_role(
    ctx: &Context<'_>,
    role_id: &RoleId,
    member: &Member,
) -> Result<(), Error> {
    member.remove_role(&ctx, role_id).await?;
    Ok(())
}

pub async fn get_role_id(ctx: &Context<'_>, role_name: &str) -> Result<RoleId, Error> {
    let guild = get_guild(ctx).await?;
    let role = guild.roles.values().find(|role| role.name == role_name);
    match role {
        Some(r) => Ok(r.clone().id),
        None => Err("Role not found".into()),
    }
}
