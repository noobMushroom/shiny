use crate::channel::{
    can_create_channel, create_and_setup_chennael, get_channel, get_user_channel,
    handle_channel_delete,
};
use crate::guilds::get_guild;
use crate::messages::{post_invite, send_ephemeral_message};
use crate::roles::{get_role_id, has_active_channel};
use crate::utils::{Context, Error, Names};
use anyhow::anyhow;
use poise::serenity_prelude::Result;

/// Responds with "world!"
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    send_ephemeral_message(&ctx, "hope works").await?;
    Ok(())
}

/// deletes the channel if user have some channel
#[poise::command(slash_command)]
pub async fn delete_channel(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or_else(|| anyhow!("failed to get guild"))?;

    // Role to check if user already have an active channel
    let has_active_channel_role = get_role_id(&ctx, Names::has_active()).await?;

    //Checking if user has an active channel and deleting their channel for them and removing active
    //cahnnel role
    if has_active_channel(&ctx, &has_active_channel_role, &guild_id).await? {
        let channel = get_user_channel(&ctx).await?;
        handle_channel_delete(&ctx, channel, has_active_channel_role).await?;
        send_ephemeral_message(&ctx, "Your channel has been deleted successfully").await?;
    } else {
        send_ephemeral_message(
            &ctx,
            "You don't have any active channel \
            you can create channel by using /create_channel command",
        )
        .await?;
    }
    Ok(())
}

/// Post the message in invites
#[poise::command(slash_command)]
pub async fn post(
    ctx: Context<'_>,
    #[description = "The tille of your post"] title: String,
    #[description = "The description of your post"] description: String,
    #[description = "The roles you want to ping ex: @role1, @role2, @role3 "] target_role: Option<
        String,
    >,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => return Err(anyhow!("Guild id not found").into()),
    };

    let post_description = match target_role {
        Some(roles) => {
            let roles_mentions = roles
                .split(',')
                .map(|role| format!("{}", role.trim()))
                .collect::<Vec<String>>()
                .join(" ");
            format!("{} \n{}", description, roles_mentions)
        }
        None => description,
    };

    // Role to check if user already have an active channel
    let has_active_channel_role = get_role_id(&ctx, Names::has_active()).await?;

    //Checking if user has an active channel and deleting their channel for them and removing active
    //channel role
    if has_active_channel(&ctx, &has_active_channel_role, &guild_id).await? {
        let channel = get_user_channel(&ctx).await?;
        let invites_channel = get_channel(&ctx, Names::invites()).await?;
        post_invite(
            &ctx,
            &title,
            &invites_channel,
            &post_description,
            &channel.id,
        )
        .await?;
        send_ephemeral_message(&ctx, "Your invite has been posted in the invite channel").await?;
    } else {
        send_ephemeral_message(
            &ctx,
            "You don't have any active channel \
            you can create channel by using /create_channel command",
        )
        .await?;
    }
    Ok(())
}

/// Creates a private channel
#[poise::command(slash_command)]
pub async fn create_channel(
    ctx: Context<'_>,
    #[description = "type of channel [options: text voice video(stage)]"] channel_type: Option<
        String,
    >,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => return Err(anyhow!("Guild id not found").into()),
    };
    let guild = get_guild(&ctx).await?;

    if can_create_channel(&ctx).await? {
        let has_active_channel_role = get_role_id(&ctx, Names::has_active()).await?;

        //Checking if user has an active channel and creating channel for them and giving them active
        //cahnnel role
        if has_active_channel(&ctx, &has_active_channel_role, &guild_id).await? {
            send_ephemeral_message(
                &ctx,
                "You already have an active channel if you want new channel delete your active\
                channel first using /delete_channel command",
            )
            .await?;
        } else {
            create_and_setup_chennael(ctx, channel_type, guild, has_active_channel_role).await?;
            send_ephemeral_message(&ctx, "Your channel is successfully created. \
                The token  is sent to your inbox you can invite people or post the token by /post command").await?;
        }
    } else {
        send_ephemeral_message(&ctx, "You don't have permission to create channel").await?;
    }

    Ok(())
}
