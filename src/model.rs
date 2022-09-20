#![allow(
    clippy::missing_docs_in_private_items,
    missing_docs,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools
)]

pub use channel::CachedChannel;
pub use emoji::CachedEmoji;
pub use guild::CachedGuild;

/// Definition and implementations for [`CachedChannel`]
mod channel;
/// Definition and implementations for [`CachedEmoji`]
mod emoji;
/// Definition and implementations for [`CachedGuild`]
mod guild;
