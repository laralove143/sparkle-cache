use std::fmt::Debug;

use async_trait::async_trait;
use thiserror::Error;
use twilight_model::{
    channel::{Channel, ChannelType},
    gateway::event::Event,
    guild::{auto_moderation::AutoModerationRule, Emoji},
    id::{
        marker::{AutoModerationRuleMarker, ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    user::CurrentUser,
};

use crate::{
    backend,
    model::{CachedChannel, CachedEmoji, CachedGuild},
    Backend,
};

/// The errors the cache might return
#[derive(Error, Debug)]
pub enum Error<E: backend::Error> {
    /// An error was returned by the backend
    #[error("An error was returned by the backend:\n{0}")]
    Backend(E),
    /// The DM channel doesn't have any recipients other than the bot itself
    #[error("The DM channel doesn't have any recipients other than the bot itself:\n{0:?}")]
    PrivateChannelMissingRecipient(Channel),
    /// The current user isn't in the cache
    #[error("The current user isn't in the cache")]
    CurrentUserMissing,
}

/// Provides methods to update the cache and get data from it
///
/// This is for the users of the cache
///
/// # Example
///
/// ```ignore
/// use twilight_model::id::Id;
/// cache.update(&event);
/// let channel = cache.channel(Id::new(123)).await?.unwrap();
/// ```
#[async_trait]
pub trait Cache: Backend {
    /// Update the cache with the given event, should be called for every event
    /// to keep the cache valid
    ///
    /// # Clones
    ///
    /// Many events don't require the event to be cloned, so the event parameter
    /// is taken by a reference, if an event does require a clone (usually
    /// add and update events), it will clone the required fields
    ///
    /// # Errors
    ///
    /// Returns the error the backend might return
    ///
    /// On `ChannelCreate`, `ChannelUpdate` and `ChannelDelete` events when the
    /// channel is a DM channel, might return
    /// [`Error::PrivateChannelMissingRecipient`]
    async fn update(&self, event: &Event) -> Result<(), Error<Self::Error>> {
        match event {
            Event::ChannelCreate(channel) => {
                self.add_channel(channel).await?;
            }
            Event::ChannelUpdate(channel) => {
                self.add_channel(channel).await?;
            }
            Event::ChannelDelete(channel) => {
                self.delete_channel(channel.id).await?;
            }
            Event::GuildCreate(guild) => {
                for channel in &guild.channels {
                    self.add_channel(channel).await?;
                }
                for emoji in &guild.emojis {
                    self.add_emoji(emoji, guild.id).await?;
                }
                self.upsert_guild(CachedGuild::from(&guild.0)).await?;
            }
            Event::GuildDelete(guild) => {
                if !guild.unavailable {
                    self.delete_guild_channels(guild.id).await?;
                    self.delete_guild_emojis(guild.id).await?;
                }
                self.delete_guild(guild.id).await?;
            }
            Event::GuildEmojisUpdate(emojis) => {
                for emoji in &emojis.emojis {
                    self.add_emoji(emoji, emojis.guild_id).await?;
                }
            }
            // Event::GuildIntegrationsUpdate(_) => {}
            // Event::GuildScheduledEventCreate(_) => {}
            // Event::GuildScheduledEventDelete(_) => {}
            // Event::GuildScheduledEventUpdate(_) => {}
            // Event::GuildScheduledEventUserAdd(_) => {}
            // Event::GuildScheduledEventUserRemove(_) => {}
            // Event::GuildStickersUpdate(_) => {}
            // Event::GuildUpdate(_) => {}
            // Event::IntegrationCreate(_) => {}
            // Event::IntegrationDelete(_) => {}
            // Event::IntegrationUpdate(_) => {}
            // Event::InteractionCreate(_) => {}
            // Event::InviteCreate(_) => {}
            // Event::InviteDelete(_) => {}
            // Event::MemberAdd(_) => {}
            // Event::MemberRemove(_) => {}
            // Event::MemberUpdate(_) => {}
            // Event::MemberChunk(_) => {}
            // Event::MessageCreate(_) => {}
            // Event::MessageDelete(_) => {}
            // Event::MessageDeleteBulk(_) => {}
            // Event::MessageUpdate(_) => {}
            // Event::PresenceUpdate(_) => {}
            // Event::PresencesReplace => {}
            // Event::ReactionAdd(_) => {}
            // Event::ReactionRemove(_) => {}
            // Event::ReactionRemoveAll(_) => {}
            // Event::ReactionRemoveEmoji(_) => {}
            // Event::Ready(_) => {}
            // Event::Resumed => {}
            // Event::RoleCreate(_) => {}
            // Event::RoleDelete(_) => {}
            // Event::RoleUpdate(_) => {}
            // Event::ShardConnected(_) => {}
            // Event::ShardConnecting(_) => {}
            // Event::ShardDisconnected(_) => {}
            // Event::ShardIdentifying(_) => {}
            // Event::ShardReconnecting(_) => {}
            // Event::ShardPayload(_) => {}
            // Event::ShardResuming(_) => {}
            // Event::StageInstanceCreate(_) => {}
            // Event::StageInstanceDelete(_) => {}
            // Event::StageInstanceUpdate(_) => {}
            // Event::ThreadCreate(_) => {}
            // Event::ThreadDelete(_) => {}
            // Event::ThreadListSync(_) => {}
            // Event::ThreadMemberUpdate(_) => {}
            // Event::ThreadMembersUpdate(_) => {}
            // Event::ThreadUpdate(_) => {}
            // Event::TypingStart(_) => {}
            // Event::UnavailableGuild(_) => {}
            // Event::UserUpdate(_) => {}
            // Event::VoiceServerUpdate(_) => {}
            // Event::VoiceStateUpdate(_) => {}
            // Event::WebhooksUpdate(_) => {}
            _ => {}
        }

        Ok(())
    }

    /// Get the current user information of the bot
    ///
    /// # Errors
    ///
    /// Returns [`Error::CurrentUserMissing`] when called before the ready event
    /// is received
    async fn current_user(&self) -> Result<CurrentUser, Error<Self::Error>>;

    /// Get a cached channel by its ID
    async fn channel(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Result<Option<CachedChannel>, Error<Self::Error>>;

    /// Get a DM channel's ID by its recipient's ID
    async fn private_channel(
        &self,
        recipient_id: Id<UserMarker>,
    ) -> Result<Option<Id<ChannelMarker>>, Error<Self::Error>>;

    /// Get an auto moderation rule by its ID
    async fn auto_moderation_rule(
        &self,
        rule_id: Id<AutoModerationRuleMarker>,
    ) -> Result<Option<AutoModerationRule>, Error<Self::Error>>;

    /// Updates the cache with the channel
    ///
    /// # Errors
    ///
    /// Returns the error the backend might return
    ///
    /// When the channel is a DM channel, might return
    /// [`cache::Error::PrivateChannelMissingRecipient`]
    #[doc(hidden)]
    async fn add_channel(&self, channel: &Channel) -> Result<(), Error<Self::Error>> {
        if channel.kind == ChannelType::Private {
            let recipient_user_id = self.private_channel_recipient(channel).await?;
            self.upsert_private_channel(channel.id, recipient_user_id)
                .await?;
        } else {
            self.upsert_channel(CachedChannel::from(channel)).await?;
        }

        Ok(())
    }

    /// Removes the channel from the cache
    ///
    /// # Errors
    ///
    /// Returns the error the backend might return
    ///
    /// When the channel is a DM channel, might return
    /// [`cache::Error::PrivateChannelMissingRecipient`]
    #[doc(hidden)]
    async fn remove_channel(&self, channel: &Channel) -> Result<(), Error<Self::Error>> {
        if channel.kind == ChannelType::Private {
            let recipient_user_id = self.private_channel_recipient(channel).await?;
            self.delete_private_channel(channel.id, recipient_user_id)
                .await?;
        } else {
            self.delete_channel(channel.id).await?;
        }

        Ok(())
    }

    /// Given a [`twilight_model::channel::ChannelType::Private`] returns the
    /// first recipient's ID that's not the current user
    ///
    /// # Errors
    ///
    /// Returns [`Error::PrivateChannelMissingRecipient`], also clones the
    /// channel to create the error
    #[doc(hidden)]
    async fn private_channel_recipient(
        &self,
        channel: &Channel,
    ) -> Result<Id<UserMarker>, Error<Self::Error>> {
        let current_user_id = self.current_user().await?.id;
        let recipient_user_id = channel
            .recipients
            .as_ref()
            .and_then(|recipients| recipients.iter().find(|user| user.id == current_user_id))
            .ok_or_else(|| Error::PrivateChannelMissingRecipient(channel.clone()))?
            .id;
        Ok(recipient_user_id)
    }

    /// Adds the emoji to the cache
    #[doc(hidden)]
    async fn add_emoji(
        &self,
        emoji: &Emoji,
        guild_id: Id<GuildMarker>,
    ) -> Result<(), Error<Self::Error>> {
        self.upsert_emoji(CachedEmoji::from_emoji(emoji, guild_id))
            .await?;
        Ok(())
    }
}
