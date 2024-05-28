use crate::channel::{create_and_setup_chennael, get_user_channel, handle_channel_delete};
use crate::guilds::get_guild;
use crate::messages::send_ephemeral_message;
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
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => return Err(anyhow!("Guild id not found").into()),
    };

    // Role to check if user already have an active channel
    let has_active_channel_role = get_role_id(&ctx, Names::has_active()).await?;

    //Checking if user has an active channel and deleting their channel for them and removing active
    //cahnnel role
    if has_active_channel(&ctx, &has_active_channel_role, &guild_id).await? {
        let channel = get_user_channel(&ctx).await?;
        handle_channel_delete(&ctx, channel, has_active_channel_role).await?;
    } else {
        ctx.say("You don't have any active channels").await?;
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

    // Role to check if user already have an active channel
    let has_active_channel_role = get_role_id(&ctx, Names::has_active()).await?;

    //Checking if user has an active channel and creating channel for them and giving them active
    //cahnnel role
    if has_active_channel(&ctx, &has_active_channel_role, &guild_id).await? {
        send_ephemeral_message(&ctx, "You already have an active channel if you want new channel delete the channel first using /delete_channel command").await?;
    } else {
        create_and_setup_chennael(ctx, channel_type, guild, has_active_channel_role).await?;
        send_ephemeral_message(&ctx, "Your channel is successfully created you the invitaion like is sent to your inbox you can invite people or post the link by /post comman").await?;
    }
    Ok(())
}
