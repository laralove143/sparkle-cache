use async_trait::async_trait;
use twilight_model::{
    guild::auto_moderation::AutoModerationRule,
    id::{marker::AutoModerationRuleMarker, Id},
};

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
}
