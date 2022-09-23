use twilight_model::{
    guild::Emoji,
    id::{
        marker::{EmojiMarker, GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};

/// A cached emoji, it is the same as [`twilight_model::guild::Emoji`]
/// except:
///
/// - [`Self.guild_id`] field is added, making it possible to return a guild's
///   emojis
///
/// - [`twilight_model::guild::Emoji.user`] is changed to a user ID, which is
///   cached separately
#[derive(Clone, Debug)]
pub struct CachedEmoji {
    pub guild_id: Id<GuildMarker>,
    pub animated: bool,
    pub available: bool,
    pub id: Id<EmojiMarker>,
    pub managed: bool,
    pub name: String,
    pub require_colons: bool,
    pub roles: Vec<Id<RoleMarker>>,
    pub user: Option<Id<UserMarker>>,
}

impl CachedEmoji {
    /// Create a cached emoji from a given emoji and guild ID
    #[must_use]
    pub fn from_emoji(emoji: &Emoji, guild_id: Id<GuildMarker>) -> Self {
        Self {
            guild_id,
            animated: emoji.animated,
            available: emoji.available,
            id: emoji.id,
            managed: emoji.managed,
            name: emoji.name.clone(),
            require_colons: emoji.require_colons,
            roles: emoji.roles.clone(),
            user: emoji.user.as_ref().map(|user| user.id),
        }
    }
}
