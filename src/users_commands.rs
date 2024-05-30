use poise::serenity_prelude::{
    ChannelId, PermissionOverwrite, PermissionOverwriteType, Permissions,
};

use crate::{
    channel::get_channel_by_id,
    messages::send_ephemeral_message,
    utils::{Context, Error},
};

use poise::serenity_prelude::Result;

/// Add user to the channel
#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "type of channel [options: text voice video(stage)]"] token: Option<String>,
) -> Result<(), Error> {
    let allow = Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::ATTACH_FILES;
    let deny = Permissions::empty();
    let overwrite = PermissionOverwrite {
        allow,
        deny,
        kind: PermissionOverwriteType::Member(ctx.author().id),
    };

    if let Some(id) = token {
        let parse_id = id.parse::<u64>();

        match parse_id {
            Ok(token) => {
                let channel_id = ChannelId::new(token);
                let channel = get_channel_by_id(&ctx, &channel_id).await?;
                channel.create_permission(ctx, overwrite).await?;
                let message = format!("You have been added to the {} channel", channel.name());
                send_ephemeral_message(&ctx, &message).await?;
            }
            Err(_) => send_ephemeral_message(&ctx, "wrong token").await?,
        }
    } else {
        send_ephemeral_message(&ctx, "Please provide a token").await?;
    }

    Ok(())
}
