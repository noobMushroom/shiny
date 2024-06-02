use poise::serenity_prelude::{
    ChannelType, CreateChannel, EditRole, EventHandler, Guild, PermissionOverwrite, Permissions,
};
use shuttle_runtime::async_trait;

use crate::{
    roles::get_everyone_id,
    utils::{defualt_permissions_invites, defualt_permissions_pvt_rooms, Names},
};

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
        let everybody_role = get_everyone_id(&guild);
        let invites_permissions = defualt_permissions_invites(everybody_role.id);
        let pvt_room_perms = defualt_permissions_pvt_rooms(everybody_role.id);

        // Creating has active role
        create_role(&ctx, Names::has_active(), &guild).await;

        // Creating can create channel role
        create_role(&ctx, Names::can_create_channel(), &guild).await;

        // Creating private room category
        create_category(
            &ctx,
            Names::private_rooms_category(),
            &guild,
            &pvt_room_perms,
        )
        .await;

        // Creating invitations category
        create_category(
            &ctx,
            Names::invitations_category(),
            &guild,
            &invites_permissions,
        )
        .await;

        // Creating channel invites inisted invitatiosn category
        create_channel(
            &ctx,
            Names::invites(),
            &guild,
            Names::invitations_category(),
            &invites_permissions,
        )
        .await;
    }
}

async fn create_category(
    ctx: &poise::serenity_prelude::Context,
    name: &str,
    guild: &Guild,
    permissons: &Vec<PermissionOverwrite>,
) {
    if let Ok(channels) = guild.channels(&ctx).await {
        if !channels
            .values()
            .any(|v| v.kind == ChannelType::Category && v.name == name)
        {
            let builder = CreateChannel::new(name)
                .kind(ChannelType::Category)
                .permissions(permissons.clone());
            if let Err(e) = guild.create_channel(&ctx.http, builder).await {
                eprintln!("Failed to create category {}", e);
            }
        }
    } else {
        eprintln!("Failed to create category");
    }
}

pub async fn create_role(ctx: &poise::serenity_prelude::Context, name: &str, guild: &Guild) {
    if !guild.roles.values().any(|role| role.name == name) {
        let builder = EditRole::new()
            .name(name)
            .mentionable(false)
            .permissions(Permissions::empty());
        if let Err(e) = guild.create_role(&ctx.http, builder).await {
            eprintln!("Failed to create role: {}", e);
        }
    }
}

pub async fn create_channel(
    ctx: &poise::serenity_prelude::Context,
    name: &str,
    guild: &Guild,
    category_name: &str,
    perms: &Vec<PermissionOverwrite>,
) {
    if let Ok(channels) = guild.channels(&ctx).await {
        if channels
            .values()
            .any(|v| v.kind == ChannelType::Category && v.name == category_name)
        {
            if let Ok(channel) = guild.channels(&ctx).await {
                if let Some(category_id) = channel
                    .values()
                    .find(|v| v.kind == ChannelType::Category && v.name == category_name)
                {
                    if !channels.values().any(|channel| {
                        channel.name == name && channel.parent_id == Some(category_id.id)
                    }) {
                        let builder = CreateChannel::new(name)
                            .kind(ChannelType::Text)
                            .permissions(perms.clone())
                            .category(category_id.id);
                        if let Err(e) = guild.create_channel(&ctx.http, builder).await {
                            println!("Failed to create channel {}", e);
                        }
                    }
                }
            }
        }
    } else {
        println!("Channel is alreay available");
    }
}
