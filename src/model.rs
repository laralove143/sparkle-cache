#![allow(
    clippy::missing_docs_in_private_items,
    missing_docs,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools
)]

pub use channel::CachedChannel;
pub use emoji::CachedEmoji;
pub use guild::CachedGuild;
pub use member::CachedMember;
pub use message::{
    CachedAttachment, CachedEmbed, CachedEmbedField, CachedMessage, CachedMessageSticker,
};
pub use presence::{CachedActivity, CachedPresence};
pub use reaction::CachedReaction;
pub use role::CachedRole;
pub use sticker::CachedSticker;

/// Definition and implementations for [`CachedChannel`]
mod channel;
/// Definition and implementations for [`CachedEmoji`]
mod emoji;
/// Definition and implementations for [`CachedGuild`]
mod guild;
/// Definition and implementations for [`CachedMember`]
mod member;
/// Definition and implementations for [`CachedMessage`] and its fields
mod message;
/// Definition and implementations for [`CachedPresence`] and its fields
mod presence;
/// Definition and implementations for [`CachedReaction`]
mod reaction;
/// Definition and implementations for [`CachedRole`]
mod role;
/// Definition and implementations for [`CachedSticker`]
mod sticker;
