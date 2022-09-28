use twilight_model::{
    channel::{
        embed::{Embed, EmbedField},
        message::{MessageActivityType, MessageFlags, MessageType},
        Attachment, Message,
    },
    gateway::payload::incoming::MessageUpdate,
    id::{
        marker::{
            ApplicationMarker, AttachmentMarker, ChannelMarker, GenericMarker, GuildMarker,
            MessageMarker, UserMarker, WebhookMarker,
        },
        Id,
    },
    util::{ImageHash, Timestamp},
};

use crate::unique_id;

/// A cached embed field
///
/// It's the same as [`twilight_model::channel::embed::EmbedField`] except:
///
/// - `embed_id` field is added, making it possible to return an embed's fields
#[derive(Clone, Debug)]
pub struct CachedEmbedField {
    pub embed_id: Id<GenericMarker>,
    pub inline: bool,
    pub name: String,
    pub value: String,
}

impl CachedEmbedField {
    /// Create a cached embed field from a given embed field and embed ID
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn from_embed_field(embed_field: EmbedField, embed_id: Id<GenericMarker>) -> Self {
        Self {
            embed_id,
            inline: embed_field.inline,
            name: embed_field.name,
            value: embed_field.value,
        }
    }
}

/// A cached embed
///
/// It's the same as [`twilight_model::channel::embed::Embed`] except:
///
/// - `fields` field is removed and `id` field is added, making it possible to
///   return an embed's fields
///
/// - `message_id` field is added, making it possible to return a message's
///   embeds
///
/// - `author`, `footer`, `image`, `provider`, `thumbnail` and `video` fields
///   are flattened, making this struct easier to cache
#[derive(Clone, Debug)]
pub struct CachedEmbed {
    pub id: Id<GenericMarker>,
    pub message_id: Id<MessageMarker>,
    pub author_icon_url: Option<String>,
    pub author_name: Option<String>,
    pub author_proxy_icon_url: Option<String>,
    pub author_url: Option<String>,
    pub color: Option<u32>,
    pub description: Option<String>,
    pub footer_icon_url: Option<String>,
    pub footer_proxy_icon_url: Option<String>,
    pub footer_text: Option<String>,
    pub image_height: Option<u64>,
    pub image_proxy_url: Option<String>,
    pub image_url: Option<String>,
    pub image_width: Option<u64>,
    pub kind: String,
    pub provider_name: Option<String>,
    pub provider_url: Option<String>,
    pub thumbnail_height: Option<u64>,
    pub thumbnail_proxy_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_width: Option<u64>,
    pub timestamp: Option<Timestamp>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub video_height: Option<u64>,
    pub video_proxy_url: Option<String>,
    pub video_url: Option<String>,
    pub video_width: Option<u64>,
}

impl CachedEmbed {
    /// Create a cached embed from a given embed and message ID
    #[allow(clippy::cast_sign_loss, clippy::as_conversions)]
    #[must_use]
    pub fn from_embed(embed: Embed, message_id: Id<MessageMarker>) -> Self {
        Self {
            id: Id::new(unique_id() as u64),
            message_id,
            author_icon_url: embed
                .author
                .as_ref()
                .and_then(|author| author.icon_url.clone()),
            author_name: embed.author.as_ref().map(|author| author.name.clone()),
            author_proxy_icon_url: embed
                .author
                .as_ref()
                .and_then(|author| author.icon_url.clone()),
            author_url: embed.author.as_ref().and_then(|author| author.url.clone()),
            color: embed.color,
            description: embed.description,
            footer_icon_url: embed
                .footer
                .as_ref()
                .and_then(|footer| footer.icon_url.clone()),
            footer_proxy_icon_url: embed
                .footer
                .as_ref()
                .and_then(|footer| footer.proxy_icon_url.clone()),
            footer_text: embed.footer.as_ref().map(|footer| footer.text.clone()),
            image_height: embed.image.as_ref().and_then(|image| image.height),
            image_proxy_url: embed
                .image
                .as_ref()
                .and_then(|image| image.proxy_url.clone()),
            image_url: embed.image.as_ref().map(|image| image.url.clone()),
            image_width: embed.image.as_ref().and_then(|image| image.width),
            kind: embed.kind,
            provider_name: embed
                .provider
                .as_ref()
                .and_then(|provider| provider.name.clone()),
            provider_url: embed
                .provider
                .as_ref()
                .and_then(|provider| provider.url.clone()),
            thumbnail_height: embed
                .thumbnail
                .as_ref()
                .and_then(|thumbnail| thumbnail.height),
            thumbnail_proxy_url: embed
                .thumbnail
                .as_ref()
                .and_then(|thumbnail| thumbnail.proxy_url.clone()),
            thumbnail_url: embed
                .thumbnail
                .as_ref()
                .map(|thumbnail| thumbnail.url.clone()),
            thumbnail_width: embed
                .thumbnail
                .as_ref()
                .and_then(|thumbnail| thumbnail.width),
            timestamp: embed.timestamp,
            title: embed.title,
            url: embed.url,
            video_height: embed.video.as_ref().and_then(|video| video.height),
            video_proxy_url: embed
                .video
                .as_ref()
                .and_then(|video| video.proxy_url.clone()),
            video_url: embed.video.as_ref().and_then(|video| video.url.clone()),
            video_width: embed.video.as_ref().and_then(|video| video.width),
        }
    }
}

