/* use std::time::Duration;

use anyhow::Context as _;
use chrono::TimeDelta;
use poise::serenity_prelude::{self as serenity, CacheHttp, ChannelId, GetMessages, Timestamp};
use tokio::time::interval;

use crate::{
    channel::get_category_id,
    guilds::get_guild,
    utils::{Context, Error, Names},
};

pub async fn delete_old_message(
    ctx: Context<'_>,
    channel_name: &str,
    category: &str,
) -> Result<(), Error> {
    let guild = get_guild(&ctx).await?;
    let all_channels = guild.channels(&ctx).await?;
    let category_id = get_category_id(&ctx, category).await?;

    let channel = all_channels
        .values()
        .find(|v| v.parent_id == Some(category_id) && v.name == channel_name);

    let now = Timestamp::now().time();
    let twenty_four_hours_ago = TimeDelta::new(24 * 60 * 60, 0).context("failed to change time")?;

    if let Some(channel) = channel {
        let builder = GetMessages::new().limit(100);
        let messages = channel.messages(ctx, builder).await?;
        for message in messages {
            let message_time = message.timestamp.time();
            if now - message_time > twenty_four_hours_ago {
                channel.id.delete_message(ctx, message.id).await?;
            }
        }
    }
    Ok(())
}

async fn schedule_message_delete(ctx: Context<'_>) {
    let mut interval = interval(Duration::from_secs(60 * 60));

    loop {
        interval.tick().await;
        if let Err(why) =
            delete_old_message(ctx, Names::invites(), Names::invitations_category()).await
        {
            eprintln!("failed to delete the message {:?}", why);
        }
    }
} */
