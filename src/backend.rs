use std::fmt::Display;

use async_trait::async_trait;
use twilight_model::{
    channel::Channel,
    guild::auto_moderation::AutoModerationRule,
    id::{
        marker::{AutoModerationRuleMarker, ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    user::CurrentUser,
};

use crate::cache;

/// Provides methods to add or replace data in the cache
///
/// This is for adding support for a backend, users of the cache itself only
/// need the methods in [`super::cache::Cache`]
///
/// # This trait is not complete
///
/// You should also add a method to expose the backend so that users can filter
/// the results in the query, for example `SELECT *  FROM users WHERE name = ?`
///
/// You should also implement your backend library's traits to (de)serialize
/// Discord models for the backend
#[async_trait]
pub trait Backend: Sized {
    /// The error the cache's backend could return
    type Error: Display + From<cache::Error<Self>>;

    /// Set the current user information of the bot
    async fn set_current_user(&self, current_user: CurrentUser) -> Result<(), Self::Error>;

    /// Add or replace an auto moderation rule in the cache
    async fn upsert_auto_moderation_rule(
        &self,
        rule: AutoModerationRule,
    ) -> Result<(), Self::Error>;

    /// Remove an auto moderation rule from the cache
    async fn remove_auto_moderation_rule(
        &self,
        rule_id: Id<AutoModerationRuleMarker>,
    ) -> Result<(), Self::Error>;

    /// Add a banned user to the cache
    async fn add_ban(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error>;

    /// Remove a banned user from the cache
    async fn remove_ban(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error>;

    /// Add or replace a channel in the cache
    async fn upsert_channel(&self, channel: Channel) -> Result<(), Self::Error>;

    /// Remove a channel from the cache
    async fn remove_channel(&self, channel_id: Id<ChannelMarker>) -> Result<(), Self::Error>;

    /// Add a DM channel to the cache
    ///
    /// This is different from a guild channel because it only has a channel ID
    /// and recipient user ID fields
    async fn add_private_channel(
        &self,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error>;

    /// Remove a DM channel from the cache
    async fn remove_private_channel(
        &self,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(), Self::Error>;
}
