use twilight_model::{
    channel::message::{
        sticker::{MessageSticker, StickerFormatType, StickerType},
        Sticker,
    },
    id::{
        marker::{GuildMarker, MessageMarker, StickerMarker, StickerPackMarker, UserMarker},
        Id,
    },
};

/// A cached sticker
///
/// It's the same as [`twilight_model::channel::message::sticker::Sticker`]
/// except:
///
/// - `message_id` field is added, making it possible to return a message's
///   stickers
///
/// - `user` field is changed to a user ID, since users are cached separately
///
/// - `available`, `kind` and `tags` fields are made optional, as they're not
///   present in message stickers
#[derive(Clone, Debug)]
#[cfg_attr(feature = "tests", derive(PartialEq, Eq))]
pub struct CachedSticker {
    pub message_id: Option<Id<MessageMarker>>,
    pub available: Option<bool>,
    pub description: Option<String>,
    pub format_type: StickerFormatType,
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<StickerMarker>,
    pub kind: Option<StickerType>,
    pub name: String,
    pub pack_id: Option<Id<StickerPackMarker>>,
    pub sort_value: Option<u64>,
    pub tags: Option<String>,
    pub user_id: Option<Id<UserMarker>>,
}

impl CachedSticker {
    /// Create a cached sticker from a given message sticker and message ID
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn from_message_sticker(
        message_sticker: MessageSticker,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            message_id: Some(message_id),
            available: None,
            description: None,
            format_type: message_sticker.format_type,
            guild_id: None,
            id: message_sticker.id,
            kind: None,
            name: message_sticker.name,
            pack_id: None,
            sort_value: None,
            tags: None,
            user_id: None,
        }
    }
}

impl From<&Sticker> for CachedSticker {
    fn from(sticker: &Sticker) -> Self {
        Self {
            message_id: None,
            available: Some(sticker.available),
            description: sticker.description.clone(),
            format_type: sticker.format_type,
            guild_id: sticker.guild_id,
            id: sticker.id,
            kind: Some(sticker.kind),
            name: sticker.name.clone(),
            pack_id: sticker.pack_id,
            sort_value: sticker.sort_value,
            tags: Some(sticker.tags.clone()),
            user_id: sticker.user.as_ref().map(|user| user.id),
        }
    }
}
