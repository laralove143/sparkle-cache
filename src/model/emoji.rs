use twilight_model::{
    guild::Emoji,
    id::{
        marker::{EmojiMarker, GuildMarker, UserMarker},
        Id,
    },
};

/// A cached emoji
///
/// It's the same as [`twilight_model::guild::Emoji`]
/// except:
///
/// - `guild_id` field is added, making it possible to return a guild's emojis
///
/// - `user` field is changed to a user ID, as users are cached separately
///
/// - `roles` field is removed, as caching it is likely unnecessary, if you need
///   this field, please create an issue
#[derive(Clone, Debug)]
#[cfg_attr(feature = "tests", derive(PartialEq, Eq))]
pub struct CachedEmoji {
    pub guild_id: Id<GuildMarker>,
    pub animated: bool,
    pub available: bool,
    pub id: Id<EmojiMarker>,
    pub managed: bool,
    pub name: String,
    pub require_colons: bool,
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
            user: emoji.user.as_ref().map(|user| user.id),
        }
    }
}
