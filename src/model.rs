#![allow(
    clippy::missing_docs_in_private_items,
    missing_docs,
    clippy::module_name_repetitions
)]

pub use channel::CachedChannel;
pub use guild::CachedGuild;

/// Definition and implementations for [`CachedChannel`]
mod channel;
/// Definition and implementations for [`CachedGuild`]
mod guild;
