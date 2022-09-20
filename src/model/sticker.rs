use twilight_model::{
    channel::message::{
        sticker::{StickerFormatType, StickerType},
        Sticker,
    },
    id::{
        marker::{GuildMarker, StickerMarker, StickerPackMarker, UserMarker},
        Id,
    },
};

/// A cached sticker, it is the same as
/// [`twilight_model::channel::message::sticker::Sticker`] except:
///
/// - [`twilight_model::channel::message::sticker::Sticker.user`] is changed to
///   a user ID which is cached separately
#[derive(Clone, Debug)]
pub struct CachedSticker {
    pub available: bool,
    pub description: Option<String>,
    pub format_type: StickerFormatType,
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<StickerMarker>,
    pub kind: StickerType,
    pub name: String,
    pub pack_id: Option<Id<StickerPackMarker>>,
    pub sort_value: Option<u64>,
    pub tags: String,
    pub user_id: Option<Id<UserMarker>>,
}

impl From<&Sticker> for CachedSticker {
    /// # Clones
    ///
    /// - [`Self.name`]
    /// - [`Self.description`]
    /// - [`Self.tags`]
    fn from(sticker: &Sticker) -> Self {
        Self {
            available: sticker.available,
            description: sticker.description.clone(),
            format_type: sticker.format_type,
            guild_id: sticker.guild_id,
            id: sticker.id,
            kind: sticker.kind,
            name: sticker.name.clone(),
            pack_id: sticker.pack_id,
            sort_value: sticker.sort_value,
            tags: sticker.tags.clone(),
            user_id: sticker.user.as_ref().map(|user| user.id),
        }
    }
}
