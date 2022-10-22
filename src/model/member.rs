use time::{error::ComponentRange, OffsetDateTime};
use twilight_model::{
    gateway::payload::incoming::MemberUpdate,
    guild::Member,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    user::{PremiumType, UserFlags},
    util::{ImageHash, Timestamp},
};

/// A cached member
///
/// It's the same as [`twilight_model::guild::member::Member`] except:
///
/// - `user` field is flattened and `avatar` field is renamed to `guild_avatar`
///   since member users are rarely duplicates, making this struct easier to
///   cache
///
/// - `roles` field is removed, as member roles are cached separately
///
/// - `email` and `verified` fields are removed, as they're only sent in some
///   HTTP endpoints
#[derive(Clone, Debug)]
#[cfg_attr(feature = "tests", derive(PartialEq, Eq))]
pub struct CachedMember {
    pub guild_avatar: Option<ImageHash>,
    pub communication_disabled_until: Option<Timestamp>,
    pub deaf: bool,
    pub guild_id: Id<GuildMarker>,
    pub joined_at: Timestamp,
    pub mute: bool,
    pub nick: Option<String>,
    pub pending: bool,
    pub premium_since: Option<Timestamp>,
    pub accent_color: Option<u32>,
    pub avatar: Option<ImageHash>,
    pub banner: Option<ImageHash>,
    pub bot: bool,
    pub discriminator: u16,
    pub flags: Option<UserFlags>,
    pub id: Id<UserMarker>,
    pub locale: Option<String>,
    pub mfa_enabled: Option<bool>,
    pub name: String,
    pub premium_type: Option<PremiumType>,
    pub public_flags: Option<UserFlags>,
    pub system: Option<bool>,
}

impl CachedMember {
    /// Return whether the user is timed out
    ///
    /// # Warnings
    ///
    /// Make sure the system time is correct
    ///
    /// # Errors
    ///
    /// Returns an error if the member's timestamp isn't valid (a Twilight or
    /// Discord error)
    pub fn communication_disabled(&self) -> Result<bool, ComponentRange> {
        if let Some(timestamp) = self.communication_disabled_until {
            Ok(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?
                > OffsetDateTime::now_utc())
        } else {
            Ok(false)
        }
    }

    /// Update the cached member with the partial member
    pub fn update(&mut self, member: &MemberUpdate) {
        self.guild_avatar = member.avatar;
        self.communication_disabled_until = member.communication_disabled_until;
        if let Some(deaf) = member.deaf {
            self.deaf = deaf;
        }
        if let Some(mute) = member.mute {
            self.mute = mute;
        }
        self.nick.clone_from(&member.nick);
        self.pending = member.pending;
        self.premium_since = member.premium_since;
        self.accent_color = member.user.accent_color;
        self.avatar = member.user.avatar;
        self.banner = member.user.banner;
        self.discriminator = member.user.discriminator;
        self.flags = member.user.flags;
        self.id = member.user.id;
        self.locale.clone_from(&member.user.locale);
        self.mfa_enabled = member.user.mfa_enabled;
        self.name.clone_from(&member.user.name);
        self.premium_type = member.user.premium_type;
        self.public_flags = member.user.public_flags;
        self.system = member.user.system;
    }
}

impl From<&Member> for CachedMember {
    fn from(member: &Member) -> Self {
        Self {
            guild_avatar: member.avatar,
            communication_disabled_until: member.communication_disabled_until,
            deaf: member.deaf,
            guild_id: member.guild_id,
            joined_at: member.joined_at,
            mute: member.mute,
            nick: member.nick.clone(),
            pending: member.pending,
            premium_since: member.premium_since,
            accent_color: member.user.accent_color,
            system: member.user.system,
            avatar: member.user.avatar,
            banner: member.user.banner,
            bot: member.user.bot,
            discriminator: member.user.discriminator,
            flags: member.user.flags,
            id: member.user.id,
            locale: member.user.locale.clone(),
            mfa_enabled: member.user.mfa_enabled,
            name: member.user.name.clone(),
            premium_type: member.user.premium_type,
            public_flags: member.user.public_flags,
        }
    }
}
