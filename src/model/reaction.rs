use twilight_model::{
    channel::{Reaction, ReactionType},
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
};

/// A cached reaction
///
/// It's the same as [`twilight_model::channel::Reaction`] except:
///
/// - `member` field is removed, as members are cached separately
///
/// - `emoji` field is changed to a string that is either the ID or the name of
///   the emoji
#[derive(Clone, Debug)]
pub struct CachedReaction {
    pub channel_id: Id<ChannelMarker>,
    pub emoji: String,
    pub guild_id: Option<Id<GuildMarker>>,
    pub message_id: Id<MessageMarker>,
    pub user_id: Id<UserMarker>,
}

impl From<&Reaction> for CachedReaction {
    fn from(reaction: &Reaction) -> Self {
        Self {
            channel_id: reaction.channel_id,
            emoji: match &reaction.emoji {
                ReactionType::Custom { id, .. } => id.to_string(),
                ReactionType::Unicode { name } => name.clone(),
            },
            guild_id: reaction.guild_id,
            message_id: reaction.message_id,
            user_id: reaction.user_id,
        }
    }
}
