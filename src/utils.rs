// use anyhow::Context as _;
// use poise::serenity_prelude as serenity;

use poise::serenity_prelude::{
    PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId, UserId,
};

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub fn defualt_permissions(user_id: UserId, role_id: RoleId) -> Vec<PermissionOverwrite> {
    vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL
                | Permissions::SEND_MESSAGES
                | Permissions::ATTACH_FILES
                | Permissions::MANAGE_MESSAGES,
            deny: Permissions::CHANGE_NICKNAME | Permissions::CREATE_INSTANT_INVITE,
            kind: PermissionOverwriteType::Member(user_id),
        },
        PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(role_id),
        },
    ]
}

// Helping struct to get name of different role and category (To avoid speeling error in somewhere
// code and Later we can change what category and what roles to use) in the server for now there are only
// three names `has active channel` to check if user has any active channel and everybody role to
// make the channel private and category name where bot will create the private channel
pub struct Names;

impl<'a> Names {
    // for has_active_channel role
    pub fn has_active() -> &'a str {
        "has_active_channel"
    }

    // for everybody role
    pub fn everyboy() -> &'a str {
        "everybody"
    }

    // For category name
    pub fn category() -> &'a str {
        "PRIVATE ROOMS"
    }
}
