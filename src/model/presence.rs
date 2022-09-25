use twilight_model::{
    gateway::presence::{Activity, ActivityFlags, ActivityType, Presence, Status},
    id::{
        marker::{ApplicationMarker, GuildMarker, UserMarker},
        Id,
    },
};

/// A cached activity
///
/// It is the same as [`twilight_model::gateway::presence::Activity`] except:
///
/// - `user_id` field is added, making it possible to return a user's activities
///
/// - `buttons` field is removed, as caching it is likely unnecessary, if you
///   need this field, please create an issue
///
/// - `assets`, `emoji`, `party` and `party` fields are flattened, making this
///   struct easier to cache
///
/// - `secrets` field is removed, as it's not sent to bots
#[derive(Clone, Debug)]
pub struct CachedActivity {
    pub user_id: Id<UserMarker>,
    pub application_id: Option<Id<ApplicationMarker>>,
    pub asset_large_image: Option<String>,
    pub asset_large_text: Option<String>,
    pub asset_small_image: Option<String>,
    pub asset_small_text: Option<String>,
    pub created_at: Option<u64>,
    pub details: Option<String>,
    pub emoji_animated: Option<bool>,
    pub emoji_name: Option<String>,
    pub emoji_id: Option<String>,
    pub flags: Option<ActivityFlags>,
    pub id: Option<String>,
    pub instance: Option<bool>,
    pub kind: ActivityType,
    pub name: String,
    pub party_id: Option<String>,
    pub party_size: Option<[u64; 2]>,
    pub state: Option<String>,
    pub timestamp_end: Option<u64>,
    pub timestamp_start: Option<u64>,
    pub url: Option<String>,
}

impl CachedActivity {
    /// Create a cached activity from a given activity and user ID
    #[must_use]
    pub fn from_activity(activity: &Activity, user_id: Id<UserMarker>) -> Self {
        Self {
            user_id,
            application_id: activity.application_id,
            asset_large_image: activity
                .assets
                .as_ref()
                .and_then(|asset| asset.large_image.clone()),
            asset_large_text: activity
                .assets
                .as_ref()
                .and_then(|asset| asset.large_text.clone()),
            asset_small_image: activity
                .assets
                .as_ref()
                .and_then(|asset| asset.small_image.clone()),
            asset_small_text: activity
                .assets
                .as_ref()
                .and_then(|asset| asset.small_text.clone()),
            created_at: activity.created_at,
            details: activity.details.clone(),
            emoji_animated: activity.emoji.as_ref().and_then(|emoji| emoji.animated),
            emoji_name: activity.emoji.as_ref().map(|emoji| emoji.name.clone()),
            emoji_id: activity.emoji.as_ref().and_then(|emoji| emoji.id.clone()),
            flags: activity.flags,
            id: activity.id.clone(),
            instance: activity.instance,
            kind: activity.kind,
            name: activity.name.clone(),
            party_id: activity.party.as_ref().and_then(|party| party.id.clone()),
            party_size: activity.party.as_ref().and_then(|party| party.size),
            state: activity.state.clone(),
            timestamp_end: activity
                .timestamps
                .as_ref()
                .and_then(|timestamp| timestamp.end),
            timestamp_start: activity
                .timestamps
                .as_ref()
                .and_then(|timestamp| timestamp.start),
            url: activity.url.clone(),
        }
    }
}

/// A cached presence
///
/// It's the same as [`twilight_model::gateway::presence::Presence`] except:
///
/// - `user` field is changed to a user ID, since users are cached separately
///
/// - `client_status` field is removed, as caching it is likely unnecessary, if
///   you need this field, please create an issue
///
/// - `activities` field is removed, since they're cached separately
#[derive(Clone, Copy, Debug)]
pub struct CachedPresence {
    pub guild_id: Id<GuildMarker>,
    pub status: Status,
    pub user: Id<UserMarker>,
}

impl From<&Presence> for CachedPresence {
    fn from(presence: &Presence) -> Self {
        Self {
            guild_id: presence.guild_id,
            status: presence.status,
            user: presence.user.id(),
        }
    }
}
