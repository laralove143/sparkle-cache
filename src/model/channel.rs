use twilight_model::{
    channel::{
        permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
        thread::{AutoArchiveDuration, ThreadMetadata},
        Channel, ChannelType, VideoQualityMode,
    },
    guild::Permissions,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GenericMarker, GuildMarker, UserMarker},
        Id,
    },
    util::ImageHash,
};

/// A cached permission overwrite
///
/// It's the same as
/// [`twilight_model::channel::permission_overwrite::PermissionOverwrite`]
/// except:
///
/// - `channel_id` field is added, making it possible to return a channel's
///   permission overwrites
#[derive(Clone, Copy, Debug)]
pub struct CachedPermissionOverwrite {
    pub channel_id: Id<ChannelMarker>,
    pub allow: Permissions,
    pub deny: Permissions,
    pub id: Id<GenericMarker>,
    pub kind: PermissionOverwriteType,
}

impl CachedPermissionOverwrite {
    /// Create a cached permission overwrite from a given permission overwrite
    /// and channel ID
    #[must_use]
    pub const fn from_permission_overwrite(
        permission_overwrite: &PermissionOverwrite,
        channel_id: Id<ChannelMarker>,
    ) -> Self {
        Self {
            channel_id,
            allow: permission_overwrite.allow,
            deny: permission_overwrite.deny,
            id: permission_overwrite.id,
            kind: permission_overwrite.kind,
        }
    }
}

/// A cached channel
///
/// It's the same as [`twilight_model::channel::Channel`] except:
///
/// - `recipients` field is removed, as it's only sent in DM channels, which are
///   cached separately
///
/// - `permission_overwrites` field is removed, as they're cached separately
///
/// - `last_message_id`, `last_pin_timestamp`, `member_count` and
///   `message_count` fields are removed, as keeping them up-to-date would add
///   unnecessary caching overhead
///
/// - `member` and `newly_created` fields are removed, as they're only sent in
///   some HTTP endpoints
#[derive(Clone, Debug)]
pub struct CachedChannel {
    pub application_id: Option<Id<ApplicationMarker>>,
    pub bitrate: Option<u32>,
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub icon: Option<ImageHash>,
    pub id: Id<ChannelMarker>,
    pub invitable: Option<bool>,
    pub kind: ChannelType,
    pub name: Option<String>,
    pub nsfw: Option<bool>,
    pub owner_id: Option<Id<UserMarker>>,
    pub parent_id: Option<Id<ChannelMarker>>,
    pub position: Option<i32>,
    pub rate_limit_per_user: Option<u16>,
    pub rtc_region: Option<String>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub topic: Option<String>,
    pub user_limit: Option<u32>,
    pub video_quality_mode: Option<VideoQualityMode>,
}

impl From<&Channel> for CachedChannel {
    fn from(channel: &Channel) -> Self {
        Self {
            application_id: channel.application_id,
            bitrate: channel.bitrate,
            default_auto_archive_duration: channel.default_auto_archive_duration,
            guild_id: channel.guild_id,
            icon: channel.icon,
            id: channel.id,
            invitable: channel.invitable,
            kind: channel.kind,
            name: channel.name.clone(),
            nsfw: channel.nsfw,
            owner_id: channel.owner_id,
            parent_id: channel.parent_id,
            position: channel.position,
            rate_limit_per_user: channel.rate_limit_per_user,
            rtc_region: channel.rtc_region.clone(),
            thread_metadata: channel.thread_metadata.clone(),
            topic: channel.topic.clone(),
            user_limit: channel.user_limit,
            video_quality_mode: channel.video_quality_mode,
        }
    }
}
