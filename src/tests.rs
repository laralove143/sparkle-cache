#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

use core::fmt::Debug;
use std::time::Instant;

use futures::StreamExt;
use twilight_gateway::{shard::Events, Shard};
use twilight_http::{
    self,
    request::{
        channel::reaction::RequestReactionType,
        guild::create_guild::{
            CategoryFields, GuildChannelFields, RoleFields, TextFields, VoiceFields,
        },
    },
    Client,
};
use twilight_model::{
    channel::{
        embed::{Embed, EmbedField},
        Channel, ChannelType, ReactionType,
    },
    gateway::Intents,
    guild::{
        DefaultMessageNotificationLevel, Emoji, ExplicitContentFilter, Permissions, Role,
        SystemChannelFlags,
    },
    http::{
        attachment::Attachment,
        permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
    },
    id::{marker::GuildMarker, Id},
    util::Timestamp,
};

use crate::{
    model::{
        CachedAttachment, CachedChannel, CachedEmbed, CachedEmbedField, CachedEmoji, CachedGuild,
        CachedMember, CachedMessage, CachedPermissionOverwrite, CachedReaction, CachedRole,
    },
    Cache,
};

/// The dummy name used for testing
const NAME: &str = "\u{2728} Cache Testing";
/// The dummy image hash used for testing
// noinspection SpellCheckingInspection
#[rustfmt::skip]
const IMAGE_HASH: &str = "data:image/png;base64,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";

/// Struct that runs the tests
#[derive(Debug)]
pub struct Tester<T: Cache + Send + Sync> {
    /// The cache to test
    cache: T,
    /// The events to update the cache with
    events: Events,
    /// The HTTP to create models to run tests against
    http: Client,
    /// The ID of the guild to run tests against
    test_guild_id: Id<GuildMarker>,
}

impl<T: Cache + Send + Sync> Tester<T> {
    /// Deletes the testing guild if it exists and creates a new one to return
    /// the tester
    ///
    /// # Warnings
    /// - The tests here are not cheap, they will make many requests to ensure
    ///   the cache is handling events correctly
    /// - Make sure the testing bot has all privileged intents
    /// - Make sure the testing bot is in less than 10 guilds
    /// - Make sure not to edit the testing guild in any way, including sending
    ///   messages or adding members in it
    #[allow(rust_2021_incompatible_closure_captures, clippy::too_many_lines)]
    pub async fn new(cache: T, token: &str) -> Result<Self, anyhow::Error> {
        let (shard, events) = Shard::new(token.to_owned(), Intents::all());
        shard.start().await?;

        let http = Client::new(token.to_owned());

        if let Some(guild) = http
            .current_user_guilds()
            .exec()
            .await?
            .models()
            .await?
            .iter()
            .find(|guild| guild.name == NAME)
        {
            http.delete_guild(guild.id).exec().await?;
        };

        let everyone_role = RoleFields {
            color: None,
            hoist: None,
            id: Id::new(10),
            mentionable: None,
            name: "@everyone".to_owned(),
            permissions: None,
            position: None,
        };
        let role = RoleFields {
            color: Some(1),
            hoist: Some(true),
            id: Id::new(1),
            mentionable: Some(true),
            name: "first".to_owned(),
            permissions: Some(Permissions::all()),
            position: Some(1),
        };

        let permission_overwrites = vec![PermissionOverwrite {
            allow: Some(Permissions::READ_MESSAGE_HISTORY),
            deny: Some(Permissions::ADMINISTRATOR),
            id: role.id.cast(),
            kind: PermissionOverwriteType::Role,
        }];
        let category = CategoryFields {
            id: Id::new(1),
            kind: ChannelType::GuildCategory,
            name: "category".to_owned(),
            permission_overwrites: Some(permission_overwrites.clone()),
        };
        let text_channel = TextFields {
            id: Id::new(2),
            kind: ChannelType::GuildText,
            name: "first_text".to_owned(),
            nsfw: Some(true),
            permission_overwrites: Some(permission_overwrites.clone()),
            parent_id: Some(category.id),
            rate_limit_per_user: Some(1),
            topic: Some("first text".to_owned()),
        };
        let voice_channel = VoiceFields {
            bitrate: Some(8000),
            id: Id::new(3),
            kind: ChannelType::GuildVoice,
            name: "first_voice".to_owned(),
            permission_overwrites: Some(permission_overwrites),
            parent_id: Some(category.id),
            user_limit: None,
        };

        let guild = http
            .create_guild(NAME.to_owned())?
            .default_message_notifications(DefaultMessageNotificationLevel::All)
            .explicit_content_filter(ExplicitContentFilter::AllMembers)
            .icon(IMAGE_HASH)
            .add_role(everyone_role)
            .add_role(role)
            .afk_channel_id(voice_channel.id)
            .afk_timeout(60)
            .system_channel_id(text_channel.id)
            .system_channel_flags(SystemChannelFlags::SUPPRESS_PREMIUM_SUBSCRIPTIONS)
            .channels(vec![
                GuildChannelFields::Category(category),
                GuildChannelFields::Text(text_channel),
                GuildChannelFields::Voice(voice_channel),
            ])?
            .exec()
            .await?
            .model()
            .await?;

        http.create_emoji(guild.id, "testing_emoji", IMAGE_HASH)
            .exec()
            .await?
            .model()
            .await?;

        // http.create_guild_sticker(
        //     guild.id,
        //     "testing sticker",
        //     "testing sticker description",
        //     "testing,sticker,tags",
        //     IMAGE_HASH
        //         .trim_start_matches("data:image/png;base64,")
        //         .as_bytes(),
        // )?
        // .exec()
        // .await?
        // .model()
        // .await?;

        let mut tester = Self {
            cache,
            http,
            events,
            test_guild_id: guild.id,
        };

        tester.update().await?;

        Ok(tester)
    }

