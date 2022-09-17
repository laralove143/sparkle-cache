use twilight_model::id::{
    marker::{AutoModerationRuleMarker, GuildMarker},
    Id,
};

/// A guild with the IDs of all of its related data
#[derive(Clone, Debug)]
pub struct CachedGuild {
    /// The guild's ID
    pub id: Id<GuildMarker>,
    /// The IDs the auto moderation rules in the guild
    pub auto_moderation_rules: Vec<Id<AutoModerationRuleMarker>>,
}
