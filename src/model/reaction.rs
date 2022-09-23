use twilight_model::{
    channel::{Reaction, ReactionType},
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
};

/// A cached reaction, it is the same as
/// [`twilight_model::channel::Reaction`] except:
///
/// - [`twilight_model::channel::Reaction.member`] is removed as they're cached
///   separately
#[derive(Clone, Debug)]
pub struct CachedReaction {
    pub channel_id: Id<ChannelMarker>,
    pub emoji: ReactionType,
    pub guild_id: Option<Id<GuildMarker>>,
    pub message_id: Id<MessageMarker>,
    pub user_id: Id<UserMarker>,
}

impl From<&Reaction> for CachedReaction {
    fn from(reaction: &Reaction) -> Self {
        Self {
            channel_id: reaction.channel_id,
            emoji: reaction.emoji.clone(),
            guild_id: reaction.guild_id,
            message_id: reaction.message_id,
            user_id: reaction.user_id,
        }
    }
}
