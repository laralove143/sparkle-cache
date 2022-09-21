use std::fmt::Display;

use async_trait::async_trait;
use twilight_model::{
    id::{
        marker::{ChannelMarker, EmojiMarker, GuildMarker, StickerMarker, UserMarker},
        Id,
    },
    user::CurrentUser,
};

use crate::{
    cache,
    model::{CachedChannel, CachedEmoji, CachedGuild, CachedMember, CachedSticker},
};

/// Implemented on backend errors, for example `Error(sqlx::Error)`
pub trait Error: Display + Send {}

impl<E: Error> From<E> for cache::Error<E> {
    fn from(err: E) -> Self {
        Self::Backend(err)
    }
}

/// Provides methods to add or replace data in the cache
///
/// This is for adding support for a backend, users of the cache itself only
/// need the methods in [`super::cache::Cache`]
///
/// # This trait is not complete
///
/// You should expose the backend so that users can filter the results in the
/// query, for example they can do `SELECT *  FROM users WHERE name = ?`
///
/// It's also advisable to implement your backend library's traits to
/// (de)serialize Discord models for the backend to streamline your codebase
///
/// Creating indexes for every ID field/column (for example, both `user_id` and
/// `guild_id` in `users`) will be a huge performance improvement
///
/// # Example
///
/// Though the example uses PostgresSQL, you can use this library with any SQL
/// or NoSQL backend
///
/// ```ignore
/// use twilight_cache::backend::Backend;
/// use twilight_model::id::{
///     marker::{GuildMarker, UserMarker},
///     Id,
/// };
/// struct MyCache {
///     pub db: sql_library::Database, // Or add a getter method instead of making the field public
/// };
/// impl MyCache {
///     fn new() {
///         let db = sql_library::Database::connect("postgresql://localhost/discord");
///         db.query("CREATE INDEX bans_guild_id_index ON bans (guild_id);");
///         db.query("CREATE INDEX bans_user_id_index ON bans (user_id);");
///     }
/// }
/// impl Backend for MyCache {
///     async fn add_ban(
///         &self,
///         guild_id: Id<GuildMarker>,
///         user_id: Id<UserMarker>,
///     ) -> Result<(), Self::Error> {
///         self.db
///             .query("INSERT INTO bans (guild_id, user_id) VALUES ($guild_id, $user_id)");
///         Ok(())
///     }
///     // Implement other methods similarly
/// }
/// impl Cache for MyCache {
///     // Implement the methods here, usually using getter queries
/// }
/// ```
#[async_trait]
pub trait Backend {
    /// The error type the backend returns, for example `Error(sqlx::Error)`
    type Error: Error;

    /// Set the current user information of the bot
    async fn set_current_user(&self, current_user: CurrentUser) -> Result<(), Self::Error>;

    /// Add or replace a channel in the cache
    async fn upsert_channel(&self, channel: CachedChannel) -> Result<(), Self::Error>;

    /// Remove a channel from the cache
    async fn delete_channel(&self, channel_id: Id<ChannelMarker>) -> Result<(), Self::Error>;

    /// Remove a guild's channels from the cache
    ///
    /// This should be something like `DELETE FROM channels WHERE guild_id = ?`
    async fn delete_guild_channels(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

    /// Add a DM channel to the cache
    ///
    /// This is different from a guild channel because it only has a channel ID
    /// and recipient user ID fields
    async fn upsert_private_channel(
        &self,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error>;

    /// Remove a DM channel from the cache
    async fn delete_private_channel(
        &self,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error>;

    /// Add or replace a guild in the cache
    async fn upsert_guild(&self, guild: CachedGuild) -> Result<(), Self::Error>;

    /// Remove a channel from the cache
    async fn delete_guild(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

    /// Add or replace a emoji in the cache
    async fn upsert_emoji(&self, emoji: CachedEmoji) -> Result<(), Self::Error>;

    /// Remove a emoji from the cache
    async fn delete_emoji(&self, emoji_id: Id<EmojiMarker>) -> Result<(), Self::Error>;

    /// Remove a guild's emojis from the cache
    ///
    /// This should be something like `DELETE FROM emojis WHERE guild_id = ?`
    async fn delete_guild_emojis(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

    /// Add or replace a sticker in the cache
    async fn upsert_sticker(&self, sticker: CachedSticker) -> Result<(), Self::Error>;

    /// Remove a sticker from the cache
    async fn delete_sticker(&self, sticker_id: Id<StickerMarker>) -> Result<(), Self::Error>;

    /// Remove a guild's stickers from the cache
    ///
    /// This should be something like `DELETE FROM stickers WHERE guild_id = ?`
    async fn delete_guild_stickers(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

    /// Add or replace a member in the cache
    async fn upsert_member(&self, member: CachedMember) -> Result<(), Self::Error>;

    /// Remove a member from the cache
    async fn delete_member(
        &self,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Result<(), Self::Error>;

    /// Remove a guild's stickers from the cache
    ///
    /// This should be something like `DELETE FROM members WHERE guild_id = ?`
    async fn delete_guild_members(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;
}
