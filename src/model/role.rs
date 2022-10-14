use twilight_model::{
    guild::{Permissions, Role},
    id::{
        marker::{GuildMarker, IntegrationMarker, RoleMarker, UserMarker},
        Id,
    },
    util::ImageHash,
};

/// A cached emoji
///
/// It's the same as [`twilight_model::guild::Role`] except:
///
/// - `guild_id` field is added, making it possible to return a guild's roles
///
/// - `user_id` field is added, making it possible to return a member's roles
///
/// - `tags` field is flattened, making this struct easier to cache
#[derive(Clone, Debug)]
#[cfg_attr(feature = "tests", derive(PartialEq, Eq))]
pub struct CachedRole {
    pub guild_id: Id<GuildMarker>,
    pub user_id: Option<Id<UserMarker>>,
    pub color: u32,
    pub hoist: bool,
    pub icon: Option<ImageHash>,
    pub id: Id<RoleMarker>,
    pub managed: bool,
    pub mentionable: bool,
    pub name: String,
    pub permissions: Permissions,
    pub position: i64,
    pub tags_bot_id: Option<Id<UserMarker>>,
    pub tags_integration_id: Option<Id<IntegrationMarker>>,
    pub tags_premium_subscriber: Option<bool>,
    pub unicode_emoji: Option<String>,
}

impl CachedRole {
    /// Create a cached role from a given role and guild ID
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn from_role(role: Role, guild_id: Id<GuildMarker>) -> Self {
        Self {
            guild_id,
            user_id: None,
            color: role.color,
            hoist: role.hoist,
            icon: role.icon,
            id: role.id,
            managed: role.managed,
            mentionable: role.mentionable,
            name: role.name,
            permissions: role.permissions,
            position: role.position,
            tags_bot_id: role.tags.as_ref().and_then(|tags| tags.bot_id),
            tags_integration_id: role.tags.as_ref().and_then(|tags| tags.integration_id),
            tags_premium_subscriber: role.tags.map(|tags| tags.premium_subscriber),
            unicode_emoji: role.unicode_emoji,
        }
    }
}