    /// Does tests related to caching the current user
    pub async fn current_user(&mut self) -> Result<(), anyhow::Error> {
        self.assert_current_users_eq().await?;

        let current_user = self.cache.current_user().await?;
        let name = if current_user.name == NAME {
            format!("{NAME} New")
        } else {
            NAME.to_owned()
        };
        self.http
            .update_current_user()
            .avatar(if current_user.avatar.is_some() {
                None
            } else {
                Some(IMAGE_HASH)
            })
            .username(&name)?
            .exec()
            .await?
            .model()
            .await?;
        self.assert_current_users_eq().await?;

        Ok(())
    }

    /// Does tests related to caching channels
    pub async fn channels(&mut self) -> Result<(), anyhow::Error> {
        self.assert_channels_eq().await?;

        let first_channel_id = self.testing_guild_channels().await?.first().unwrap().id;

        self.http
            .update_channel(first_channel_id)
            .name("first_text_new")?
            .exec()
            .await?;
        self.assert_channels_eq().await?;

        let new_channel = self
            .http
            .create_guild_channel(self.test_guild_id, "second_text")?
            .exec()
            .await?
            .model()
            .await?;
        self.assert_channels_eq().await?;

        self.http.delete_channel(new_channel.id).exec().await?;
        self.assert_channels_eq().await?;

        Ok(())
    }

    /// Does tests related to caching permission overwrites
    pub async fn permission_overwrites(&mut self) -> Result<(), anyhow::Error> {
        self.assert_permission_overwrites_eq().await?;

        let first_channel_id = self.testing_guild_channels().await?.first().unwrap().id;
        let first_role_id = self.testing_guild_roles().await?.first().unwrap().id;

        self.http
            .update_channel_permission(
                first_channel_id,
                &PermissionOverwrite {
                    allow: Some(Permissions::ADD_REACTIONS),
                    deny: None,
                    id: first_role_id.cast(),
                    kind: PermissionOverwriteType::Role,
                },
            )
            .exec()
            .await?;
        self.assert_permission_overwrites_eq().await?;

        self.http
            .delete_channel_permission(first_channel_id)
            .role(first_role_id.cast())
            .exec()
            .await?;
        self.assert_channels_eq().await?;

        Ok(())
    }

