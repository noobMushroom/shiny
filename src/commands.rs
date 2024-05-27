use crate::channel::{channel_builder, get_channel_type};
use crate::roles::{give_role, is_user_allowed};
use crate::utils::{Context, Error, SecretToken};
use poise::serenity_prelude::{Guild, Result, RoleId};
use secrecy::ExposeSecret;

/// Responds with "world!"
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn create_channel(
    ctx: Context<'_>,
    #[description = "type of channel [options: text voice video(stage)]"] channel_type: Option<
        String,
    >,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("failed to get guild id");
    let guild = Guild::get(&ctx.http(), guild_id).await?;
    let permission_roleid = SecretToken::get_token::<u64>(&ctx.data().secrets, "PERMIT_ROLE")
        .expect("failed to get token");
    let permission_roleid = RoleId::new(*permission_roleid.expose_secret());
    if is_user_allowed(&ctx, &permission_roleid, &guild_id).await? {
        ctx.say("Sanka you already have a channel go fuck yourself bitch :middle_finger:")
            .await?;
    } else {
        let channel_type = get_channel_type(&channel_type.unwrap_or_else(|| "text".to_string()))?;
        let builder = channel_builder(&ctx, &channel_type, &ctx.author().name).await;
        guild.create_channel(&ctx.http(), builder).await?;
        let mem = guild.member(&ctx.http(), ctx.author().id).await?;
        let _ = give_role(&ctx, &permission_roleid, &mem).await?;
    }
    Ok(())
}
