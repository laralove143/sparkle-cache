use async_trait::async_trait;
pub use error::Error;
use twilight_model::{
    channel::{Channel, ChannelType},
    gateway::event::Event,
    guild::Emoji,
    id::{
        marker::{
            ChannelMarker, EmojiMarker, GenericMarker, GuildMarker, MessageMarker, StickerMarker,
            UserMarker,
        },
        Id,
    },
    user::CurrentUser,
};

use crate::{
    model::{
        CachedAttachment, CachedChannel, CachedEmbed, CachedEmbedField, CachedEmoji, CachedGuild,
        CachedMember, CachedMessage, CachedMessageSticker, CachedReaction, CachedSticker,
    },
    Backend,
};

/// Put into a mod to allow lints
#[allow(clippy::std_instead_of_core)]
mod error {
    use thiserror::Error;
    use twilight_model::channel::Channel;

    use crate::backend;

    #[derive(Error, Debug)]
    /// The errors the cache might return
    pub enum Error<E: backend::Error> {
        /// An error was returned by the backend
        #[error("An error was returned by the backend:\n{0}")]
        Backend(E),
        /// The DM channel doesn't have any recipients other than the bot itself
        #[error("The DM channel doesn't have any recipients other than the bot itself:\n{0:?}")]
        PrivateChannelMissingRecipient(Box<Channel>),
        /// The current user isn't in the cache
        #[error("The current user isn't in the cache")]
        CurrentUserMissing,
    }
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
    #[allow(clippy::too_many_lines)]
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
                for sticker in &guild.stickers {
                    self.upsert_sticker(sticker.into()).await?;
                }
                for member in &guild.members {
                    self.upsert_member(member.into()).await?;
                }
                self.upsert_guild(CachedGuild::from(&guild.0)).await?;
            }
            Event::GuildUpdate(guild) => {
                if let Some(mut cached_guild) = self.guild(guild.id).await? {
                    cached_guild.update(guild);
                    self.upsert_guild(cached_guild).await?;
                }
            }
            Event::GuildDelete(guild) => {
                if !guild.unavailable {
                    self.delete_guild_channels(guild.id).await?;
                    self.delete_guild_emojis(guild.id).await?;
                    self.delete_guild_stickers(guild.id).await?;
                    self.delete_guild_members(guild.id).await?;
                    self.delete_guild(guild.id).await?;
                }
            }
            Event::GuildEmojisUpdate(emojis) => {
                self.delete_guild_emojis(emojis.guild_id).await?;
                for emoji in &emojis.emojis {
                    self.add_emoji(emoji, emojis.guild_id).await?;
                }
            }
            Event::GuildStickersUpdate(stickers) => {
                self.delete_guild_stickers(stickers.guild_id).await?;
                for sticker in &stickers.stickers {
                    self.upsert_sticker(sticker.into()).await?;
                }
            }
            Event::MemberAdd(member) => {
                self.upsert_member(CachedMember::from(&member.0)).await?;
            }
            Event::MemberUpdate(member) => {
                if let Some(mut cached_member) =
                    self.member(member.guild_id, member.user.id).await?
                {
                    cached_member.update(member);
                    self.upsert_member(cached_member).await?;
                }
            }
            Event::MemberChunk(members) => {
                for member in &members.members {
                    self.upsert_member(member.into()).await?;
                }
            }
            Event::MemberRemove(member) => {
                self.delete_member(member.user.id, member.guild_id).await?;
            }
            Event::MessageCreate(message) => {
                for attachment in message.attachments.clone() {
                    self.upsert_attachment(CachedAttachment::from_attachment(
                        attachment, message.id,
                    ))
                    .await?;
                }
                for reaction in message.reactions.clone() {
                    self.upsert_reaction(CachedReaction::from_reaction(reaction, message.id))
                        .await?;
                }
                for sticker in message.sticker_items.clone() {
                    self.upsert_message_sticker(CachedMessageSticker::from_sticker(
                        sticker, message.id,
                    ))
                    .await?;
                }
                for embed in message.embeds.clone() {
                    let fields = embed.fields.clone();
                    let cached_embed = CachedEmbed::from_embed(embed, message.id);
                    for field in fields {
                        self.upsert_embed_field(CachedEmbedField::from_embed_field(
                            field,
                            cached_embed.id,
                        ))
                        .await?;
                    }
                    self.upsert_embed(cached_embed).await?;
                }
                self.upsert_message(CachedMessage::from(&message.0)).await?;
            }
            Event::MessageUpdate(message) => {
                if let Some(mut cached_message) = self.message(message.id).await? {
                    cached_message.update(message);
                    if let Some(attachments) = &message.attachments {
                        for attachment in attachments.clone() {
                            self.upsert_attachment(CachedAttachment::from_attachment(
                                attachment, message.id,
                            ))
                            .await?;
                        }
                    }
                    if let Some(embeds) = &message.embeds {
                        for embed in embeds.clone() {
                            let fields = embed.fields.clone();
                            let cached_embed = CachedEmbed::from_embed(embed, message.id);
                            for field in fields {
                                self.upsert_embed_field(CachedEmbedField::from_embed_field(
                                    field,
                                    cached_embed.id,
                                ))
                                .await?;
                            }
                            self.upsert_embed(cached_embed).await?;
                        }
                    }
                    self.upsert_message(cached_message).await?;
                }
            }
            Event::MessageDelete(message) => {
                self.remove_message(message.id).await?;
            }
            Event::MessageDeleteBulk(messages) => {
                for message_id in &messages.ids {
                    self.remove_message(*message_id).await?;
                }
            }
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

    /// Get a cached guild by its ID
    async fn guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Option<CachedGuild>, Error<Self::Error>>;

    /// Get a cached emoji by its ID
    async fn emoji(
        &self,
        emoji_id: Id<EmojiMarker>,
    ) -> Result<Option<CachedEmoji>, Error<Self::Error>>;

    /// Get a cached sticker by its ID
    async fn sticker(
        &self,
        sticker_id: Id<StickerMarker>,
    ) -> Result<Option<CachedSticker>, Error<Self::Error>>;

    /// Get a cached member by its guild ID and user ID
    async fn member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<Option<CachedMember>, Error<Self::Error>>;

    /// Get a cached message by its ID
    async fn message(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Option<CachedMessage>, Error<Self::Error>>;

    /// Get embeds of a message by its ID
    async fn embeds(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedEmbed>, Error<Self::Error>>;

    /// Get fields of an embed by its ID
    async fn embed_fields(
        &self,
        embed_id: Id<GenericMarker>,
    ) -> Result<Vec<CachedEmbedField>, Error<Self::Error>>;

    /// Get attachments of a message by its ID
    async fn attachments(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedAttachment>, Error<Self::Error>>;

    /// Get reactions of a message by its ID
    async fn reactions(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedReaction>, Error<Self::Error>>;

    /// Get stickers of a message by its ID
    async fn stickers(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedMessageSticker>, Error<Self::Error>>;

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
    /// Returns [`Error::PrivateChannelMissingRecipient`]
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
            .ok_or_else(|| Error::PrivateChannelMissingRecipient(Box::new(channel.clone())))?
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

    /// Removes the message from the cache
    #[doc(hidden)]
    async fn remove_message(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<(), Error<Self::Error>> {
        let embeds = self.embeds(message_id).await?;
        for embed in embeds {
            self.delete_embed_fields(embed.id).await?;
            self.delete_embed(embed.id).await?;
        }
        self.delete_message_attachments(message_id).await?;
        self.delete_message_reactions(message_id).await?;
        self.delete_message_stickers(message_id).await?;
        self.delete_message(message_id).await?;
        Ok(())
    }
}
