use async_trait::async_trait;
pub use error::Error;
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, Channel, ReactionType, StageInstance},
    gateway::event::Event,
    guild::Permissions,
    id::{
        marker::{
            ChannelMarker, EmojiMarker, GuildMarker, MessageMarker, RoleMarker, StageMarker,
            StickerMarker, UserMarker,
        },
        Id,
    },
    user::CurrentUser,
};
use twilight_util::permission_calculator::PermissionCalculator;

use crate::{
    model::{
        CachedActivity, CachedAttachment, CachedChannel, CachedEmbed, CachedEmbedField,
        CachedEmoji, CachedGuild, CachedMember, CachedMessage, CachedPermissionOverwrite,
        CachedPresence, CachedReaction, CachedRole, CachedSticker,
    },
    Backend,
};

/// Put into a mod to allow lints
#[allow(clippy::std_instead_of_core)]
mod error {
    use thiserror::Error;
    use twilight_model::id::{
        marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id,
    };

    use crate::model::{CachedChannel, CachedMember};

    /// The errors the cache might return
    #[derive(Error, Debug)]
    pub enum Error<E: Send> {
        /// An error was returned by the backend
        #[error("An error was returned by the backend:\n{0}")]
        Backend(E),
        /// The current user isn't in the cache
        #[error("The current user isn't in the cache")]
        CurrentUserMissing,
        /// One of the roles of the member to cache isn't in the cache
        #[error(
            "One of the roles of the member to cache isn't in the cache:\nUser ID: {user_id}, \
             Role ID: {role_id}"
        )]
        MemberRoleMissing {
            /// The member's user ID
            user_id: Id<UserMarker>,
            /// The missing role's ID
            role_id: Id<RoleMarker>,
        },
        /// The timestamp the member's communication is disabled until isn't
        /// valid
        #[error("The timestamp the member's communication is disabled until isn't valid:\n{0:?}")]
        MemberBadTimeoutTimestamp(Box<CachedMember>),
        /// The channel to calculate permissions for isn't in the cache
        #[error("The channel to calculate permissions for isn't in the cache:\n{0}")]
        PermissionsChannelMissing(Id<ChannelMarker>),
        /// The guild to calculate permissions for isn't in the cache
        #[error("The guild to calculate permissions for isn't in the cache:\n{0}")]
        PermissionsGuildMissing(Id<GuildMarker>),
        /// The member to calculate permissions for isn't in the cache
        #[error(
            "The member to calculate permissions for isn't in the cache:\nUser ID: {user_id}, \
             Guild ID: {guild_id}"
        )]
        PermissionsMemberMissing {
            /// The member's user ID
            user_id: Id<UserMarker>,
            /// The guild's ID
            guild_id: Id<GuildMarker>,
        },
        /// The everyone role in the guild to calculate permissions for isn't in
        /// the cache
        #[error(
            "The everyone role in the guild to calculate permissions for isn't in the cache:\n{0}"
        )]
        PermissionsGuildEveryoneRoleMissing(Id<GuildMarker>),
        /// The given channel to calculate permissions for doesn't have a guild
        /// ID
        #[error("The given channel to calculate permissions for doesn't have a guild ID:\n{0:?}")]
        PermissionsChannelNotInGuild(Box<CachedChannel>),
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
    // noinspection DuplicatedCode
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
    /// On [`twilight_model::gateway::event::Event::ChannelCreate`],
    /// [`twilight_model::gateway::event::Event::ChannelUpdate`] and
    /// [`twilight_model::gateway::event::Event::ChannelDelete`], events when
    /// the channel is a DM channel, might return
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
                self.delete_channel_permission_overwrites(channel.id)
                    .await?;
                self.delete_channel(channel.id).await?;
            }
            Event::ThreadCreate(thread) => {
                self.add_channel(thread).await?;
            }
            Event::ThreadUpdate(thread) => {
                self.add_channel(thread).await?;
            }
            Event::ThreadDelete(thread) => {
                self.delete_channel_permission_overwrites(thread.id).await?;
                self.delete_channel(thread.id).await?;
            }
            Event::GuildCreate(guild) => {
                for channel in guild.channels.iter().chain(&guild.threads) {
                    self.add_channel(channel).await?;
                }
                for emoji in &guild.emojis {
                    self.upsert_emoji(CachedEmoji::from_emoji(emoji, guild.id))
                        .await?;
                }
                for sticker in &guild.stickers {
                    self.upsert_sticker(sticker.into()).await?;
                }
                for member in &guild.members {
                    self.add_member_roles(member.user.id, member.roles.clone())
                        .await?;
                    self.upsert_member(member.into()).await?;
                }
                for presence in &guild.presences {
                    self.upsert_presence(presence.into()).await?;
                }
                for role in &guild.roles {
                    self.upsert_role(CachedRole::from_role(role.clone(), guild.id))
                        .await?;
                }
                for stage in &guild.stage_instances {
                    self.upsert_stage_instance(stage.clone()).await?;
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
                    for channel in self.guild_channels(guild.id).await? {
                        self.delete_channel_permission_overwrites(channel.id)
                            .await?;
                    }
                    self.delete_guild_channels(guild.id).await?;
                    self.delete_guild_emojis(guild.id).await?;
                    self.delete_guild_stickers(guild.id).await?;
                    self.delete_guild_members(guild.id).await?;
                    self.delete_guild_presences(guild.id).await?;
                    self.delete_guild_roles(guild.id).await?;
                    self.delete_guild_stage_instances(guild.id).await?;
                    self.delete_guild(guild.id).await?;
                }
            }
            Event::GuildEmojisUpdate(emojis) => {
                self.delete_guild_emojis(emojis.guild_id).await?;
                for emoji in &emojis.emojis {
                    self.upsert_emoji(CachedEmoji::from_emoji(emoji, emojis.guild_id))
                        .await?;
                }
            }
            Event::GuildStickersUpdate(stickers) => {
                self.delete_guild_stickers(stickers.guild_id).await?;
                for sticker in &stickers.stickers {
                    self.upsert_sticker(sticker.into()).await?;
                }
            }
            Event::MemberAdd(member) => {
                self.add_member_roles(member.user.id, member.roles.clone())
                    .await?;
                self.upsert_member(CachedMember::from(&member.0)).await?;
            }
            Event::MemberChunk(members) => {
                for member in &members.members {
                    self.add_member_roles(member.user.id, member.roles.clone())
                        .await?;
                    self.upsert_member(member.into()).await?;
                }
            }
            Event::MemberUpdate(member) => {
                if let Some(mut cached_member) =
                    self.member(member.user.id, member.guild_id).await?
                {
                    cached_member.update(member);
                    self.upsert_member(cached_member).await?;
                    self.delete_member_roles(member.guild_id, member.user.id)
                        .await?;
                    self.add_member_roles(member.user.id, member.roles.clone())
                        .await?;
                }
            }
            Event::MemberRemove(member) => {
                self.delete_member(member.user.id, member.guild_id).await?;
                self.delete_member_roles(member.guild_id, member.user.id)
                    .await?;
            }
            Event::MessageCreate(message) => {
                for attachment in message.attachments.clone() {
                    self.upsert_attachment(CachedAttachment::from_attachment(
                        attachment, message.id,
                    ))
                    .await?;
                }
                for message_sticker in message.sticker_items.clone() {
                    let sticker =
                        if let Some(mut cached_sticker) = self.sticker(message_sticker.id).await? {
                            cached_sticker.message_id = Some(message.id);
                            cached_sticker
                        } else {
                            CachedSticker::from_message_sticker(message_sticker, message.id)
                        };
                    self.upsert_sticker(sticker).await?;
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
                        self.delete_message_attachments(message.id).await?;
                        for attachment in attachments.clone() {
                            self.upsert_attachment(CachedAttachment::from_attachment(
                                attachment, message.id,
                            ))
                            .await?;
                        }
                    }
                    if let Some(embeds) = &message.embeds {
                        let cached_embeds = self.embeds(message.id).await?;
                        for (embed, _) in cached_embeds {
                            self.delete_embed_fields(embed.id).await?;
                            self.delete_embed(embed.id).await?;
                        }
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
            Event::PresenceUpdate(presence) => {
                self.delete_user_activities(presence.guild_id, presence.user.id())
                    .await?;
                for activity in &presence.activities {
                    self.upsert_activity(CachedActivity::from_activity(
                        activity,
                        presence.user.id(),
                        presence.guild_id,
                    ))
                    .await?;
                }
                self.upsert_presence(CachedPresence::from(&presence.0))
                    .await?;
            }
            Event::ReactionAdd(reaction) => {
                self.upsert_reaction(CachedReaction::from(&reaction.0))
                    .await?;
            }
            Event::ReactionRemove(reaction) => {
                self.delete_reaction(
                    reaction.message_id,
                    reaction.user_id,
                    match &reaction.emoji {
                        ReactionType::Custom { id, .. } => id.to_string(),
                        ReactionType::Unicode { name } => name.clone(),
                    },
                )
                .await?;
            }
            Event::ReactionRemoveEmoji(reaction) => {
                self.delete_message_reactions_by_emoji(
                    reaction.message_id,
                    match &reaction.emoji {
                        ReactionType::Custom { id, .. } => id.to_string(),
                        ReactionType::Unicode { name } => name.clone(),
                    },
                )
                .await?;
            }
            Event::ReactionRemoveAll(reaction) => {
                self.delete_message_reactions(reaction.message_id).await?;
            }
            Event::Ready(ready) => {
                self.set_current_user(ready.user.clone()).await?;
            }
            Event::UserUpdate(user) => {
                self.set_current_user(user.0.clone()).await?;
            }
            Event::RoleCreate(role) => {
                self.upsert_role(CachedRole::from_role(role.role.clone(), role.guild_id))
                    .await?;
            }
            Event::RoleUpdate(role) => {
                self.upsert_role(CachedRole::from_role(role.role.clone(), role.guild_id))
                    .await?;
            }
            Event::RoleDelete(role) => {
                self.delete_role(role.role_id).await?;
            }
            Event::StageInstanceCreate(stage) => {
                self.upsert_stage_instance(stage.clone().0).await?;
            }
            Event::StageInstanceUpdate(stage) => {
                self.upsert_stage_instance(stage.clone().0).await?;
            }
            Event::StageInstanceDelete(stage) => {
                self.delete_stage_instance(stage.id).await?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Get permissions of the current user in the given channel
    ///
    /// This is a convenience method for [`Self::channel_permissions`] with the
    /// current user's ID
    ///
    /// # Errors
    ///
    /// Returns the error the backend might return
    ///
    /// Returns [`Error::PermissionsChannelMissing`],
    /// [`Error::PermissionsChannelNotInGuild`],
    /// [`Error::PermissionsGuildMissing`] or
    /// [`Error::PermissionsGuildEveryoneRoleMissing`]
    async fn self_channel_permissions(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Result<Permissions, Error<Self::Error>> {
        let current_user_id = self.current_user().await?.id;
        self.channel_permissions(current_user_id, channel_id).await
    }

    /// Get permissions of the current user in the given guild
    ///
    /// This is a convenience method for [`Self::guild_permissions`] with the
    /// current user's ID
    ///
    /// # Errors
    ///
    /// Returns the error the backend might return
    ///
    /// Returns [`Error::PermissionsGuildMissing`] or
    /// [`Error::PermissionsGuildEveryoneRoleMissing`]
    async fn self_guild_permissions(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Permissions, Error<Self::Error>> {
        let current_user_id = self.current_user().await?.id;
        self.guild_permissions(current_user_id, guild_id).await
    }

    /// Get the permissions of the given user and channel
    ///
    /// # Errors
    ///
    /// Returns the error the backend might return
    ///
    /// Returns [`Error::PermissionsChannelMissing`],
    /// [`Error::PermissionsChannelNotInGuild`],
    /// [`Error::PermissionsGuildMissing`],
    /// [`Error::PermissionsGuildEveryoneRoleMissing`],
    /// [`Error::PermissionsMemberMissing`] or
    /// [`Error::MemberBadTimeoutTimestamp`]
    async fn channel_permissions(
        &self,
        user_id: Id<UserMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> Result<Permissions, Error<Self::Error>> {
        let channel = self
            .channel(channel_id)
            .await?
            .ok_or(Error::PermissionsChannelMissing(channel_id))?;
        let guild_id = channel
            .guild_id
            .ok_or_else(|| Error::PermissionsChannelNotInGuild(Box::new(channel.clone())))?;
        self.permissions(user_id, guild_id, Some(channel)).await
    }

    /// Get the permissions of the given user and guild
    ///
    /// # Errors
    ///
    /// Returns the error the backend might return
    ///
    /// Returns [`Error::PermissionsGuildMissing`],
    /// [`Error::PermissionsGuildEveryoneRoleMissing`],
    /// [`Error::PermissionsMemberMissing`] or
    /// [`Error::MemberBadTimeoutTimestamp`]
    async fn guild_permissions(
        &self,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Result<Permissions, Error<Self::Error>> {
        self.permissions(user_id, guild_id, None).await
    }

    /// Get the permissions with the given parameters
    ///
    /// # Errors
    ///
    /// Returns the error the backend might return
    ///
    /// Returns [`Error::PermissionsGuildMissing`],
    /// [`Error::PermissionsGuildEveryoneRoleMissing`],
    /// [`Error::PermissionsMemberMissing`] or
    /// [`Error::MemberBadTimeoutTimestamp`]
    #[doc(hidden)]
    async fn permissions(
        &self,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
        cached_channel: Option<CachedChannel>,
    ) -> Result<Permissions, Error<Self::Error>> {
        let guild = self
            .guild(guild_id)
            .await?
            .ok_or(Error::PermissionsGuildMissing(guild_id))?;
        let everyone_role = self
            .role(guild_id.cast())
            .await?
            .ok_or(Error::PermissionsGuildEveryoneRoleMissing(guild_id))?;
        let roles: Vec<_> = self
            .member_roles(user_id, guild_id)
            .await?
            .iter()
            .map(|role| (role.id, role.permissions))
            .collect();

        let calculator =
            PermissionCalculator::new(guild_id, user_id, everyone_role.permissions, &roles)
                .owner_id(guild.owner_id);
        let permissions = if let Some(channel) = cached_channel {
            calculator.in_channel(
                channel.kind,
                &self
                    .permission_overwrites(channel.id)
                    .await?
                    .iter()
                    .map(|overwrite| PermissionOverwrite {
                        allow: overwrite.allow,
                        deny: overwrite.deny,
                        id: overwrite.id,
                        kind: overwrite.kind,
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            calculator.root()
        };

        let member = self
            .member(user_id, guild_id)
            .await?
            .ok_or(Error::PermissionsMemberMissing { user_id, guild_id })?;
        if !permissions.contains(Permissions::ADMINISTRATOR)
            && member
                .communication_disabled()
                .map_err(|_err| Error::MemberBadTimeoutTimestamp(Box::new(member)))?
        {
            Ok(permissions
                .intersection(Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY))
        } else {
            Ok(permissions)
        }
    }

    /// Get the current user information of the bot
    ///
    /// # Errors
    ///
    /// Returns [`Error::CurrentUserMissing`] when called before the ready event
    /// is received
    async fn current_user(&self) -> Result<CurrentUser, Error<Self::Error>>;

    /// Get a cached channel or thread by its ID
    ///
    /// The users that are joined in a thread aren't cached, as caching them is
    /// likely unnecessary, if you need them, please create an issue
    async fn channel(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Result<Option<CachedChannel>, Error<Self::Error>>;

    /// Get a cached permission overwrites of a channel by its ID
    async fn permission_overwrites(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Result<Vec<CachedPermissionOverwrite>, Error<Self::Error>>;

    /// Get a guild's channels and threads by its ID
    async fn guild_channels(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Vec<CachedChannel>, Error<Self::Error>>;

    /// Get a cached message by its ID
    ///
    /// The returned message doesn't contain embeds, attachments, reactions or
    /// stickers, since they're cached separately and the method doesn't query
    /// them for you to reduce overhead in case you don't need them
    async fn message(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Option<CachedMessage>, Error<Self::Error>>;

    /// Get cached embeds of a message by its ID
    async fn embeds(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<(CachedEmbed, Vec<CachedEmbedField>)>, Error<Self::Error>> {
        let mut embeds = vec![];
        let cached_embeds = self.select_message_embeds(message_id).await?;
        for embed in cached_embeds {
            let fields = self.select_embed_fields(embed.id).await?;
            embeds.push((embed, fields));
        }
        Ok(embeds)
    }

    /// Get cached attachments of a message by its ID
    async fn attachments(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedAttachment>, Error<Self::Error>>;

    /// Get cached reactions of a message by its ID
    async fn reactions(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedReaction>, Error<Self::Error>>;

    /// Get cached stickers of a message by its ID
    async fn stickers(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<Vec<CachedSticker>, Error<Self::Error>>;

    /// Get a channel's most recent `limit` messages by its ID
    ///
    /// A limit of 0 means to return all messages
    ///
    /// The messages are ordered from most recent to least recent
    async fn channel_messages(
        &self,
        channel_id: Id<ChannelMarker>,
        limit: u16,
    ) -> Result<Vec<CachedMessage>, Error<Self::Error>>;

    /// Get a cached member by its guild ID and user ID
    async fn member(
        &self,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Result<Option<CachedMember>, Error<Self::Error>>;

    /// Get cached roles of a member by their ID
    async fn member_roles(
        &self,
        user_id: Id<UserMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Result<Vec<CachedRole>, Error<Self::Error>>;

    /// Get cached presence of a member by their ID
    async fn presence(
        &self,
        user_id: Id<UserMarker>,
    ) -> Result<Option<CachedPresence>, Error<Self::Error>>;

    /// Get cached activities of a member by their ID
    async fn member_activities(
        &self,
        user_id: Id<UserMarker>,
    ) -> Result<Vec<CachedActivity>, Error<Self::Error>>;

    /// Get a guild's members by its ID
    async fn guild_members(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Vec<CachedMember>, Error<Self::Error>>;

    /// Get a cached guild by its ID
    async fn guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Option<CachedGuild>, Error<Self::Error>>;

    /// Get a cached role by its ID
    async fn role(&self, role_id: Id<RoleMarker>)
        -> Result<Option<CachedRole>, Error<Self::Error>>;

    /// Get a guild's roles by its ID
    async fn guild_roles(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Vec<CachedRole>, Error<Self::Error>>;

    /// Get a cached emoji by its ID
    async fn emoji(
        &self,
        emoji_id: Id<EmojiMarker>,
    ) -> Result<Option<CachedEmoji>, Error<Self::Error>>;

    /// Get a guild's emojis by its ID
    async fn guild_emojis(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Vec<CachedEmoji>, Error<Self::Error>>;

    /// Get a cached sticker by its ID
    async fn sticker(
        &self,
        sticker_id: Id<StickerMarker>,
    ) -> Result<Option<CachedSticker>, Error<Self::Error>>;

    /// Get a guild's stickers by its ID
    async fn guild_stickers(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Vec<CachedSticker>, Error<Self::Error>>;

    /// Get a cached stage instance by its ID
    async fn stage_instance(
        &self,
        stage_id: Id<StageMarker>,
    ) -> Result<Option<StageInstance>, Error<Self::Error>>;

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
        for overwrite in channel
            .permission_overwrites
            .as_ref()
            .unwrap_or(&Vec::new())
        {
            self.upsert_permission_overwrite(CachedPermissionOverwrite::from_permission_overwrite(
                overwrite, channel.id,
            ))
            .await?;
        }
        self.upsert_channel(CachedChannel::from(channel)).await?;

        Ok(())
    }

    /// Updates the cache with the member's roles
    #[doc(hidden)]
    async fn add_member_roles(
        &self,
        user_id: Id<UserMarker>,
        role_ids: Vec<Id<RoleMarker>>,
    ) -> Result<(), Error<Self::Error>> {
        for role_id in role_ids {
            let mut role = self
                .role(role_id)
                .await?
                .ok_or(Error::MemberRoleMissing { user_id, role_id })?;
            role.user_id = Some(user_id);
            self.upsert_role(role).await?;
        }

        Ok(())
    }

    /// Removes the message from the cache
    #[doc(hidden)]
    async fn remove_message(
        &self,
        message_id: Id<MessageMarker>,
    ) -> Result<(), Error<Self::Error>> {
        let embeds = self.embeds(message_id).await?;
        for (embed, _) in embeds {
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
