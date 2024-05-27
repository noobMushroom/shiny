use crate::utils::{self, Context, SecretToken};
use poise::serenity_prelude::{ChannelType, CreateChannel, Result, RoleId, UserId};
use secrecy::ExposeSecret;

pub fn get_channel_type(channel: &str) -> Result<ChannelType, String> {
    let channel = channel.to_lowercase();
    match channel.as_str() {
        "text" => Ok(ChannelType::Text),
        "voice" => Ok(ChannelType::Voice),
        "video" => Ok(ChannelType::Stage),
        _ => Err("Wrong input".to_string()),
    }
}

pub async fn channel_builder<'a>(
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
