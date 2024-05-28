use crate::guilds::get_guild;
use crate::messages::send_msg;
use crate::roles::{get_role_id, give_role, remove_role};
use crate::utils::{self, Context, Error, Names};
use poise::serenity_prelude::{
    ChannelId, ChannelType, CreateChannel, CreateInvite, GuildChannel, PartialGuild, Result,
    RichInvite, RoleId, UserId,
};

// Returns the channel type based on the input of user
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
) -> Result<CreateChannel<'a>, Error> {
    // Category Id unser which user want to create private channel
    let category_id = get_category_id(ctx, Names::category()).await?;

    // Everybody role in the server id
    let everybody_role_id = get_role_id(ctx, Names::everyboy()).await?;

    // Id of the users to give them permissions
    let user_id = UserId::new(ctx.author().id.into());

    // Getting default permissions
    let permissions = utils::defualt_permissions(user_id, everybody_role_id);

    Ok(CreateChannel::new(channel_name)
        .category(category_id)
        .nsfw(true)
        .permissions(permissions)
        .kind(*channel_type))
}

// Creating a invite link for the channel
pub async fn get_invite_link(
    ctx: &Context<'_>,
    channel: GuildChannel,
) -> Result<RichInvite, Error> {
    let builder = CreateInvite::new().max_age(8640);
    let invite = channel.create_invite(&ctx, builder).await?;
    Ok(invite)
}

// This functions create the channel and give has actvie channel role to user
pub async fn create_and_setup_chennael(
    ctx: Context<'_>,
    channel_type: Option<String>,
    guild: PartialGuild,
    has_active_channel: RoleId,
) -> Result<(), Error> {
    // Getting the channel type and if user give wrong input or don't input anything creating a
    // text channel
    let channel_type = get_channel_type(&channel_type.unwrap_or_else(|| "text".to_string()))?;

    // Builder to create channel
    let builder = channel_builder(&ctx, &channel_type, &ctx.author().name).await?;

    // Creating the channel for user
    let channel = guild.create_channel(&ctx, builder).await?;

    // Getting member to give role of has active channel
    let mem = guild.member(&ctx, ctx.author().id).await?;
    give_role(&ctx, &has_active_channel, &mem).await?;

    // Creating the invite and sending it to user's dm
    let invite = get_invite_link(&ctx, channel).await?;
    send_msg(&ctx, "invite link of channel", &invite.url()).await?;
    Ok(())
}

// returns the channel of user
pub async fn get_user_channel(ctx: &Context<'_>) -> Result<GuildChannel, Error> {
    let guild = get_guild(&ctx).await?;

    // Getting all the channels from the guild
    let all_channels = guild.channels(&ctx).await?;

    // Getting category id to filter the channel
    let category_id = get_category_id(ctx, Names::category()).await?;

    // Filtering the channel with users name
    let channel_opt = all_channels.values().find(|channel| {
        channel.parent_id == Some(category_id) && channel.name == ctx.author().name
    });

    Ok(channel_opt
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Channel not found"))?)
}

// Deleting the user channel and removing has active channel role from them
pub async fn handle_channel_delete(
    ctx: &Context<'_>,
    channel: GuildChannel,
    role_id: RoleId,
) -> Result<(), Error> {
    let _ = channel.delete(ctx).await?;
    let guild = get_guild(ctx).await?;
    let member = guild.member(ctx, ctx.author().id).await?;
    remove_role(ctx, &role_id, &member).await?;
    Ok(())
}

pub async fn get_category_id(ctx: &Context<'_>, category_name: &str) -> Result<ChannelId, Error> {
    let guild = get_guild(ctx).await?;
    let all_channels = guild.channels(ctx).await?;

    let category = all_channels
        .values()
        .find(|val| val.kind == ChannelType::Category && val.name == category_name);

    match category {
        Some(cat) => Ok(cat.clone().id),
        None => Err("Failed to find category".into()),
    }
}