    /// Does tests related to caching messages
    #[allow(clippy::too_many_lines)]
    pub async fn messages(&mut self) -> Result<(), anyhow::Error> {
        self.assert_messages_eq().await?;

        let first_channel_id = self.testing_guild_channels().await?.first().unwrap().id;

        let new_message = self
            .http
            .create_message(first_channel_id)
            .content("testing message")?
            .embeds(&[
                Embed {
                    description: Some("first testing embed".to_owned()),
                    fields: vec![
                        EmbedField {
                            inline: true,
                            name: "first testing embed first field".to_owned(),
                            value: "first testing embed first field value".to_owned(),
                        },
                        EmbedField {
                            inline: false,
                            name: "first testing embed second field".to_owned(),
                            value: "first testing embed second field value".to_owned(),
                        },
                    ],
                    author: None,
                    color: None,
                    footer: None,
                    image: None,
                    kind: String::new(),
                    provider: None,
                    thumbnail: None,
                    timestamp: None,
                    title: None,
                    url: None,
                    video: None,
                },
                Embed {
                    description: Some("second testing embed".to_owned()),
                    fields: vec![],
                    author: None,
                    color: None,
                    footer: None,
                    image: None,
                    kind: String::new(),
                    provider: None,
                    thumbnail: None,
                    timestamp: None,
                    title: None,
                    url: None,
                    video: None,
                },
            ])?
            .attachments(&[
                Attachment {
                    description: Some("first testing attachment".to_owned()),
                    file: IMAGE_HASH.as_bytes().to_owned(),
                    filename: "testing_attachment_first.png".to_owned(),
                    id: 0,
                },
                Attachment {
                    description: Some("second testing attachment".to_owned()),
                    file: IMAGE_HASH.as_bytes().to_owned(),
                    filename: "testing_attachment_second.png".to_owned(),
                    id: 1,
                },
            ])?
            // .sticker_ids(&[self.testing_guild_stickers().await?.first().unwrap().id])?
            .exec()
            .await?
            .model()
            .await?;
        self.assert_messages_eq().await?;

        self.http
            .update_message(first_channel_id, new_message.id)
            .content(None)?
            .exec()
            .await?;
        self.assert_messages_eq().await?;

        let first_emoji = self.testing_guild_emojis().await?.remove(0);
        self.http
            .create_reaction(
                first_channel_id,
                new_message.id,
                &RequestReactionType::Custom {
                    id: first_emoji.id,
                    name: Some(&first_emoji.name),
                },
            )
            .exec()
            .await?;
        self.assert_messages_eq().await?;

        self.http
            .delete_all_reactions(first_channel_id, new_message.id)
            .exec()
            .await?;
        self.assert_messages_eq().await?;

        self.http
            .delete_message(first_channel_id, new_message.id)
            .exec()
            .await?;
        self.assert_messages_eq().await?;

        Ok(())
    }

    /// Does tests related to caching members
    pub async fn members(&mut self) -> Result<(), anyhow::Error> {
        self.assert_members_eq().await?;

        let first_role_id = self.testing_guild_roles().await?.first().unwrap().id;
        let current_user_id = self.cache.current_user().await?.id;

        self.http
            .add_guild_member_role(self.test_guild_id, current_user_id, first_role_id)
            .exec()
            .await?;
        self.assert_members_eq().await?;

        self.http
            .remove_guild_member_role(self.test_guild_id, current_user_id, first_role_id)
            .exec()
            .await?;
        self.assert_members_eq().await?;

        Ok(())
    }

    /// Does tests related to the fields of [`crate::model::CachedGuild`]
    pub async fn guilds(&mut self) -> Result<(), anyhow::Error> {
        self.assert_guilds_eq().await?;

        self.http
            .update_guild(self.test_guild_id)
            .name(&format!("{NAME} New"))?
            .default_message_notifications(None)
            .explicit_content_filter(None)
            .icon(None)
            .afk_channel_id(None)
            .system_channel(None)
            .exec()
            .await?;
        self.assert_guilds_eq().await?;

        Ok(())
    }

