use twilight_model::{
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, Guild, GuildFeature, MfaLevel,
        NSFWLevel, PartialGuild, Permissions, PremiumTier, SystemChannelFlags, VerificationLevel,
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    util::{ImageHash, Timestamp},
};

/// A cached guild
///
/// It's the same as [`twilight_model::guild::Guild`] except:
///
/// - `channels`, `threads`, `members`, `roles`, `emojis`, `stickers`,
///   `presences` and `stage_instances` fields are removed, as they're cached
///   separately
///
/// - `member_count` field is removed, as keeping it up-to-date would add
///   unnecessary caching overhead
///
/// - `approximate_member_count` and `approximate_presence_count` fields are
///   removed, as they're only sent in some HTTP endpoints
///
/// - `voice_states` field is removed, as voice-related caching is not handled
///   by this library
#[derive(Clone, Debug)]
pub struct CachedGuild {
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: u64,
    pub application_id: Option<Id<ApplicationMarker>>,
    pub banner: Option<ImageHash>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<ImageHash>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<GuildFeature>,
    pub icon: Option<ImageHash>,
    pub id: Id<GuildMarker>,
    pub joined_at: Option<Timestamp>,
    pub large: bool,
    pub max_members: Option<u64>,
    pub max_presences: Option<u64>,
    pub max_video_channel_users: Option<u64>,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub nsfw_level: NSFWLevel,
    pub owner_id: Id<UserMarker>,
    pub owner: Option<bool>,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_progress_bar_enabled: bool,
    pub premium_subscription_count: Option<u64>,
    pub premium_tier: PremiumTier,
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    pub splash: Option<ImageHash>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub unavailable: bool,
    pub vanity_url_code: Option<String>,
    pub verification_level: VerificationLevel,
    pub widget_channel_id: Option<Id<ChannelMarker>>,
    pub widget_enabled: Option<bool>,
}

impl CachedGuild {
    /// Update the cached guild with the partial guild
    pub fn update(&mut self, guild: &PartialGuild) {
        self.id = guild.id;
        self.afk_channel_id = guild.afk_channel_id;
        self.afk_timeout = guild.afk_timeout;
        self.application_id = guild.application_id;
        self.banner = guild.banner;
        self.default_message_notifications = guild.default_message_notifications;
        self.description.clone_from(&guild.description);
        self.discovery_splash = guild.discovery_splash;
        self.explicit_content_filter = guild.explicit_content_filter;
        self.features.clone_from(&guild.features);
        self.icon.clone_from(&guild.icon);
        self.max_members = guild.max_members;
        self.max_presences = guild.max_presences;
        self.mfa_level = guild.mfa_level;
        self.name.clone_from(&guild.name);
        self.nsfw_level = guild.nsfw_level;
        self.owner_id = guild.owner_id;
        self.owner = guild.owner;
        self.permissions = guild.permissions;
        self.preferred_locale.clone_from(&guild.preferred_locale);
        self.premium_progress_bar_enabled = guild.premium_progress_bar_enabled;
        self.premium_subscription_count = guild.premium_subscription_count;
        self.premium_tier = guild.premium_tier;
        self.rules_channel_id = guild.rules_channel_id;
        self.splash = guild.splash;
        self.system_channel_flags = guild.system_channel_flags;
        self.system_channel_id = guild.system_channel_id;
        self.verification_level = guild.verification_level;
        self.vanity_url_code.clone_from(&guild.vanity_url_code);
        self.widget_channel_id = guild.widget_channel_id;
        self.widget_enabled = guild.widget_enabled;
    }
}

impl From<&Guild> for CachedGuild {
    fn from(guild: &Guild) -> Self {
        Self {
            afk_channel_id: guild.afk_channel_id,
            afk_timeout: guild.afk_timeout,
            application_id: guild.application_id,
            banner: guild.banner,
            default_message_notifications: guild.default_message_notifications,
            description: guild.description.clone(),
            discovery_splash: guild.discovery_splash,
            explicit_content_filter: guild.explicit_content_filter,
            features: guild.features.clone(),
            icon: guild.icon,
            id: guild.id,
            joined_at: guild.joined_at,
            large: guild.large,
            max_members: guild.max_members,
            max_presences: guild.max_presences,
            max_video_channel_users: guild.max_video_channel_users,
            mfa_level: guild.mfa_level,
            name: guild.name.clone(),
            nsfw_level: guild.nsfw_level,
            owner_id: guild.owner_id,
            owner: guild.owner,
            permissions: guild.permissions,
            preferred_locale: guild.preferred_locale.clone(),
            premium_progress_bar_enabled: guild.premium_progress_bar_enabled,
            premium_subscription_count: guild.premium_subscription_count,
            premium_tier: guild.premium_tier,
            rules_channel_id: guild.rules_channel_id,
            splash: guild.splash,
            system_channel_flags: guild.system_channel_flags,
            system_channel_id: guild.system_channel_id,
            unavailable: guild.unavailable,
            vanity_url_code: guild.vanity_url_code.clone(),
            verification_level: guild.verification_level,
            widget_channel_id: guild.widget_channel_id,
            widget_enabled: guild.widget_enabled,
        }
    }
}
