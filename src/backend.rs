use core::fmt::Display;

use async_trait::async_trait;
use twilight_model::{
    channel::StageInstance,
    id::{
        marker::{
            ChannelMarker, EmojiMarker, GenericMarker, GuildMarker, MessageMarker, RoleMarker,
            StageMarker, UserMarker,
        },
        Id,
    },
    user::CurrentUser,
};

use crate::{
    cache,
    model::{
        CachedActivity, CachedAttachment, CachedChannel, CachedEmbed, CachedEmbedField,
        CachedEmoji, CachedGuild, CachedMember, CachedMessage, CachedPermissionOverwrite,
        CachedPresence, CachedReaction, CachedRole, CachedSticker,
    },
};

impl<E: Display + Send> From<E> for cache::Error<E> {
    fn from(err: E) -> Self {
        Self::Backend(err)
    }
}

/// Provides methods to add, replace or delete data in the cache
///
/// This is for adding support for a backend, users of the cache itself only
/// need the methods in [`super::Cache`]
///
/// # Uniqueness
///
/// Unless documented otherwise, only the main `id` field is unique, if there's
/// other fields that are also unique, they will be documented
///
/// None of the upsert methods should return an error on a conflict, unless
/// documented otherwise, they should delete the old value and insert
/// the new one (or replace all fields with their new values)
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
/// use sparkle_cache::backend::Backend;
/// use twilight_model::id::{
///     marker::{GuildMarker, UserMarker},
///     Id,
/// };
///
/// struct MyCache {
///     pub db: sql_library::Database, // Or add a getter method instead of making the field public
/// };
///
/// impl MyCache {
///     fn new() {
///         let db = sql_library::Database::connect("postgresql://localhost/discord");
///         db.query("CREATE UNIQUE INDEX channels_idx ON channels (channel_id);");
///         db.query("CREATE INDEX channels_guild_id_idx ON channels (guild_id);");
///     }
/// }
///
/// impl Backend for MyCache {
///     type Error = sqlx::Error;
///
///     async fn upsert_channel(&self, channel: CachedChannel) -> Result<(), Self::Error> {
///         sqlx::query!(
///             channel.id,
///             // Other fields here
///             "INSERT INTO channels (id, ...) VALUES ($1, ...)"
///         ).exec(&self.db)?;
///         Ok(())
///     }
///     // Implement other methods similarly
/// }
///
/// impl Cache for MyCache {
///     // Implement the methods here, usually using getter queries
/// }
/// ```
#[async_trait]
pub trait Backend {
    /// The error type the backend returns, for example `sqlx::Error`
    type Error: Display + Send;

    /// Set or replace the current user information of the bot
    async fn set_current_user(&self, current_user: CurrentUser) -> Result<(), Self::Error>;

    /// Add or replace a channel in the cache
    async fn upsert_channel(&self, channel: CachedChannel) -> Result<(), Self::Error>;

    /// Remove a channel from the cache
    async fn delete_channel(&self, channel_id: Id<ChannelMarker>) -> Result<(), Self::Error>;