    /// Does tests related to caching roles
    pub async fn roles(&mut self) -> Result<(), anyhow::Error> {
        self.assert_roles_eq().await?;

        let first_role_id = self.testing_guild_roles().await?.first().unwrap().id;

        self.http
            .update_role(self.test_guild_id, first_role_id)
            .name(Some("first new"))
            .exec()
            .await?;
        self.assert_roles_eq().await?;

        let new_role = self
            .http
            .create_role(self.test_guild_id)
            .name("second")
            .exec()
            .await?
            .model()
            .await?;
        self.assert_roles_eq().await?;

        self.http
            .delete_role(self.test_guild_id, new_role.id)
            .exec()
            .await?;
        self.assert_roles_eq().await?;

        Ok(())
    }

    /// Does tests related to caching emojis
    pub async fn emojis(&mut self) -> Result<(), anyhow::Error> {
        self.assert_emojis_eq().await?;

        let first_emoji_id = self.testing_guild_emojis().await?.first().unwrap().id;

        self.http
            .update_emoji(self.test_guild_id, first_emoji_id)
            .name("testing_emoji_new")
            .exec()
            .await?;
        self.assert_emojis_eq().await?;

        self.http
            .delete_emoji(self.test_guild_id, first_emoji_id)
            .exec()
            .await?;
        self.assert_emojis_eq().await?;

        Ok(())
    }

    /// Updates the cache with the pending events for 1 second
    async fn update(&mut self) -> Result<(), anyhow::Error> {
        let started = Instant::now();

        while let Some(event) = self.events.next().await {
            self.cache.update(&event).await?;

            if started.elapsed().as_secs() > 1 {
                return Ok(());
            }
        }

        Ok(())
    }

    // /// Does tests related to caching stickers
    // pub async fn stickers(&self) -> Result<(), anyhow::Error> {
    //     self.assert_stickers_eq().await?;
    //
    //     let first_sticker_id =
    // self.testing_guild_stickers().await?.first().unwrap().id;
    //
    //     self.http
    //         .update_guild_sticker(self.test_guild_id, first_sticker_id)
    //         .name("testing_sticker_new")?
    //         .exec()
    //         .await?;
    //     self.assert_stickers_eq().await?;
    //
    //     self.http
    //         .delete_guild_sticker(self.test_guild_id, first_sticker_id)
    //         .exec()
    //         .await?;
    //     self.assert_stickers_eq().await?;
    //
    //     Ok(())
    // }

    /// Asserts that the cached current user and the current user are equal
    async fn assert_current_users_eq(&mut self) -> Result<(), anyhow::Error> {
        self.update().await?;

        let mut current_user = self.http.current_user().exec().await?.model().await?;
        let mut cached_current_user = self.cache.current_user().await?;
        current_user.locale = None;
        cached_current_user.locale = None;
        current_user.premium_type = None;
        cached_current_user.premium_type = None;
        current_user.public_flags = None;
        cached_current_user.public_flags = None;
        assert_eq!(cached_current_user, current_user);

        Ok(())
    }

    /// Asserts that the cached channels and the channels in the testing guild
    /// are equal
    async fn assert_channels_eq(&mut self) -> Result<(), anyhow::Error> {
        self.update().await?;

        let channels: Vec<_> = self
            .testing_guild_channels()
            .await?
            .iter_mut()
            .map(|channel| {
                if channel.nsfw == Some(false) {
                    channel.nsfw = None;
                }
                CachedChannel::from(&*channel)
            })
            .collect();
        let mut cached_channels = self
            .cache
            .guild_channels(self.test_guild_id)
            .await?
            .into_iter()
            .map(|mut channel| {
                if channel.nsfw == Some(false) {
                    channel.nsfw = None;
                }
                channel
            })
            .collect();
        assert_vecs_eq(&channels, &cached_channels);

        cached_channels = vec![];
        for channel in &channels {
            let mut cached_channel = self.cache.channel(channel.id).await?.unwrap();
            if cached_channel.nsfw == Some(false) {
                cached_channel.nsfw = None;
            }
            cached_channels.push(cached_channel);
        }
        assert_vecs_eq(&channels, &cached_channels);

        Ok(())
    }