/// A cached attachment
///
/// It's the same as [`twilight_model::channel::Attachment`] except:
///
/// - `message_id` field is added, making it possible to return a message's
///   attachments
#[derive(Clone, Debug)]
pub struct CachedAttachment {
    pub message_id: Id<MessageMarker>,
    pub content_type: Option<String>,
    pub ephemeral: bool,
    pub filename: String,
    pub description: Option<String>,
    pub height: Option<u64>,
    pub id: Id<AttachmentMarker>,
    pub proxy_url: String,
    pub size: u64,
    pub url: String,
    pub width: Option<u64>,
}

impl CachedAttachment {
    /// Create a cached attachment from a given attachment and message ID
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn from_attachment(attachment: Attachment, message_id: Id<MessageMarker>) -> Self {
        Self {
            message_id,
            content_type: attachment.content_type,
            ephemeral: attachment.ephemeral,
            filename: attachment.filename,
            description: attachment.description,
            height: attachment.height,
            id: attachment.id,
            proxy_url: attachment.proxy_url,
            size: attachment.size,
            url: attachment.url,
            width: attachment.width,
        }
    }
}

/// A cached message
///
/// It's the same as [`twilight_model::channel::message::Message`] except:
///
/// - `activity` and `reference` fields are  flattened, making this struct
///   easier to cache
///
/// - `author`, `referenced_message` and `thread` fields are changed to their
///   IDs, since they're cached separately
///
/// - `components`, `interaction`, `mention_channels`, `mention_roles` and
///   `mentions` fields are removed, as caching them is likely unnecessary, if
///   you need these fields, please create an issue
///
/// - `member`, `reactions`, `attachments`, `embeds` and `sticker_items` fields
///   are removed, since they are cached separately
#[derive(Clone, Debug)]
pub struct CachedMessage {
    pub activity_type: Option<MessageActivityType>,
    pub activity_party_id: Option<String>,
    pub application_cover_image: Option<ImageHash>,
    pub application_description: Option<String>,
    pub application_icon: Option<ImageHash>,
    pub application_id: Option<Id<ApplicationMarker>>,
    pub application_name: Option<String>,
    pub interaction_application_id: Option<Id<ApplicationMarker>>,
    pub author: Id<UserMarker>,
    pub channel_id: Id<ChannelMarker>,
    pub content: String,
    pub edited_timestamp: Option<Timestamp>,
    pub flags: Option<MessageFlags>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<MessageMarker>,
    pub kind: MessageType,
    pub mention_everyone: bool,
    pub pinned: bool,
    pub reference_channel_id: Option<Id<ChannelMarker>>,
    pub reference_guild_id: Option<Id<GuildMarker>>,
    pub reference_message_id: Option<Id<MessageMarker>>,
    pub reference_fail_if_not_exists: Option<bool>,
    pub referenced_message: Option<Id<MessageMarker>>,
    pub timestamp: Timestamp,
    pub thread: Option<Id<ChannelMarker>>,
    pub tts: bool,
    pub webhook_id: Option<Id<WebhookMarker>>,
}

impl CachedMessage {
    /// Update the cached message with the message update
    pub fn update(&mut self, message: &MessageUpdate) {
        if let Some(content) = &message.content {
            self.content.clone_from(content);
        }
        if message.edited_timestamp.is_some() {
            self.edited_timestamp = message.edited_timestamp;
        }
        if let Some(mentions) = message.mention_everyone {
            self.mention_everyone = mentions;
        }
        if let Some(pinned) = message.pinned {
            self.pinned = pinned;
        }
    }
}

impl From<&Message> for CachedMessage {
    fn from(message: &Message) -> Self {
        Self {
            activity_type: message.activity.as_ref().map(|activity| activity.kind),
            activity_party_id: message
                .activity
                .as_ref()
                .and_then(|activity| activity.party_id.clone()),
            application_cover_image: message
                .application
                .as_ref()
                .and_then(|application| application.cover_image),
            application_description: message
                .application
                .as_ref()
                .map(|application| application.description.clone()),
            application_icon: message
                .application
                .as_ref()
                .and_then(|application| application.icon),
            application_id: message
                .application
                .as_ref()
                .map(|application| application.id),
            application_name: message
                .application
                .as_ref()
                .map(|application| application.name.clone()),
            interaction_application_id: message.application_id,
            author: message.author.id,
            channel_id: message.channel_id,
            content: message.content.clone(),
            edited_timestamp: message.edited_timestamp,
            guild_id: message.guild_id,
            id: message.id,
            kind: message.kind,
            mention_everyone: message.mention_everyone,
            pinned: message.pinned,
            reference_channel_id: message
                .reference
                .as_ref()
                .and_then(|reference| reference.channel_id),
            reference_guild_id: message
                .reference
                .as_ref()
                .and_then(|reference| reference.guild_id),
            reference_message_id: message
                .reference
                .as_ref()
                .and_then(|reference| reference.message_id),
            reference_fail_if_not_exists: message
                .reference
                .as_ref()
                .and_then(|reference| reference.fail_if_not_exists),
            referenced_message: message
                .referenced_message
                .as_ref()
                .map(|reference| reference.id),
            timestamp: message.timestamp,
            thread: message.thread.as_ref().map(|thread| thread.id),
            tts: message.tts,
            flags: message.flags,
            webhook_id: message.webhook_id,
        }
    }
}
