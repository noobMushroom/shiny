use poise::serenity_prelude::{EditRole, EventHandler, Guild};
use shuttle_runtime::async_trait;

pub struct Handler;

// Creating has_active_channel role everytime bot starts and when user connects bot to their server
#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(
        &self,
        ctx: poise::serenity_prelude::Context,
        guild: Guild,
        _is_new: Option<bool>,
    ) {
        if !guild
            .roles
            .values()
            .any(|role| role.name == "has_active_channel")
        {
            let builder = EditRole::new()
                .name("has_active_channel")
                .mentionable(false);
            if let Err(e) = guild.create_role(&ctx.http, builder).await {
                println!("Failed to create role: {}", e);
            }
        }
    }
}