    /// Asserts that the cached channels and the channels in the testing guild
    /// are equal
    async fn assert_permission_overwrites_eq(&mut self) -> Result<(), anyhow::Error> {
        self.update().await?;

        let first_channel = self.testing_guild_channels().await?.remove(0);

        let permission_overwrites: Vec<_> = first_channel
            .permission_overwrites
            .unwrap_or_default()
            .into_iter()
            .map(|overwrite| {
                CachedPermissionOverwrite::from_permission_overwrite(&overwrite, first_channel.id)
            })
            .collect();

        let cached_permission_overwrites =
            self.cache.permission_overwrites(first_channel.id).await?;

        assert_vecs_eq(&permission_overwrites, &cached_permission_overwrites);

        Ok(())
    }

    /// Asserts that the cached messages and the messages in the testing guild
    /// are equal
    #[allow(clippy::too_many_lines)]
    async fn assert_messages_eq(&mut self) -> Result<(), anyhow::Error> {
        self.update().await?;

        let first_channel_id = self.testing_guild_channels().await?.first().unwrap().id;
        let messages: Vec<_> = self
            .http
            .channel_messages(first_channel_id)
            .exec()
            .await?
            .models()
            .await?
            .into_iter()
            .map(|mut message| {
                message.guild_id = Some(self.test_guild_id);
                message.timestamp = Timestamp::from_secs(message.timestamp.as_secs()).unwrap();
                message.edited_timestamp = message
                    .edited_timestamp
                    .map(|ts| Timestamp::from_secs(ts.as_secs()).unwrap());
                message
            })
            .collect();
        let mut cached_messages = self
            .cache
            .channel_messages(first_channel_id, 0)
            .await?
            .into_iter()
            .map(|mut message| {
                message.timestamp = Timestamp::from_secs(message.timestamp.as_secs()).unwrap();
                message.edited_timestamp = message
                    .edited_timestamp
                    .map(|ts| Timestamp::from_secs(ts.as_secs()).unwrap());
                message
            })
            .collect();
        assert_eq!(
            messages.iter().map(CachedMessage::from).collect::<Vec<_>>(),
            cached_messages
        );

        cached_messages = vec![];
        for message in &messages {
            let mut cached_message = self.cache.message(message.id).await?.unwrap();
            cached_message.timestamp =
                Timestamp::from_secs(cached_message.timestamp.as_secs()).unwrap();
            cached_message.edited_timestamp = cached_message
                .edited_timestamp
                .map(|ts| Timestamp::from_secs(ts.as_secs()).unwrap());
            cached_messages.push(cached_message);

            let cached_embeds = self.cache.embeds(message.id).await?;
            let embeds: Vec<_> = message
                .embeds
                .iter()
                .zip(&cached_embeds)
                .map(|(embed, (cached_embed, _))| {
                    let mut embed_into = CachedEmbed::from_embed(embed.clone(), message.id);
                    embed_into.id = cached_embed.id;
                    (
                        embed_into,
                        embed
                            .fields
                            .iter()
                            .map(|field| {
                                CachedEmbedField::from_embed_field(field.clone(), cached_embed.id)
                            })
                            .collect(),
                    )
                })
                .collect();
            assert_eq!(embeds, cached_embeds);

            let cached_attachments = self.cache.attachments(message.id).await?;
            assert_eq!(
                message
                    .attachments
                    .iter()
                    .map(|attachment| CachedAttachment::from_attachment(
                        attachment.clone(),
                        message.id
                    ))
                    .collect::<Vec<_>>(),
                cached_attachments
            );

            let cached_reactions = self.cache.reactions(message.id).await?;
            let current_user_id = self.cache.current_user().await?.id;
            assert_vecs_eq(
                &message
                    .reactions
                    .iter()
                    .map(|reaction| CachedReaction {
                        channel_id: message.channel_id,
                        emoji: match &reaction.emoji {
                            ReactionType::Custom { id, .. } => id.to_string(),
                            ReactionType::Unicode { name } => name.clone(),
                        },
                        guild_id: message.guild_id,
                        message_id: message.id,
                        user_id: current_user_id,
                    })
                    .collect::<Vec<_>>(),
                &cached_reactions,
            );
        }

        assert_eq!(
            messages.iter().map(CachedMessage::from).collect::<Vec<_>>(),
            cached_messages
        );

        Ok(())
    }

