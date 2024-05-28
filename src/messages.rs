use crate::utils::{Context, Error};
use poise::{
    serenity_prelude::{CreateEmbed, CreateMessage, Result},
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
