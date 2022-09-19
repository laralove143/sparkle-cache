use std::fmt::{Debug, Display};

use async_trait::async_trait;
use twilight_model::{
    guild::auto_moderation::AutoModerationRule,
    id::{
        marker::{AutoModerationRuleMarker, ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    user::CurrentUser,
};

use crate::{cache, model::CachedChannel};

/// A wrapper around the backend error, for example `Error(sqlx::Error)`
///
/// This is required because negative bounds aren't currently supported and
/// there's no other way to implement `From` a generic for [`cache::Error`]
#[derive(Clone, Debug)]
pub struct Error<E: Display + Debug>(pub E);

impl<E: Display + Debug> From<Error<E>> for cache::Error<E> {
    fn from(err: Error<E>) -> Self {
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
pub trait Backend<E: Display + Debug>: Sized {
    /// Set the current user information of the bot
    async fn set_current_user(&self, current_user: CurrentUser) -> Result<(), Error<E>>;

    /// Add or replace an auto moderation rule in the cache
    async fn upsert_auto_moderation_rule(&self, rule: AutoModerationRule) -> Result<(), Error<E>>;

    /// Remove an auto moderation rule from the cache
    async fn delete_auto_moderation_rule(
        &self,
        rule_id: Id<AutoModerationRuleMarker>,
    ) -> Result<(), Error<E>>;

    /// Add a banned user to the cache
    async fn upsert_ban(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Error<E>>;

    /// Remove a banned user from the cache
    async fn delete_ban(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Error<E>>;

    /// Add or replace a channel in the cache
    async fn upsert_channel(&self, channel: CachedChannel) -> Result<(), Error<E>>;

    /// Remove a channel from the cache
    async fn delete_channel(&self, channel_id: Id<ChannelMarker>) -> Result<(), Error<E>>;

    /// Add a DM channel to the cache
    ///
    /// This is different from a guild channel because it only has a channel ID
    /// and recipient user ID fields
    async fn upsert_private_channel(
        &self,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Error<E>>;

    /// Remove a DM channel from the cache
    async fn delete_private_channel(
        &self,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Error<E>>;
}