    /// Asserts that the cached members and the members in the testing guild are
    /// equal
    async fn assert_members_eq(&mut self) -> Result<(), anyhow::Error> {
        self.update().await?;

        let members: Vec<_> = self
            .http
            .guild_members(self.test_guild_id)
            .exec()
            .await?
            .models()
            .await?
            .into_iter()
            .map(|mut member| {
                member.joined_at = Timestamp::from_secs(member.joined_at.as_secs()).unwrap();
                member
            })
            .collect();
        let roles = self.testing_guild_roles().await?;
        let mut cached_members = self
            .cache
            .guild_members(self.test_guild_id)
            .await?
            .into_iter()
            .map(|mut member| {
                member.joined_at = Timestamp::from_secs(member.joined_at.as_secs()).unwrap();
                member
            })
            .collect();

        assert_vecs_eq(
            &members.iter().map(CachedMember::from).collect::<Vec<_>>(),
            &cached_members,
        );

        cached_members = vec![];
        for member in &members {
            let mut cached_member = self
                .cache
                .member(member.user.id, self.test_guild_id)
                .await?
                .unwrap();
            cached_member.joined_at =
                Timestamp::from_secs(cached_member.joined_at.as_secs()).unwrap();
            cached_members.push(cached_member);

            let mut member_roles = vec![];
            for role_id in &member.roles {
                member_roles.push(
                    roles
                        .iter()
                        .find(|role| role.id == *role_id)
                        .unwrap()
                        .clone(),
                );
            }
            assert_vecs_eq(
                &member_roles
                    .into_iter()
                    .map(|role| {
                        let mut cached_role = CachedRole::from_role(role, self.test_guild_id);
                        cached_role.user_id = Some(member.user.id);
                        cached_role
                    })
                    .collect::<Vec<_>>(),
                &self
                    .cache
                    .member_roles(member.user.id, self.test_guild_id)
                    .await?
                    .into_iter()
                    .map(|mut role| {
                        if role.tags_premium_subscriber == Some(false) {
                            role.tags_premium_subscriber = None;
                        }
                        role
                    })
                    .collect::<Vec<_>>(),
            );
        }

        assert_eq!(
            members.iter().map(CachedMember::from).collect::<Vec<_>>(),
            cached_members
        );

        Ok(())
    }

    /// Asserts that the cached testing guild and the testing guild are equal
    async fn assert_guilds_eq(&mut self) -> Result<(), anyhow::Error> {
        self.update().await?;

        let guild = self
            .http
            .guild(self.test_guild_id)
            .exec()
            .await?
            .model()
            .await?;

        assert_eq!(
            self.cache.guild(self.test_guild_id).await?.unwrap(),
            CachedGuild::from(&guild),
        );

        Ok(())
    }

    /// Asserts that the cached roles and the roles in the testing guild are
    /// equal
    async fn assert_roles_eq(&mut self) -> Result<(), anyhow::Error> {
        self.update().await?;

        let roles: Vec<_> = self
            .testing_guild_roles()
            .await?
            .into_iter()
            .map(|role| CachedRole::from_role(role, self.test_guild_id))
            .collect();
        let mut cached_roles = self.cache.guild_roles(self.test_guild_id).await?;

        assert_vecs_eq(&roles, &cached_roles);

        cached_roles = vec![];
        for role in &roles {
            cached_roles.push(self.cache.role(role.id).await?.unwrap());
        }

        assert_eq!(roles, cached_roles);

        Ok(())
    }

