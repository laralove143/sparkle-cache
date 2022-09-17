use async_trait::async_trait;
use twilight_model::{
    gateway::event::Event,
    guild::auto_moderation::AutoModerationRule,
    id::{
        marker::{AutoModerationRuleMarker, GuildMarker},
        Id,
    },
};

use crate::{backend::Backend, model::CachedGuild};

/// Provides methods to update the cache and get data from it
///
/// This is for the users of the cache
#[async_trait]
pub trait Cache: Backend {
    /// Update the cache with the given event, should be called for every event
    /// to keep the cache valid
    ///
    /// # Errors
    ///
    /// Returns the error the backend might return
    async fn update(&self, event: &Event) -> Result<(), Self::Error> {
        match event {
            Event::AutoModerationRuleCreate(rule) => {
                let mut guild = self.guild(rule.guild_id).await?;
                guild.auto_moderation_rules.push(rule.id);
                self.upsert_guild(guild).await?;
                self.upsert_auto_moderation_rule((*rule.clone()).0).await?;
            }
            Event::AutoModerationRuleUpdate(rule) => {
                let mut guild = self.guild(rule.guild_id).await?;
                guild.auto_moderation_rules.push(rule.id);
                self.upsert_guild(guild).await?;
                self.upsert_auto_moderation_rule((*rule.clone()).0).await?;
            }
            Event::AutoModerationRuleDelete(rule) => {
                let mut guild = self.guild(rule.guild_id).await?;
                guild
                    .auto_moderation_rules
                    .retain(|rule_id| *rule_id != rule.id);
                self.upsert_guild(guild).await?;
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

    /// Get a cached guild by its ID
    async fn guild(&self, guild_id: Id<GuildMarker>) -> Result<CachedGuild, Self::Error>;

    /// Get an auto moderation rule by its ID
    async fn auto_moderation_rule(
        &self,
        rule_id: Id<AutoModerationRuleMarker>,
    ) -> Result<AutoModerationRule, Self::Error>;

    /// Get a cached guild's auto moderation rules
    async fn guild_auto_moderation_rules(
        &self,
        guild: CachedGuild,
    ) -> Result<Vec<AutoModerationRule>, Self::Error> {
        let mut rules = vec![];
        for rule_id in guild.auto_moderation_rules {
            rules.push(self.auto_moderation_rule(rule_id).await?);
        }
        Ok(rules)
    }
}
