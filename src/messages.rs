use crate::utils::{Context, Error};
use poise::{
    serenity_prelude::{ChannelId, CreateEmbed, CreateMessage, GuildChannel, Result},
    CreateReply,
};

// sends the message
pub async fn send_msg(ctx: &Context<'_>, title: &str, description: &str) -> Result<(), Error> {
    let embed = CreateEmbed::new().title(title).description(description);
    let message = CreateMessage::new().embed(embed);
    ctx.author().direct_message(&ctx, message).await?;
    Ok(())
}

// sends ephemeral messages
pub async fn send_ephemeral_message(ctx: &Context<'_>, description: &str) -> Result<(), Error> {
    ctx.send(CreateReply::default().content(description).ephemeral(true))
        .await?;

    Ok(())
}

pub async fn post_invite(
    ctx: &Context<'_>,
    title: &str,
    channel: &GuildChannel,
    description: &str,
    channel_token: &ChannelId,
) -> Result<(), Error> {
    let message = format!("**Channel Token: {}**", channel_token.to_string());
    let embed = CreateEmbed::new().title(title).description(description);
    let builder = CreateMessage::new()
        .content(message)
        .embed(embed);
    channel.send_message(ctx, builder).await?;

    Ok(())
}
