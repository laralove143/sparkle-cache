use async_trait::async_trait;
use twilight_model::{
    guild::auto_moderation::AutoModerationRule,
    id::{
        marker::{AutoModerationRuleMarker, GuildMarker},
        Id,
    },
};

use crate::model::CachedGuild;

/// Provides methods to add or replace data in the cache
///
/// This is for adding support for a backend
#[async_trait]
pub trait Backend {
    /// The error the cache's backend could return
    type Error;

    /// Add or replace a cached guild
    async fn upsert_guild(&self, guild: CachedGuild) -> Result<(), Self::Error>;

    /// Remove a cached guild
    async fn remove_guild(&self, guild_id: Id<GuildMarker>) -> Result<(), Self::Error>;

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
}
