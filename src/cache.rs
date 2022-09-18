use async_trait::async_trait;
use thiserror::Error;
use twilight_model::{
    gateway::event::Event,
    guild::auto_moderation::AutoModerationRule,
    id::{
        marker::{AutoModerationRuleMarker, UserMarker},
        Id,
    },
    user::CurrentUser,
};

use crate::backend::Backend;

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
    /// # Warnings
    ///
    /// Channel's `last_message_id` is invalid as it would mean updating the
    /// channel in the cache for every message, to get a channel's last message,
    /// use [`self::channel_messages`]
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
            // Event::BanAdd(_) => {}
            // Event::BanRemove(_) => {}
            // Event::ChannelCreate(_) => {}
            // Event::ChannelDelete(_) => {}
            // Event::ChannelPinsUpdate(_) => {}
            // Event::ChannelUpdate(_) => {}
            // Event::CommandPermissionsUpdate(_) => {}
            // Event::GatewayHeartbeat(_) => {}
            // Event::GatewayHeartbeatAck => {}
            // Event::GatewayHello(_) => {}
            // Event::GatewayInvalidateSession(_) => {}
            // Event::GatewayReconnect => {}
            // Event::GiftCodeUpdate => {}
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

    /// Get an auto moderation rule by its ID
    async fn auto_moderation_rule(
        &self,
        rule_id: Id<AutoModerationRuleMarker>,
    ) -> Result<AutoModerationRule, Self::Error>;
}
