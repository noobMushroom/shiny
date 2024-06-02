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
                | Permissions::MANAGE_MESSAGES
                | Permissions::MANAGE_NICKNAMES
                | Permissions::CHANGE_NICKNAME,
            deny: Permissions::CREATE_INSTANT_INVITE,
            kind: PermissionOverwriteType::Member(user_id),
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(role_id),
        },
    ]
}

pub fn defualt_permissions_invites(role_id: RoleId) -> Vec<PermissionOverwrite> {
    vec![PermissionOverwrite {
        allow: Permissions::VIEW_CHANNEL,
        deny: Permissions::SEND_MESSAGES | Permissions::CREATE_INSTANT_INVITE,
        kind: PermissionOverwriteType::Role(role_id),
    }]
}

pub fn defualt_permissions_pvt_rooms(role_id: RoleId) -> Vec<PermissionOverwrite> {
    vec![PermissionOverwrite {
        allow: Permissions::empty(),
        deny: Permissions::all(),
        kind: PermissionOverwriteType::Role(role_id),
    }]
}

// Helping struct to get name of different roles and categories in the server(To avoid spelling error somewhere
// in the code and Later we can change what category and what roles to use)
pub struct Names;

impl<'a> Names {
    // for has_active_channel role
    pub fn has_active() -> &'a str {
        "has_active_channel"
    }

    // For category name
    pub fn private_rooms_category() -> &'a str {
        "Private Rooms"
    }

    // For the role to be able to create channel
    pub fn can_create_channel() -> &'a str {
        "can_create_channel"
    }

    //Everyone role
    pub fn everyone_role() -> &'a str {
        "@everyone"
    }

    // For the invitations category
    pub fn invitations_category() -> &'a str {
        "invitations"
    }

    // For the invites channel
    pub fn invites() -> &'a str {
        "invites"
    }
}
