use crate::roles::is_user_allowed;
use crate::utils::{self, Context, Error, SecretToken};
use poise::serenity_prelude::{ChannelType, CreateChannel, Guild, Member, Result, RoleId, UserId};
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
    #[description = "type of channel options: text voice video(stage)"] channel_type: Option<
        String,
    >,
) -> Result<(), Error> {
    let guild = Guild::get(&ctx.http(), ctx.guild_id().unwrap()).await?;
    let permission_roleid = SecretToken::get_token::<u64>(&ctx.data().secrets, "PERMIT_ROLE")
        .expect("failed to get token");
    let permission_roleid = RoleId::new(*permission_roleid.expose_secret());
    if is_user_allowed(&ctx, &permission_roleid, &ctx.guild_id().unwrap()).await? {
        ctx.say("Sanka you already have a channel go fuck yourself bitch :middle_finger:")
            .await?;
    } else {
        let channel_type =
            get_channel_type(&channel_type.unwrap_or_else(|| "text".to_string())).unwrap();
        let builder = channel_builder(&ctx, &channel_type, &ctx.author().name).await;
        guild.create_channel(&ctx.http(), builder).await?;
        let mem = guild.member(&ctx.http(), ctx.author().id).await?;
        let _ = give_role(&ctx, &permission_roleid, &mem).await;
    }
    Ok(())
}

async fn give_role(ctx: &Context<'_>, role_id: &RoleId, member: &Member) -> Result<(), Error> {
    member.add_role(&ctx.http(), role_id).await?;
    Ok(())
}

fn get_channel_type(channel: &str) -> Result<ChannelType, String> {
    let channel = channel.to_lowercase();

    match channel.as_str() {
        "text" => Ok(ChannelType::Text),
        "voice" => Ok(ChannelType::Voice),
        "video" => Ok(ChannelType::Stage),
        _ => Err("Wrong input".to_string()),
    }
}

async fn channel_builder<'a>(
    ctx: &Context<'_>,
    channel_type: &ChannelType,
    channel_name: &str,
) -> CreateChannel<'a> {
    let category_id = SecretToken::get_token::<u64>(&ctx.data().secrets, "CATEGORY_ID")
        .expect("Failed to get error");
    let default_role = SecretToken::get_token::<u64>(&ctx.data().secrets, "EVERYBODY_ROLE")
        .expect("Failed to get token");
    let user_id = UserId::new(ctx.author().id.into());
    let role_id = RoleId::new(*default_role.expose_secret());
    let permissions = utils::defualt_permissions(user_id, role_id);
    CreateChannel::new(channel_name)
        .category(*category_id.expose_secret())
        .nsfw(true)
        .permissions(permissions)
        .kind(*channel_type)
}