    /// Remove a guild's channels from the cache
    ///
    /// This should be something like `DELETE FROM channels WHERE guild_id = ?`
    async fn delete_guild_channels(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

    /// Add a permission overwrite to the cache
    ///
    /// None of the fields in this type is unique
    async fn upsert_permission_overwrite(
        &self,
        permission_overwrite: CachedPermissionOverwrite,
    ) -> Result<(), Self::Error>;

    /// Remove a channel's permission overwrites from the cache
    ///
    /// This should be something like `DELETE FROM channel_overwrites WHERE
    /// channel_id = ?`
    async fn delete_channel_permission_overwrites(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Result<(), Self::Error>;

    /// Add a DM channel to the cache
    ///
    /// Both of the IDs in this type are unique
    async fn upsert_private_channel(
        &self,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error>;

    /// Add or replace a message in the cache
    async fn upsert_message(&self, message: CachedMessage) -> Result<(), Self::Error>;

    /// Remove a message from the cache
    async fn delete_message(&self, message_id: Id<MessageMarker>) -> Result<(), Self::Error>;

    /// Add an embed to the cache
    async fn upsert_embed(&self, embed: CachedEmbed) -> Result<(), Self::Error>;

    /// Remove an embed from the cache
    async fn delete_embed(&self, embed_id: Id<GenericMarker>) -> Result<(), Self::Error>;

    /// Add an embed field to the cache
    ///
    /// None of the fields in this type is unique
    async fn upsert_embed_field(&self, embed_field: CachedEmbedField) -> Result<(), Self::Error>;

    /// Remove an embed's fields from the cache
    ///
    /// This should be something like `DELETE FROM embed_fields WHERE embed_id =
    /// ?`
    async fn delete_embed_fields(&self, embed_id: Id<GenericMarker>) -> Result<(), Self::Error>;

    /// Get embeds of a message by its ID
    ///
    /// This method is used internally in [`super::Cache::embeds`]
    async fn select_message_embeds(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedEmbed>, Self::Error>;

    /// Get fields of an embed by its ID
    ///
    /// This method is used internally in [`super::Cache::embeds`]
    async fn select_embed_fields(
        &self,
        embed_id: Id<GenericMarker>,
    ) -> Result<Vec<CachedEmbedField>, Self::Error>;

    /// Add an attachment to the cache
    async fn upsert_attachment(&self, attachment: CachedAttachment) -> Result<(), Self::Error>;

    /// Remove a message's attachments from the cache
    ///
    /// This should be something like `DELETE FROM attachments WHERE message_id
    /// = ?`
    async fn delete_message_attachments(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<(), Self::Error>;

    /// Add a reaction to the cache
    ///
    /// Only the combination of message ID, user ID and emoji is unique, they're
    /// not unique on their own
    async fn upsert_reaction(&self, reaction: CachedReaction) -> Result<(), Self::Error>;

    /// Remove a reaction from the cache
    async fn delete_reaction(
        &self,
        message_id: Id<MessageMarker>,
        user_id: Id<UserMarker>,
        emoji: String,
    ) -> Result<(), Self::Error>;

    /// Remove a message's reactions of the given emoji from the cache
    ///
    /// This should be something like `DELETE FROM reactions WHERE message_id =
    /// ? AND emoji = ?`
    async fn delete_message_reactions_by_emoji(
        &self,
        message_id: Id<MessageMarker>,
        emoji: String,
    ) -> Result<(), Self::Error>;

    /// Remove a message's reactions from the cache
    ///
    /// This should be something like `DELETE FROM reactions WHERE message_id =
    /// ?`
    async fn delete_message_reactions(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<(), Self::Error>;

    /// Add or replace a member in the cache
    ///
    /// Only the combination of guild ID and user ID is unique, they're not
    /// unique on their own
    async fn upsert_member(&self, member: CachedMember) -> Result<(), Self::Error>;

    /// Remove a member from the cache
    async fn delete_member(
        &self,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Result<(), Self::Error>;

    /// Remove a guild's members from the cache
    ///
    /// This should be something like `DELETE FROM members WHERE guild_id = ?`
    async fn delete_guild_members(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

    /// Add or replace a presence in the cache
    ///
    /// Only the combination of guild ID and user ID is unique, they're not
    /// unique on their own
    async fn upsert_presence(&self, presence: CachedPresence) -> Result<(), Self::Error>;

    /// Remove a presence from the cache
    async fn delete_presence(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error>;

    /// Remove a guild's presences from the cache
    ///
    /// This should be something like `DELETE FROM presences WHERE guild_id = ?`
    async fn delete_guild_presences(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

    /// Add an activity to the cache
    ///
    /// None of the fields in this type is unique
    async fn upsert_activity(&self, activity: CachedActivity) -> Result<(), Self::Error>;

    /// Remove a user's activities from the cache
    ///
    /// This should be something like `DELETE FROM activities WHERE guild_id = ?
    /// AND user_id = ?`
    async fn delete_user_activities(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error>;

    /// Add or replace a guild in the cache
    async fn upsert_guild(&self, guild: CachedGuild) -> Result<(), Self::Error>;

    /// Remove a channel from the cache
    async fn delete_guild(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

    /// Add or update a role to the cache
    ///
    /// Only the combination of role ID and user ID is unique, they're not
    /// unique on their own
    ///
    /// When updating roles, make sure not to update the user ID field
    async fn upsert_role(&self, role: CachedRole) -> Result<(), Self::Error>;

    /// Remove a role from the cache
    async fn delete_role(&self, role_id: Id<RoleMarker>) -> Result<(), Self::Error>;

    /// Remove a guild's roles from the cache
    ///
    /// This should be something like `DELETE FROM roles WHERE guild_id = ?`
    async fn delete_guild_roles(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

    /// Remove a member's roles from the cache
    ///
    /// This should be something like `DELETE FROM roles WHERE guild_id = ? AND
    /// user_id = ?`
    async fn delete_member_roles(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error>;

    /// Add or replace an emoji in the cache
    async fn upsert_emoji(&self, emoji: CachedEmoji) -> Result<(), Self::Error>;

    /// Remove an emoji from the cache
    async fn delete_emoji(&self, emoji_id: Id<EmojiMarker>) -> Result<(), Self::Error>;

    /// Remove a guild's emojis from the cache
    ///
    /// This should be something like `DELETE FROM emojis WHERE guild_id = ?`
    async fn delete_guild_emojis(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

    /// Add or replace a sticker in the cache
    ///
    /// When updating stickers, make sure not to update the message ID field
    async fn upsert_sticker(&self, sticker: CachedSticker) -> Result<(), Self::Error>;

    /// Remove a message's stickers from the cache
    ///
    /// This should be something like `DELETE FROM stickers WHERE
    /// message_id = ?`
    async fn delete_message_stickers(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<(), Self::Error>;

    /// Remove a guild's stickers from the cache
    ///
    /// This should be something like `DELETE FROM stickers WHERE guild_id = ?
    /// AND message_id IS NULL`
    async fn delete_guild_stickers(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

    /// Add or replace a stage instance in the cache
    async fn upsert_stage_instance(&self, stage: StageInstance) -> Result<(), Self::Error>;

    /// Remove a stage instance from the cache
    async fn delete_stage_instance(&self, stage_id: Id<StageMarker>) -> Result<(), Self::Error>;

    /// Remove a guild's stage instance from the cache
    ///
    /// This should be something like `DELETE FROM stage_instances WHERE
    /// guild_id = ?`
    async fn delete_guild_stage_instances(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<(), Self::Error>;
}
