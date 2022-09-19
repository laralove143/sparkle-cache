use twilight_model::{
    channel::{
        permission_overwrite::PermissionOverwrite,
        thread::{AutoArchiveDuration, ThreadMetadata},
        Channel, ChannelType, VideoQualityMode,
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    util::ImageHash,
};

/// A cached channel, it is the same as [`twilight_model::channel::Channel`]
/// except:
///
/// - [`twilight_model::channel::Channel.recipients`] is changed to user IDs,
///   which are cached separately
///
/// - [`twilight_model::channel::Channel.last_message_id`],
///   [`twilight_model::channel::Channel.last_pin_timestamp`],
///   [`twilight_model::channel::Channel.member_count`] and
///   [`twilight_model::channel::Channel.message_count`] are removed, as keeping
///   them up-to-date would add unnecessary caching overhead
///
/// - [`twilight_model::channel::Channel.member`] and
///   [`twilight_model::channel::Channel.newly_created`] are removed, as they're
///   only sent in some HTTP endpoints
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
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub position: Option<i32>,
    pub rate_limit_per_user: Option<u16>,
    pub recipients: Option<Vec<Id<UserMarker>>>,
    pub rtc_region: Option<String>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub topic: Option<String>,
    pub user_limit: Option<u32>,
    pub video_quality_mode: Option<VideoQualityMode>,
}

impl From<&Channel> for CachedChannel {
    /// # Clones
    ///
    /// - [`Self.name`]
    /// - [`Self.permission_overwrites`]
    /// - [`Self.rtc_region`]
    /// - [`Self.thread_metadata`]
    /// - [`Self.topic`]
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
            permission_overwrites: channel.permission_overwrites.clone(),
            position: channel.position,
            rate_limit_per_user: channel.rate_limit_per_user,
            recipients: channel
                .recipients
                .as_ref()
                .map(|recipients| recipients.iter().map(|recipient| recipient.id).collect()),
            rtc_region: channel.rtc_region.clone(),
            thread_metadata: channel.thread_metadata.clone(),
            topic: channel.topic.clone(),
            user_limit: channel.user_limit,
            video_quality_mode: channel.video_quality_mode,
        }
    }
}
