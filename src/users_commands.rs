use anyhow::anyhow;
use poise::serenity_prelude::{
    ChannelId, PermissionOverwrite, PermissionOverwriteType, Permissions,
};

use crate::{
    channel::get_channel,
    messages::send_ephemeral_message,
    utils::{Context, Error},
};

use poise::serenity_prelude::Result;

/// Add user to the channel
#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "The token of the channel that you want to join"] token: String,
) -> Result<(), Error> {
    let allow = Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::ATTACH_FILES;
    let deny = Permissions::empty();
    let overwrite = PermissionOverwrite {
        allow,
        deny,
        kind: PermissionOverwriteType::Member(ctx.author().id),
    };
    let channel_id = match token.parse::<u64>() {
        Ok(id) => ChannelId::new(id),
        Err(_) => {
            send_ephemeral_message(&ctx, "wrong token").await?;
            return Err(anyhow!("wrong token").into());
        }
    };

    let channel = get_channel(&ctx, &channel_id).await?;
    channel.create_permission(ctx, overwrite).await?;
    let message = format!("You have been added to the {} channel", channel.name());
    send_ephemeral_message(&ctx, &message).await?;
    Ok(())
}

//TODO
/* #[allow(unused)]
#[poise::command(slash_command)]
pub async fn add_user(
    ctx: Context<'_>,
    #[description = "The user you want to add example: @user1, @user2, @user3"] users: String,
    #[description = "What does you want to tell them"] message: String,
) -> Result<(), Error> {
    let guild = get_guild(&ctx).await?;
    let user_channel = get_user_channel(&ctx).await?;

    let users: Vec<String> = users
        .split(',')
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    for user in users {
        let members = guild.search_members(&ctx, &user, Some(1)).await?;
        for member in members {}
    }

    Ok(())
} */