    /// Asserts that the cached emojis and the emojis in the testing guild are
    /// equal
    async fn assert_emojis_eq(&mut self) -> Result<(), anyhow::Error> {
        self.update().await?;

        let emojis = self.testing_guild_emojis().await?;
        let mut cached_emojis = self.cache.guild_emojis(self.test_guild_id).await?;

        assert_vecs_eq(
            &emojis
                .iter()
                .map(|emoji| CachedEmoji::from_emoji(emoji, self.test_guild_id))
                .collect::<Vec<_>>(),
            &cached_emojis,
        );

        cached_emojis = vec![];
        for emoji in &emojis {
            cached_emojis.push(self.cache.emoji(emoji.id).await?.unwrap());
        }

        assert_eq!(
            emojis
                .iter()
                .map(|emoji| CachedEmoji::from_emoji(emoji, self.test_guild_id))
                .collect::<Vec<_>>(),
            cached_emojis
        );

        Ok(())
    }

    // /// Asserts that the cached stickers and the stickers in the testing guild
    // /// are equal
    // async fn assert_stickers_eq(&mut self) -> Result<(), anyhow::Error> {
    //     self.update().await?;
    //
    //     let stickers = self.testing_guild_stickers().await?;
    //     let mut cached_stickers =
    // self.cache.guild_stickers(self.test_guild_id).await?;     assert_eq!(
    //         stickers.iter().map(CachedSticker::from).collect::<Vec<_>>(),
    //         cached_stickers
    //     );
    //
    //     cached_stickers = vec![];
    //     for sticker in &stickers {
    //         cached_stickers.push(self.cache.sticker(sticker.id).await?.unwrap());
    //     }
    //
    //     assert_eq!(
    //         stickers.iter().map(CachedSticker::from).collect::<Vec<_>>(),
    //         cached_stickers
    //     );
    //
    //     Ok(())
    // }

    /// Returns the channels in the testing guild
    async fn testing_guild_channels(&self) -> Result<Vec<Channel>, anyhow::Error> {
        let mut channels = self
            .http
            .guild_channels(self.test_guild_id)
            .exec()
            .await?
            .models()
            .await?;
        channels.sort_unstable_by(|old, new| new.position.unwrap().cmp(&old.position.unwrap()));

        Ok(channels)
    }

    /// Returns the roles in the testing guild
    async fn testing_guild_roles(&self) -> Result<Vec<Role>, anyhow::Error> {
        let mut roles = self
            .http
            .roles(self.test_guild_id)
            .exec()
            .await?
            .models()
            .await?;
        roles.retain(|role| role.id.cast() != self.test_guild_id);

        Ok(roles)
    }

    /// Returns the emojis in the testing guild
    async fn testing_guild_emojis(&self) -> Result<Vec<Emoji>, anyhow::Error> {
        Ok(self
            .http
            .emojis(self.test_guild_id)
            .exec()
            .await?
            .models()
            .await?)
    }

    // /// Returns the stickers in the testing guild
    // async fn testing_guild_stickers(&self) -> Result<Vec<Sticker>, anyhow::Error>
    // {     Ok(self
    //         .http
    //         .guild_stickers(self.test_guild_id)
    //         .exec()
    //         .await?
    //         .models()
    //         .await?)
    // }
}

/// Asserts that the vectors are equal ignoring the order
fn assert_vecs_eq<T: PartialEq + Debug>(vec_a: &Vec<T>, vec_b: &Vec<T>) {
    assert_eq!(vec_a.len(), vec_b.len());
    for a in vec_a {
        assert!(vec_b.contains(a), "{a:#?} is not in {vec_b:#?}");
    }
}
