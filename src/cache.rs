use async_trait::async_trait;
use thiserror::Error;
use twilight_model::{
    channel::{Channel, ChannelType},
    gateway::event::Event,
    guild::auto_moderation::AutoModerationRule,
    id::{
        marker::{AutoModerationRuleMarker, ChannelMarker, UserMarker},
        Id,
    },
    user::CurrentUser,
};

use crate::{backend::Backend, model::CachedChannel};

/// The errors the cache might return
#[derive(Error, Debug)]
pub enum Error<B: Backend> {
    /// An error was returned by the backend
    #[error("An error was returned by the backend:\n{0}")]
    Backend(B::Error),
    /// The DM channel doesn't have any recipients other than the bot itself
    #[error("The DM channel doesn't have any recipients other than the bot itself:\n{0:?}")]
    PrivateChannelMissingRecipient(Channel),
}

/// Provides methods to update the cache and get data from it
///
/// This is for the users of the cache
#[async_trait]
pub trait Cache: Backend
where
    Error<Self>: From<<Self as Backend>::Error>,
{
    /// Update the cache with the given event, should be called for every event
    /// to keep the cache valid
    ///
    /// # Clones
    ///
    /// Many events don't require the event to be cloned, so the event parameter
    /// is taken by a reference, if an event does require a clone (usually
    /// add and update events), it will clone the required data implicitly
    ///
    /// # Errors
    ///
    /// Returns the error the backend might return
    async fn update(&self, event: &Event) -> Result<(), Error<Self>> {
        match event {
            Event::AutoModerationRuleCreate(rule) => {
                self.upsert_auto_moderation_rule((*rule.clone()).0).await?;
            }
            Event::AutoModerationRuleUpdate(rule) => {
                self.upsert_auto_moderation_rule((*rule.clone()).0).await?;
            }
            Event::AutoModerationRuleDelete(rule) => {
                self.remove_auto_moderation_rule(rule.id).await?;
            }
            Event::BanAdd(ban) => {
                self.add_ban(ban.guild_id, ban.user.id).await?;
            }
            Event::BanRemove(ban) => {
                self.remove_ban(ban.guild_id, ban.user.id).await?;
            }
            Event::ChannelCreate(channel) => {
                if channel.kind == ChannelType::Private {
                    let recipient_user_id =
                        private_channel_recipient(self.current_user().await?.id, channel)?;
                    self.add_private_channel(channel.id, recipient_user_id)
                        .await?;
                } else {
                    self.upsert_channel((&(*channel).0).into()).await?;
                }
            }
            Event::ChannelDelete(channel) => {
                if channel.kind == ChannelType::Private {
                    let recipient_user_id =
                        private_channel_recipient(self.current_user().await?.id, channel)?;
                    self.remove_private_channel(channel.id, recipient_user_id)
                        .await?;
                } else {
                    self.remove_channel(channel.id).await?;
                }
            }
            Event::ChannelUpdate(channel) => {
                if channel.kind != ChannelType::Private {
                    self.upsert_channel((&(*channel).0).into()).await?;
                }
            }
            // Event::GuildDelete(_) => {}
            // Event::GuildCreate(_) => {}
            // Event::GuildEmojisUpdate(_) => {}
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
    async fn current_user(&self) -> Result<CurrentUser, Error<Self>>;

    /// Get a cached channel by its ID
    async fn channel(&self, channel_id: Id<ChannelMarker>) -> Result<CachedChannel, Error<Self>>;

    /// Get a DM channel's ID by its recipient's ID
    async fn private_channel(
        &self,
        recipient_id: Id<UserMarker>,
    ) -> Result<Id<ChannelMarker>, Error<Self>>;

    /// Get an auto moderation rule by its ID
    async fn auto_moderation_rule(
        &self,
        rule_id: Id<AutoModerationRuleMarker>,
    ) -> Result<AutoModerationRule, Error<Self>>;
}

/// Given a [`twilight_model::channel::ChannelType::Private`] returns the first
/// recipient's ID that's not the current user
///
/// # Errors
///
/// Returns [`Error::PrivateChannelMissingRecipient`], also clones the channel
/// to create the error
fn private_channel_recipient<T: Backend>(
    current_user_id: Id<UserMarker>,
    channel: &Channel,
) -> Result<Id<UserMarker>, Error<T>> {
    let recipient_user_id = channel
        .recipients
        .as_ref()
        .and_then(|recipients| recipients.iter().find(|user| user.id == current_user_id))
        .ok_or_else(|| Error::PrivateChannelMissingRecipient(channel.clone()))?
        .id;
    Ok(recipient_user_id)
}
