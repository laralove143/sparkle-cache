use twilight_model::{
    guild::{Permissions, Role, RoleTags},
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
    util::ImageHash,
};

/// A cached emoji, it is the same as [`twilight_model::guild::Role`]
/// except:
///
/// - [`Self.guild_id`] field is added, making it possible to return a guild's
///   roles
#[derive(Clone, Debug)]
pub struct CachedRole {
    pub guild_id: Id<GuildMarker>,
    pub color: u32,
    pub hoist: bool,
    pub icon: Option<ImageHash>,
    pub id: Id<RoleMarker>,
    pub managed: bool,
    pub mentionable: bool,
    pub name: String,
    pub permissions: Permissions,
    pub position: i64,
    pub tags: Option<RoleTags>,
    pub unicode_emoji: Option<String>,
}

impl CachedRole {
    /// Create a cached role from a given role and guild ID
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn from_role(role: Role, guild_id: Id<GuildMarker>) -> Self {
        Self {
            guild_id,
            color: role.color,
            hoist: role.hoist,
            icon: role.icon,
            id: role.id,
            managed: role.managed,
            mentionable: role.mentionable,
            name: role.name,
            permissions: role.permissions,
            position: role.position,
            tags: role.tags,
            unicode_emoji: role.unicode_emoji,
        }
    }
}
