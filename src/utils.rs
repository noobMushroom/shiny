// use anyhow::Context as _;
// use poise::serenity_prelude as serenity;

use std::str::FromStr;

use anyhow::Context as _;
use poise::serenity_prelude::{
    PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId, UserId,
};
use secrecy::{zeroize::DefaultIsZeroes, Secret};

pub struct Data {
    pub secrets: shuttle_runtime::SecretStore,
}
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

pub struct SecretToken;

impl SecretToken {
    pub fn get_token<T>(
        secret_store: &shuttle_runtime::SecretStore,
        token_name: &str,
    ) -> Result<Secret<T>, anyhow::Error>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
        T: FromStr + std::fmt::Debug + DefaultIsZeroes,
    {
        let token = secret_store
            .get(token_name)
            .context(format!("Failed to get token, {}", token_name))?;
        let token_parse = token
            .parse::<T>()
            .context(format!("Failed to parse token, {}", token_name))?;
        Ok(Secret::new(token_parse))
    }
}
